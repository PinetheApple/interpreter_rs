use codecrafters_interpreter::{Expr, Token, TokenType};
use std::collections::HashMap;

use crate::evaluate::Eval;

pub struct State {
    variables: HashMap<String, Token>,
}

impl State {
    pub fn new() -> Self {
        State {
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self, expressions: Vec<Expr>) -> Result<(), ()> {
        for expr in expressions {
            self.run_expression(expr)?;
        }

        Ok(())
    }

    fn run_expression(&mut self, expr: Expr) -> Result<(), ()> {
        match expr {
            Expr::PrintStatement(expr) => {
                let output = self.evaluate(*expr)?;
                println!("{}", output.lexeme);
            }
            Expr::DeclarationStatment(var_def) => match var_def.value {
                Some(expr) => {
                    let value = self.evaluate(*expr)?;
                    // add to variables list
                    self.variables.insert(var_def.variable.lexeme, value);
                }
                _ => {}
            },
            _ => {
                self.evaluate(expr)?;
            }
        }

        Ok(())
    }
}

impl Eval for State {
    fn evaluate(&self, expr: Expr) -> Result<Token, ()> {
        let res: Token;
        match expr {
            Expr::Literal(token) => match token.token_type {
                TokenType::STRING
                | TokenType::NUMBER
                | TokenType::FALSE
                | TokenType::TRUE
                | TokenType::NIL => res = token,
                TokenType::IDENTIFIER => {
                    if !self.variables.contains_key(&token.lexeme) {
                        eprintln!(
                            "[line {}] Undefined variable '{}'",
                            token.line_num, token.lexeme
                        );
                        return Err(());
                    }

                    res = self.variables.get(&token.lexeme).unwrap().clone();
                }
                _ => return Err(()),
            },
            Expr::Unary(unary_expr) => res = Self::evaluate_unary_expr(self, unary_expr)?,
            Expr::Grouping(group_expr) => res = Self::evaluate_group_expr(self, group_expr)?,
            Expr::Binary(binary_expr) => res = Self::evaluate_binary_expr(self, binary_expr)?,
            Expr::PrintStatement(expr) => res = Self::evaluate(self, *expr)?,
            Expr::DeclarationStatment(dec_expr) => res = dec_expr.variable,
        }

        Ok(res)
    }
}
