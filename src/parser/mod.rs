use codecrafters_interpreter::{Conditional, Expr, Statement, Token, TokenType};
mod tests;

pub struct Parser {
    pub tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, ()> {
        let mut expressions: Vec<Expr> = vec![];
        loop {
            if self.curr_matches_type(TokenType::EOF) {
                break;
            }

            if self.curr_matches_type(TokenType::RIGHT_BRACE) {
                return Ok(expressions);
            }

            let expr = self.parse_expression()?;
            expressions.push(expr);
        }

        Ok(expressions)
    }

    pub fn parse_expression(&mut self) -> Result<Expr, ()> {
        let expr = self.parse_assignment()?;
        if matches!(
            expr,
            Expr::Scope(_)
                | Expr::Stmt(Statement::IfStmt(_))
                | Expr::Stmt(Statement::ForStmt(..))
                | Expr::Stmt(Statement::WhileStmt(_))
        ) {
            return Ok(expr);
        }

        if !self.curr_matches_type(TokenType::SEMICOLON) {
            self.print_token_err("Missing ';'")?;
        }

        self.current += 1;
        Ok(expr)
    }

    pub fn parse_assignment(&mut self) -> Result<Expr, ()> {
        // check if the start is an identifier if it followed by EQUAL token
        let mut expr = self.parse_or()?;
        while self.curr_matches_type(TokenType::EQUAL) {
            match expr {
                Expr::Literal(token) => {
                    if token.token_type != TokenType::IDENTIFIER {
                        self.print_token_err("Cannot assign to non-identifier")?;
                    }
                    self.current += 1;
                    let value = self.parse_assignment()?;
                    expr = Expr::Stmt(Statement::AssignmentStmt(token, Box::new(value)));
                }
                _ => {
                    self.print_token_err("Cannot assign to non-identifier")?;
                }
            }
        }

        Ok(expr)
    }

