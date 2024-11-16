use codecrafters_interpreter::{
    BinaryExpr, Expr, GroupingExpr, LiteralExpr, Token, TokenType, UnaryExpr,
};

pub struct Parser {
    pub tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ()> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Expr, ()> {
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
            TokenType::FALSE | TokenType::TRUE | TokenType::NIL => {
                return Ok(Expr::Literal(LiteralExpr::new(
                    token.token_type,
                    token.lexeme.clone(),
                )));
            }
            TokenType::STRING | TokenType::NUMBER => {
                return Ok(Expr::Literal(LiteralExpr::new(
                    token.token_type,
                    token.literal.clone(),
                )))
            }
            TokenType::LEFT_PAREN => {
                let expr: Expr = self.parse_expression()?;
                let right_paren = &self.tokens[self.current];
                self.current += 1;
                if right_paren.token_type != TokenType::RIGHT_PAREN {
                    eprintln!("[line {}] Error: missing ')'", right_paren.line_num);
                    return Err(());
                }

                return Ok(Expr::Grouping(GroupingExpr::new(expr)));
            }
            _ => {
                self.current -= 1;
            }
        }

        eprintln!("Error: missing expression.");
        Err(())
    }
}
