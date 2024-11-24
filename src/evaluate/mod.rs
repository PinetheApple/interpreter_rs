use codecrafters_interpreter::{Expr, Token, TokenType};
mod tests;

pub struct Stateless;

impl Eval for Stateless {}

pub trait Eval {
    fn evaluate(&mut self, expr: Expr) -> Result<Token, ()> {
        let res: Token;
        match expr {
            Expr::Literal(token) => res = token,
            Expr::Unary(operator, val) => res = Self::eval_unary_expr(self, operator, *val)?,
            Expr::Grouping(expr) => res = Self::eval_group_expr(self, *expr)?,
            Expr::Binary(left_expr, operator, right_expr) => {
                res = Self::eval_binary_expr(self, *left_expr, operator, *right_expr)?;
            }
            Expr::Logical(left_expr, operator, right_expr) => match operator.token_type {
                TokenType::OR => res = Self::eval_logical_or_expr(self, *left_expr, *right_expr)?,
                TokenType::AND => res = Self::eval_logical_and_expr(self, *left_expr, *right_expr)?,
                _ => panic!("this shouldn't happen"),
            },
            _ => res = Token::new(TokenType::INVALID, String::new(), String::new(), 0),
        }

        Ok(res)
    }

    fn eval_logical_or_expr(&mut self, left_expr: Expr, right_expr: Expr) -> Result<Token, ()> {
        let token: Token;
        let left_val = Self::evaluate(self, left_expr)?;
        if Self::get_bool(left_val.clone())? {
            token = left_val;
        } else {
            token = Self::evaluate(self, right_expr)?;
        }

        Ok(token)
    }

    fn eval_logical_and_expr(&mut self, left_expr: Expr, right_expr: Expr) -> Result<Token, ()> {
        let token: Token;
        let left_val = Self::evaluate(self, left_expr)?;
        if !Self::get_bool(left_val.clone())? {
            token = left_val;
        } else {
            token = Self::evaluate(self, right_expr)?;
        }

        Ok(token)
    }

    fn eval_unary_expr(&mut self, operator: Token, val: Expr) -> Result<Token, ()> {
        let mut token = Token::new(TokenType::INVALID, String::new(), String::new(), 0);
        let right = Self::evaluate(self, val)?;
        match operator.token_type {
            TokenType::MINUS => {
                if right.token_type != TokenType::NUMBER {
                    eprintln!("Operand must be a number.\n[line {}]", right.line_num);
                    return Err(());
                }

                token.token_type = TokenType::NUMBER;
                if right.lexeme.starts_with("-") {
                    token.lexeme = format!("{}", &right.lexeme[1..]);
                    token.literal = format!("{}", &right.literal[1..]);
                } else {
                    token.lexeme = format!("-{}", right.lexeme);
                    token.literal = format!("-{}", right.literal);
                }
            }
            TokenType::BANG => {
                token.literal = String::from("null");
                match Self::get_bool(right) {
                    Ok(flag) => {
                        if flag {
                            token.token_type = TokenType::FALSE;
                            token.lexeme = String::from("false");
                        } else {
                            token.token_type = TokenType::TRUE;
                            token.lexeme = String::from("true");
                        }
                    }
                    Err(_) => {
                        return Err(());
                    }
                }
            }
            _ => {
                panic!("this shouldn't happen");
            }
        }

        Ok(token)
    }

    fn eval_group_expr(&mut self, expr: Expr) -> Result<Token, ()> {
        Self::evaluate(self, expr)
    }

    fn eval_binary_expr(
        &mut self,
        left_expr: Expr,
        operator: Token,
        right_expr: Expr,
    ) -> Result<Token, ()> {
        let token: Token;
        let left = Self::evaluate(self, left_expr)?;
        let right = Self::evaluate(self, right_expr)?;
        let operator_type = operator.token_type;
        match operator_type {
            TokenType::PLUS | TokenType::MINUS | TokenType::STAR | TokenType::SLASH => {
                token = Self::eval_arithmetic_op(left, right, operator_type)?;
            }
            TokenType::GREATER_EQUAL
            | TokenType::GREATER
            | TokenType::LESS
            | TokenType::LESS_EQUAL
            | TokenType::EQUAL_EQUAL
            | TokenType::BANG_EQUAL => token = Self::eval_comparison(left, right, operator_type)?,
            _ => {
                panic!("this shouldn't happen");
            }
        }

        Ok(token)
    }

    fn eval_arithmetic_op(
        left_token: Token,
        right_token: Token,
        operator_type: TokenType,
    ) -> Result<Token, ()> {
        let token: Token;
        match operator_type {
            TokenType::PLUS => {
                let str_type = TokenType::STRING;
                if left_token.token_type == str_type && right_token.token_type == str_type {
                    token = Self::concat_strings(left_token, right_token);
                } else if Self::num_check(left_token.token_type, right_token.token_type) {
                    token = Self::add(left_token, right_token);
                } else {
                    eprintln!("Operands must be two numbers or two strings.");
                    return Err(());
                }
            }
            TokenType::MINUS => token = Self::subtract(left_token, right_token)?,
            TokenType::STAR => token = Self::multiply(left_token, right_token)?,
            TokenType::SLASH => token = Self::divide(left_token, right_token)?,
            _ => return Err(()),
        }

        Ok(token)
    }

