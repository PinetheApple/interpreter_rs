use crate::evaluate::Eval;
use codecrafters_interpreter::{Expr, Statement, Token, TokenType};
use std::collections::HashMap;
mod tests;

pub struct State {
    scopes: Vec<Scope>,
    len: usize,
}

struct Scope {
    variables: HashMap<String, Token>,
}

impl Scope {
    fn new() -> Self {
        Scope {
            variables: HashMap::new(),
        }
    }

    fn has_var(&self, name: &str) -> bool {
        if self.variables.contains_key(name) {
            return true;
        }

        false
    }
}

impl State {
    pub fn new() -> Self {
        State {
            scopes: vec![Scope::new()],
            len: 0,
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
            Expr::Stmt(statement) => match statement {
                Statement::PrintStmt(expr) => {
                    let output = self.evaluate(*expr)?;
                    if output.token_type == TokenType::STRING {
                        println!("{}", output.literal);
                    } else {
                        println!("{}", output.lexeme);
                    }
                }
                Statement::DeclarationStmt(variable, value) => self.declare(variable, value)?,
                Statement::AssignmentStmt(variable, value) => {
                    let _ = self.assign(variable, value)?;
                    return Ok(());
                }
                Statement::IfStmt(conditionals) => {
                    for conditional in conditionals {
                        let condition = self.evaluate(*conditional.0)?;
                        if Self::get_bool(condition)? {
                            self.run_expression(*conditional.1)?;
                            break;
                        }
                    }
                }
                Statement::WhileStmt(conditional) => loop {
                    let condition = self.evaluate(*conditional.0.clone())?;
                    if !Self::get_bool(condition)? {
                        break;
                    }

                    self.run_expression(*conditional.1.clone())?;
                },
                Statement::ForStmt(_) => todo!(),
            },
            Expr::Scope(exprs) => {
                self.len += 1;
                self.scopes.push(Scope::new());
                self.run(exprs)?;
                self.len -= 1;
                self.scopes.pop();
            }
            _ => {
                self.evaluate(expr)?;
            }
        }

        Ok(())
    }

    fn declare(&mut self, variable: Token, value: Option<Box<Expr>>) -> Result<(), ()> {
        match value {
            Some(expr) => {
                let value = self.evaluate(*expr)?;
                // add to variables list
                self.insert_var(variable.lexeme, value, self.len);
            }
            _ => {
                self.insert_var(
                    variable.lexeme,
                    Token::new(
                        TokenType::INVALID,
                        String::from("nil"),
                        String::from("null"),
                        variable.line_num,
                    ),
                    self.len,
                );
            }
        }

        Ok(())
    }

    fn assign(&mut self, variable: Token, value: Box<Expr>) -> Result<Token, ()> {
        let scope = self.has_var(&variable.lexeme);
        if scope == -1 {
            eprintln!(
                "[line {}] Undeclared variable: '{}'",
                variable.line_num, variable.lexeme
            );
            return Err(());
        }

        let token = self.evaluate(*value)?;
        self.insert_var(variable.lexeme, token.clone(), scope as usize);

        Ok(token)
    }

    fn has_var(&self, name: &str) -> i8 {
        let mut scope: i8 = -1;
        for s in (0..(self.len + 1)).rev() {
            if self.scopes[s].has_var(name) {
                scope = s as i8;
                break;
            }
        }

        scope
    }

    fn get_var(&mut self, name: &str, scope: usize) -> Token {
        let var_scope = &self.scopes[scope];

        var_scope.variables.get(name).unwrap().clone()
    }

    fn insert_var(&mut self, name: String, value: Token, scope: usize) {
        let var_scope = &mut self.scopes[scope];
        var_scope.variables.insert(name, value);
    }
}

impl Eval for State {
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
                    let scope = self.has_var(&token.lexeme);
                    if scope == -1 {
                        eprintln!(
                            "[line {}] Undeclared variable: '{}'",
                            token.line_num, token.lexeme
                        );
                        return Err(());
                    } else {
                        res = self.get_var(&token.lexeme, scope as usize);
                    }
                }
                _ => return Err(()),
            },
            Expr::Unary(operator, value) => res = self.eval_unary_expr(operator, *value)?,
            Expr::Grouping(expr) => res = self.eval_group_expr(*expr)?,
            Expr::Binary(left_expr, operator, right_expr) => {
                res = self.eval_binary_expr(*left_expr, operator, *right_expr)?
            }
            Expr::Logical(left_expr, operator, right_expr) => match operator.token_type {
                TokenType::OR => res = Self::eval_logical_or_expr(self, *left_expr, *right_expr)?,
                TokenType::AND => res = Self::eval_logical_and_expr(self, *left_expr, *right_expr)?,
                _ => panic!("this shouldn't happen"),
            },
            Expr::Stmt(Statement::AssignmentStmt(variable, value)) => {
                res = self.assign(variable, value)?;
            }
            _ => {
                eprintln!("Unexpected print/variable declaration statement.");
                return Err(());
            }
        }

        Ok(res)
    }
}
