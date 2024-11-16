use codecrafters_interpreter::{
    BinaryExpr, Expr, GroupingExpr, LiteralExpr, Token, TokenType, UnaryExpr,
};

pub struct Parser {
    pub tokens: Vec<Token>,
    current: usize,
    stack: Vec<char>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            stack: vec![],
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ()> {
        let expr = self.expression();
        if self.stack.len() > 0 {
            eprintln!(
                "[line {}] Unmatched parentheses",
                self.tokens[self.current].line_num
            );
        }

        expr
    }

    fn expression(&mut self) -> Result<Expr, ()> {
        //self.equality()
        self.factor()
    }

    //fn equality(&self) -> Result<Expr, ()> {
    //    let mut expr = self.comparsion()?;
    //
    //    Ok(expr)
    //}
    //
    //fn comparsion(&self) -> Result<Expr, ()> {
    //    let mut expr = self.term()?;
    //
    //    Ok(expr)
    //}
    //
    //fn term(&self) -> Result<Expr, ()> {
    //    let mut expr = self.factor();
    //
    //    expr
    //}

    fn factor(&mut self) -> Result<Expr, ()> {
        let mut expr = self.unary()?;

        while matches!(
            self.tokens[self.current].token_type,
            TokenType::STAR | TokenType::SLASH
        ) {
            if self.current == 0 {
                return Err(());
            }
            let operator = &self.tokens[self.current].clone();
            self.current += 1;
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr::new(expr, operator.clone(), right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ()> {
        let token = &self.tokens[self.current].clone();
        match token.token_type {
            TokenType::BANG | TokenType::MINUS => {
                let operator = token;
                self.current += 1;
                let right = self.unary()?;
                return Ok(Expr::Unary(UnaryExpr::new(operator.clone(), right)));
            }
            _ => {}
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ()> {
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
                self.stack.push('(');
                let expr: Expr = self.expression()?;
                return Ok(Expr::Grouping(GroupingExpr::new(expr)));
            }
            TokenType::RIGHT_PAREN => {
                if self.stack.len() == 0 {
                    eprintln!("[line {}] Error at ')': Unexpected token.", token.line_num);
                    return Err(());
                }
                self.stack.pop();
            }
            _ => {
                self.current -= 1;
            }
        }

        eprintln!("Error: missing expression.");
        Err(())
    }
}
