use codecrafters_interpreter::{Expr, Statement, Token, TokenType};

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
            if matches!(curr_token_type, TokenType::EOF) {
                break;
            }

            if matches!(curr_token_type, TokenType::RIGHT_BRACE) {
                return Ok(expressions);
            }

            let expr = self.parse_expression()?;
            expressions.push(expr);
        }

        Ok(expressions)
    }

    pub fn parse_expression(&mut self) -> Result<Expr, ()> {
        let expr = self.parse_assignment()?;
        if matches!(expr, Expr::Scope(_)) {
            return Ok(expr);
        }

        if !matches!(self.tokens[self.current].token_type, TokenType::SEMICOLON) {
            eprintln!(
                "[line {}] Error: missing ';'",
                self.tokens[self.current].line_num
            );
            return Err(());
        }

        self.current += 1;
        Ok(expr)
    }

    pub fn parse_assignment(&mut self) -> Result<Expr, ()> {
        // check if the start is an identifier if it followed by EQUAL token
        let mut expr = self.parse_equality()?;

        while matches!(self.tokens[self.current].token_type, TokenType::EQUAL) {
            match expr {
                Expr::Literal(token) => {
                    if token.token_type != TokenType::IDENTIFIER {
                        eprintln!(
                            "[line {}] Cannot assign to non-identifiers.",
                            self.tokens[self.current].line_num
                        );
                        return Err(());
                    }
                    self.current += 1;
                    let value = self.parse_assignment()?;
                    expr = Expr::Stmt(Statement::AssignmentStmt(token, Box::new(value)));
                }
                _ => {
                    eprintln!(
                        "[line {}] Cannot assign to non-identifiers.",
                        self.tokens[self.current].line_num
                    );
                    return Err(());
                }
            }
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
                let right_paren = &self.tokens[self.current];
                self.current += 1;
                if right_paren.token_type != TokenType::RIGHT_PAREN {
                    eprintln!("[line {}] Error: missing ')'", right_paren.line_num);
                    return Err(());
                }

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
                return self.handle_conditional();
            }
            _ => {
                self.current -= 1;
            }
        }

        eprintln!(
            "[line {}] Error: Unexpected token: '{}' or missing expression.",
            token.line_num, token
        );
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
                    eprintln!(
                        "[line {}] Error: missing '}}'",
                        self.tokens[self.current].line_num
                    );
                    return Err(());
                }
                _ => {}
            }

            let expr = self.parse_expression()?;
            exprs.push(expr);
        }

        Ok(exprs)
    }

    fn handle_conditional(&mut self) -> Result<Expr, ()> {
        Err(())
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
}
