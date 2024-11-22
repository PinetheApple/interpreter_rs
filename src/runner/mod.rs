use codecrafters_interpreter::{Assignment, Expr, Token, TokenType, VarDefinition};
use std::collections::HashMap;

use crate::evaluate::Eval;

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

    fn has_var(&mut self, name: &str) -> bool {
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

    fn declare(&mut self, var_def: VarDefinition) -> Result<(), ()> {
        match var_def.value {
            Some(expr) => {
                let value = self.evaluate(*expr)?;
                // add to variables list
                self.insert_var(var_def.variable.lexeme, value, self.len);
            }
            _ => {
                self.insert_var(
                    var_def.variable.lexeme,
                    Token::new(
                        TokenType::INVALID,
                        String::from("nil"),
                        String::from("null"),
                        var_def.variable.line_num,
                    ),
                    self.len,
                );
            }
        }

        Ok(())
    }

    fn assign(&mut self, assignment: Assignment) -> Result<Token, ()> {
        let scope = self.has_var(&assignment.variable.lexeme);
        if scope == -1 {
            eprintln!(
                "[line {}] Undeclared variable: '{}'",
                assignment.variable.line_num, assignment.variable.lexeme
            );
            return Err(());
        }

        let token = self.evaluate(*assignment.value)?;
        self.insert_var(assignment.variable.lexeme, token.clone(), scope as usize);

        Ok(token)
    }

    fn has_var(&mut self, name: &str) -> i8 {
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