    fn parse_or(&mut self) -> Result<Expr, ()> {
        let mut expr = self.parse_and()?;
        while self.curr_matches_type(TokenType::OR) {
            let operator = self.tokens[self.current].clone();
            self.current += 1;
            let right = self.parse_and()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expr, ()> {
        let mut expr = self.parse_equality()?;
        while self.curr_matches_type(TokenType::AND) {
            let operator = self.tokens[self.current].clone();
            self.current += 1;
            let right = self.parse_equality()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, ()> {
        let mut expr = self.parse_comparison()?;
        while matches!(
            self.tokens[self.current].token_type,
            TokenType::BANG_EQUAL | TokenType::EQUAL_EQUAL
        ) {
            let operator = &self.tokens[self.current].clone();
            self.current += 1;
            let right = self.parse_comparison()?;
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ()> {
        let mut expr = self.parse_additive()?;
        while matches!(
            self.tokens[self.current].token_type,
            TokenType::GREATER | TokenType::GREATER_EQUAL | TokenType::LESS | TokenType::LESS_EQUAL
        ) {
            let operator = &self.tokens[self.current].clone();
            self.current += 1;
            let right = self.parse_additive()?;
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }

        Ok(expr)
    }

    fn parse_additive(&mut self) -> Result<Expr, ()> {
        let mut expr = self.parse_multiplicative()?;
        while matches!(
            self.tokens[self.current].token_type,
            TokenType::PLUS | TokenType::MINUS
        ) {
            let operator = &self.tokens[self.current].clone();
            self.current += 1;
            let right = self.parse_multiplicative()?;
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }

        Ok(expr)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, ()> {
        let mut expr = self.parse_unary()?;
        while matches!(
            self.tokens[self.current].token_type,
            TokenType::STAR | TokenType::SLASH
        ) {
            let operator = &self.tokens[self.current].clone();
            self.current += 1;
            let right = self.parse_unary()?;
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, ()> {
        let token = &self.tokens[self.current].clone();
        match token.token_type {
            TokenType::BANG | TokenType::MINUS => {
                let operator = token;
                self.current += 1;
                let right = self.parse_unary()?;
                return Ok(Expr::Unary(operator.clone(), Box::new(right)));
            }
            _ => {}
        }

        self.parse_primary_expr()
    }

    fn parse_primary_expr(&mut self) -> Result<Expr, ()> {
        let token = &self.tokens[self.current];
        self.current += 1;
        match token.token_type {
            TokenType::FALSE
            | TokenType::TRUE
            | TokenType::NIL
            | TokenType::STRING
            | TokenType::NUMBER
            | TokenType::IDENTIFIER => {
                return Ok(Expr::Literal(token.clone()));
            }
            TokenType::LEFT_BRACE => {
                let exprs = self.parse_scope()?;
                return Ok(Expr::Scope(exprs));
            }
            TokenType::LEFT_PAREN => {
                let expr = self.parse_assignment()?;
                if !self.curr_matches_type(TokenType::RIGHT_PAREN) {
                    self.print_token_err("Missing ')'")?;
                }

                self.current += 1;
                return Ok(Expr::Grouping(Box::new(expr)));
            }
            TokenType::PRINT => {
                let expr = self.parse_assignment()?;
                return Ok(Expr::Stmt(Statement::PrintStmt(Box::new(expr))));
            }
            TokenType::VAR => {
                return self.variable_declaration();
            }
            TokenType::IF => {
                let conditionals = self.handle_if_stmt()?;
                return Ok(Expr::Stmt(Statement::IfStmt(conditionals)));
            }
            TokenType::WHILE => {
                let conditional = self.handle_conditional()?;
                return Ok(Expr::Stmt(Statement::WhileStmt(conditional)));
            }
            TokenType::FOR => {
                let (var_init, condition, var_update, expr) = self.handle_for_stmt()?;
                return Ok(Expr::Stmt(Statement::ForStmt(
                    var_init,
                    Box::new(condition),
                    var_update,
                    Box::new(expr),
                )));
            }
            _ => {
                self.current -= 1;
            }
        }

        self.print_token_err("Unexpected token or missing expression")?;
        Err(())
    }

    fn parse_scope(&mut self) -> Result<Vec<Expr>, ()> {
        let mut exprs: Vec<Expr> = vec![];
        loop {
            match self.tokens[self.current].token_type {
                TokenType::RIGHT_BRACE => {
                    self.current += 1;
                    break;
                }
                TokenType::EOF => {
                    self.print_token_err("Missing '}}'")?;
                }
                _ => {}
            }

            let expr = self.parse_expression()?;
            exprs.push(expr);
        }

        Ok(exprs)
    }

    fn handle_if_stmt(&mut self) -> Result<Vec<Conditional>, ()> {
        // also deals with else if and else blocks
        let mut conditionals: Vec<Conditional> = vec![];
        loop {
            let conditional = self.handle_conditional()?;
            conditionals.push(conditional);
            if self.tokens[self.current].token_type != TokenType::ELSE {
                break;
            }

            self.current += 1;
            if self.tokens[self.current].token_type != TokenType::IF {
                // should be the else block
                let expr = self.parse_expression()?;
                conditionals.push(Conditional(
                    Box::new(Expr::Literal(Token::new(
                        TokenType::TRUE,
                        String::from("true"),
                        String::from("null"),
                        self.tokens[self.current].line_num,
                    ))),
                    Box::new(expr),
                ));
                break;
            }

            self.current += 1; // loop after skipping the IF token
        }

        Ok(conditionals)
    }

    fn handle_conditional(&mut self) -> Result<Conditional, ()> {
        let condition = self.parse_primary_expr()?;
        match condition {
            Expr::Grouping(_) => {}
            _ => self.print_token_err(
                "Expected condition (make sure to enclose within parentheses '()')",
            )?,
        }

        let expr = self.parse_expression()?;
        Ok(Conditional(Box::new(condition), Box::new(expr)))
    }

    fn handle_for_stmt(
        &mut self,
    ) -> Result<(Option<Box<Expr>>, Expr, Option<Box<Expr>>, Expr), ()> {
        if self.tokens[self.current].token_type != TokenType::LEFT_PAREN {
            self.print_token_err("Expected for init expression/ loop condition (make sure to enclose within parentheses '()'")?;
        }

        self.current += 1;
        let mut var_init: Option<Box<Expr>> = None;
        if !self.curr_matches_type(TokenType::SEMICOLON) {
            let init_expr = self.parse_assignment()?;
            self.empty_scope_check(&init_expr)?;
            var_init = Some(Box::new(init_expr));
        }
        if !self.curr_matches_type(TokenType::SEMICOLON) {
            self.print_token_err("Missing ';'")?;
        }

        self.current += 1;
        if self.curr_matches_type(TokenType::SEMICOLON) {
            self.print_token_err("Expected condition for the loop")?;
        }

        let condition = self.parse_assignment()?;
        self.empty_scope_check(&condition)?;
        if !self.curr_matches_type(TokenType::SEMICOLON) {
            self.print_token_err("Missing ';'")?;
        }

        self.current += 1;
        let mut var_update: Option<Box<Expr>> = None;
        if !matches!(self.tokens[self.current].token_type, TokenType::RIGHT_PAREN) {
            let update_expr = self.parse_assignment()?;
            self.empty_scope_check(&update_expr)?;
            var_update = Some(Box::new(update_expr));
        }

        if !matches!(self.tokens[self.current].token_type, TokenType::RIGHT_PAREN) {
            self.print_token_err("Missing ')'")?;
        }

        self.current += 1;
        let expr = self.parse_expression()?;
        Ok((var_init, condition, var_update, expr))
    }

    fn variable_declaration(&mut self) -> Result<Expr, ()> {
        let variable = self.tokens[self.current].clone();
        let mut value = None;
        self.current += 1;
        if variable.token_type != TokenType::IDENTIFIER {
            return Err(());
        }

        if self.tokens[self.current].token_type == TokenType::EQUAL {
            self.current += 1;
            let expr = self.parse_assignment()?;

            value = Some(Box::new(expr));
        }

        Ok(Expr::Stmt(Statement::DeclarationStmt(variable, value)))
    }

    #[inline]
    fn empty_scope_check(&self, expr: &Expr) -> Result<(), ()> {
        match expr {
            Expr::Scope(ref exprs) => {
                if exprs.len() == 0 {
                    self.print_token_err("Invalid condition for the loop")?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    #[inline]
    fn print_token_err(&self, message: &str) -> Result<(), ()> {
        eprintln!(
            "[line {}] Error at '{}': {}.",
            self.tokens[self.current].line_num, self.tokens[self.current].lexeme, message
        );

        return Err(());
    }

    #[inline]
    fn curr_matches_type(&self, t_type: TokenType) -> bool {
        self.tokens[self.current].token_type == t_type
    }
}
