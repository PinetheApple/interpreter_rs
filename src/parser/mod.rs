use codecrafters_interpreter::{
    BinaryExpr, Expr, GroupingExpr, Token, TokenType, UnaryExpr, VarDefinition,
};

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
            let curr_token_type = self.tokens[self.current].token_type;
            if curr_token_type == TokenType::EOF {
                break;
            }

            let expr = self.parse_expression()?;
            expressions.push(expr);
            let semicolon = &self.tokens[self.current];
            self.current += 1;
            if semicolon.token_type != TokenType::SEMICOLON {
                eprintln!("[line {}] Error: missing ';'", semicolon.line_num);
                return Err(());
            }
        }

        Ok(expressions)
    }

    pub fn parse_expression(&mut self) -> Result<Expr, ()> {
        self.parse_equality()
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
            expr = Expr::Binary(BinaryExpr::new(expr, operator.clone(), right));
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
            expr = Expr::Binary(BinaryExpr::new(expr, operator.clone(), right));
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
            expr = Expr::Binary(BinaryExpr::new(expr, operator.clone(), right));
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
            expr = Expr::Binary(BinaryExpr::new(expr, operator.clone(), right));
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
                return Ok(Expr::Unary(UnaryExpr::new(operator.clone(), right)));
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
            TokenType::LEFT_PAREN => {
                let expr = self.parse_expression()?;
                let right_paren = &self.tokens[self.current];
                self.current += 1;
                if right_paren.token_type != TokenType::RIGHT_PAREN {
                    eprintln!("[line {}] Error: missing ')'", right_paren.line_num);
                    return Err(());
                }

                return Ok(Expr::Grouping(GroupingExpr::new(expr)));
            }
            TokenType::PRINT => {
                let expr = self.parse_expression()?;
                return Ok(Expr::PrintStatement(Box::new(expr)));
            }
            TokenType::VAR => {
                return self.variable_declaration();
            }
            _ => {
                self.current -= 1;
            }
        }

        eprintln!("Error: missing expression.");
        Err(())
    }

    fn variable_declaration(&mut self) -> Result<Expr, ()> {
        // also deals with variable definitions
        let variable = self.tokens[self.current].clone();
        let mut value = None;
        self.current += 1;
        if variable.token_type != TokenType::IDENTIFIER {
            return Err(());
        }

        if self.tokens[self.current].token_type == TokenType::EQUAL {
            self.current += 1;
            let expr = self.parse_expression()?;
            value = Some(expr);
        }

        Ok(Expr::DeclarationStatment(VarDefinition::new(
            variable, value,
        )))
    }
}