    fn eval_comparison(
        left_token: Token,
        right_token: Token,
        operator_type: TokenType,
    ) -> Result<Token, ()> {
        let true_token = Ok(Token::new(
            TokenType::TRUE,
            String::from("true"),
            String::from("null"),
            left_token.line_num,
        ));
        let false_token = Ok(Token::new(
            TokenType::FALSE,
            String::from("false"),
            String::from("null"),
            left_token.line_num,
        ));
        match operator_type {
            TokenType::EQUAL_EQUAL => {
                if (left_token.token_type != right_token.token_type)
                    || (left_token.lexeme != right_token.lexeme)
                {
                    return false_token;
                }

                return true_token;
            }
            TokenType::BANG_EQUAL => {
                if (left_token.token_type != right_token.token_type)
                    || (left_token.lexeme != right_token.lexeme)
                {
                    return true_token;
                }

                return false_token;
            }
            TokenType::GREATER_EQUAL => {
                if !Self::num_check(left_token.token_type, right_token.token_type) {
                    return Err(());
                }

                let (num1, num2) = Self::parse_nums(left_token.literal, right_token.literal);
                if num1 >= num2 {
                    return true_token;
                }

                return false_token;
            }
            TokenType::GREATER => {
                if !Self::num_check(left_token.token_type, right_token.token_type) {
                    return Err(());
                }

                let (num1, num2) = Self::parse_nums(left_token.literal, right_token.literal);
                if num1 > num2 {
                    return true_token;
                }

                return false_token;
            }
            TokenType::LESS => {
                if !Self::num_check(left_token.token_type, right_token.token_type) {
                    return Err(());
                }

                let (num1, num2) = Self::parse_nums(left_token.literal, right_token.literal);
                if num1 < num2 {
                    return true_token;
                }

                return false_token;
            }
            TokenType::LESS_EQUAL => {
                if !Self::num_check(left_token.token_type, right_token.token_type) {
                    return Err(());
                }

                let (num1, num2) = Self::parse_nums(left_token.literal, right_token.literal);
                if num1 <= num2 {
                    return true_token;
                }

                return false_token;
            }
            _ => return Err(()),
        }
    }

    fn get_bool(token: Token) -> Result<bool, ()> {
        let mut flag = false;
        if matches!(token.token_type, TokenType::TRUE | TokenType::STRING)
            || (token.token_type == TokenType::NUMBER && token.literal != "0")
        {
            flag = true;
        } else if matches!(
            token.token_type,
            TokenType::FALSE | TokenType::NIL | TokenType::NUMBER
        ) {
        } else {
            eprintln!("[line {}] Invalid condition used.", token.line_num);
            return Err(());
        }

        Ok(flag)
    }

    fn concat_strings(str1_token: Token, str2_token: Token) -> Token {
        Token::new(
            TokenType::STRING,
            format!("\"{}{}\"", str1_token.literal, str2_token.literal),
            format!("{}{}", str1_token.literal, str2_token.literal),
            0,
        )
    }

    fn add(val1_token: Token, val2_token: Token) -> Token {
        let (num1, num2) = Self::parse_nums(val1_token.literal, val2_token.literal);
        let res = num1 + num2;

        Token::new(
            TokenType::NUMBER,
            Self::parse_lexeme(res.to_string()),
            Self::parse_literal(res.to_string()),
            0,
        )
    }

    fn subtract(val1_token: Token, val2_token: Token) -> Result<Token, ()> {
        if !Self::num_check(val1_token.token_type, val2_token.token_type) {
            return Err(());
        }

        let (num1, num2) = Self::parse_nums(val1_token.literal, val2_token.literal);
        let res = num1 - num2;

        Ok(Token::new(
            TokenType::NUMBER,
            Self::parse_lexeme(res.to_string()),
            Self::parse_literal(res.to_string()),
            0,
        ))
    }

    fn multiply(val1_token: Token, val2_token: Token) -> Result<Token, ()> {
        if !Self::num_check(val1_token.token_type, val2_token.token_type) {
            return Err(());
        }

        let (num1, num2) = Self::parse_nums(val1_token.literal, val2_token.literal);
        let res = num1 * num2;

        Ok(Token::new(
            TokenType::NUMBER,
            Self::parse_lexeme(res.to_string()),
            Self::parse_literal(res.to_string()),
            0,
        ))
    }

    fn divide(val1_token: Token, val2_token: Token) -> Result<Token, ()> {
        if !Self::num_check(val1_token.token_type, val2_token.token_type) {
            return Err(());
        }

        let (num1, num2) = Self::parse_nums(val1_token.literal, val2_token.literal);
        let res = num1 / num2;

        Ok(Token::new(
            TokenType::NUMBER,
            Self::parse_lexeme(res.to_string()),
            Self::parse_literal(res.to_string()),
            0,
        ))
    }

    fn parse_literal(val: String) -> String {
        match val.parse::<i32>() {
            Ok(_) => return format!("{}.0", val),
            _ => return val,
        }
    }

    fn parse_lexeme(val: String) -> String {
        val.parse::<f32>().unwrap().to_string()
    }

    fn parse_nums(val1: String, val2: String) -> (f32, f32) {
        (val1.parse::<f32>().unwrap(), val2.parse::<f32>().unwrap())
    }

    fn num_check(type1: TokenType, type2: TokenType) -> bool {
        let num_type = TokenType::NUMBER;
        if !(type1 == num_type && type2 == num_type) {
            eprintln!("Operands must be numbers");
            return false;
        }

        true
    }
}
