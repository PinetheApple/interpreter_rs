use codecrafters_interpreter::{Assignment, Expr, Token, TokenType, VarDefinition};
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
                if output.token_type == TokenType::STRING {
                    println!("{}", output.literal);
                } else {
                    println!("{}", output.lexeme);
                }
            }
            Expr::DeclarationStatment(var_def) => self.declare(var_def)?,
            Expr::AssignmentStatement(assignment) => {
                let _ = self.assign(assignment)?;
                return Ok(());
            }
            _ => {
                self.evaluate(expr)?;
            }
        }

        Ok(())
    }

    fn declare(&mut self, var_def: VarDefinition) -> Result<(), ()> {
        match var_def.value {
            Some(expr) => {
                let value = self.evaluate(*expr)?;
                // add to variables list
                self.variables.insert(var_def.variable.lexeme, value);
            }
            _ => {
                self.variables.insert(
                    var_def.variable.lexeme,
                    Token::new(
                        TokenType::INVALID,
                        String::from("nil"),
                        String::from("null"),
                        var_def.variable.line_num,
                    ),
                );
            }
        }

        Ok(())
    }

    fn assign(&mut self, assignment: Assignment) -> Result<Token, ()> {
        if !self.variables.contains_key(&assignment.variable.lexeme) {
            eprintln!(
                "[line {}] Undeclared variable: '{}'",
                assignment.variable.line_num, assignment.variable.lexeme
            );
            return Err(());
        }

        let token = self.evaluate(*assignment.value)?;
        self.variables
            .insert(assignment.variable.lexeme, token.clone());

        Ok(token)
    }

    fn evaluate(&mut self, expr: Expr) -> Result<Token, ()> {
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
                            "[line {}] Undeclared variable: '{}'",
                            token.line_num, token.lexeme
                        );
                        return Err(());
                    } else {
                        res = self.variables.get(&token.lexeme).unwrap().clone();
                    }
                }
                _ => return Err(()),
            },
            Expr::Unary(unary_expr) => res = self.evaluate_unary_expr(unary_expr)?,
            Expr::Grouping(group_expr) => res = self.evaluate_group_expr(group_expr)?,
            Expr::Binary(binary_expr) => res = self.evaluate_binary_expr(binary_expr)?,
            Expr::AssignmentStatement(assignment) => {
                res = self.assign(assignment)?;
            }
            _ => {
                eprintln!("Unexpected print/variable declaration statement.");
                return Err(());
            }
        }

        Ok(res)
    }
}

impl Eval for State {}
