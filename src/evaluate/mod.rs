use codecrafters_interpreter::{BinaryExpr, Expr, GroupingExpr, Token, TokenType, UnaryExpr};

pub struct Stateless;

impl Eval for Stateless {}

pub trait Eval {
    fn evaluate(&mut self, expr: Expr) -> Result<Token, ()> {
        let res: Token;
        match expr {
            Expr::Literal(token) => res = token,
            Expr::Unary(unary_expr) => res = Self::evaluate_unary_expr(self, unary_expr)?,
            Expr::Grouping(group_expr) => res = Self::evaluate_group_expr(self, group_expr)?,
            Expr::Binary(binary_expr) => res = Self::evaluate_binary_expr(self, binary_expr)?,
            _ => res = Token::new(TokenType::INVALID, String::new(), String::new(), 0),
        }

        Ok(res)
    }

    fn evaluate_unary_expr(&mut self, expr: UnaryExpr) -> Result<Token, ()> {
        let mut token = Token::new(TokenType::INVALID, String::new(), String::new(), 0);
        let right = Self::evaluate(self, *expr.val)?;
        match expr.operator.token_type {
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
                match right.lexeme.as_str() {
                    "false" | "0" | "nil" => {
                        token.token_type = TokenType::TRUE;
                        token.lexeme = String::from("true");
                    }
                    _ => {
                        token.token_type = TokenType::FALSE;
                        token.lexeme = String::from("false");
                    }
                }
            }
            _ => {}
        }

        Ok(token)
    }

    fn evaluate_group_expr(&mut self, expr: GroupingExpr) -> Result<Token, ()> {
        Self::evaluate(self, *expr.expression)
    }

    fn evaluate_binary_expr(&mut self, expr: BinaryExpr) -> Result<Token, ()> {
        let token: Token;
        let left = Self::evaluate(self, *expr.left_val)?;
        let right = Self::evaluate(self, *expr.right_val)?;
        let operator_type = expr.operator.token_type;
        match operator_type {
            TokenType::PLUS | TokenType::MINUS | TokenType::STAR | TokenType::SLASH => {
                token = Self::evaluate_arithmetic_op(left, right, operator_type)?;
            }
            TokenType::GREATER_EQUAL
            | TokenType::GREATER
            | TokenType::LESS
            | TokenType::LESS_EQUAL
            | TokenType::EQUAL_EQUAL
            | TokenType::BANG_EQUAL => {
                token = Self::evaluate_comparison(left, right, operator_type)?
            }
            _ => {
                return Err(());
            }
        }

        Ok(token)
    }

    fn evaluate_arithmetic_op(
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

    fn evaluate_comparison(
        left_token: Token,
        right_token: Token,
        operator_type: TokenType,
    ) -> Result<Token, ()> {
        let mut token = Token::new(TokenType::INVALID, String::new(), String::from("null"), 0);
        let (false_type, true_type) = (TokenType::FALSE, TokenType::TRUE);
        match operator_type {
            TokenType::EQUAL_EQUAL => {
                if (left_token.token_type != right_token.token_type)
                    || (left_token.lexeme != right_token.lexeme)
                {
                    token.token_type = false_type;
                    token.lexeme = String::from("false");
                } else {
                    token.token_type = true_type;
                    token.lexeme = String::from("true");
                }
            }
            TokenType::BANG_EQUAL => {
                if (left_token.token_type != right_token.token_type)
                    || (left_token.lexeme != right_token.lexeme)
                {
                    token.token_type = true_type;
                    token.lexeme = String::from("true");
                } else {
                    token.token_type = false_type;
                    token.lexeme = String::from("false");
                }
            }
            TokenType::GREATER_EQUAL => {
                if !Self::num_check(left_token.token_type, right_token.token_type) {
                    return Err(());
                }

                let (num1, num2) = Self::parse_nums(left_token.literal, right_token.literal);
                if num1 >= num2 {
                    token.token_type = true_type;
                    token.lexeme = String::from("true");
                } else {
                    token.token_type = false_type;
                    token.lexeme = String::from("false");
                }
            }
            TokenType::GREATER => {
                if !Self::num_check(left_token.token_type, right_token.token_type) {
                    return Err(());
                }

                let (num1, num2) = Self::parse_nums(left_token.literal, right_token.literal);
                if num1 > num2 {
                    token.token_type = true_type;
                    token.lexeme = String::from("true");
                } else {
                    token.token_type = false_type;
                    token.lexeme = String::from("false");
                }
            }
            TokenType::LESS => {
                if !Self::num_check(left_token.token_type, right_token.token_type) {
                    return Err(());
                }

                let (num1, num2) = Self::parse_nums(left_token.literal, right_token.literal);
                if num1 < num2 {
                    token.token_type = true_type;
                    token.lexeme = String::from("true");
                } else {
                    token.token_type = false_type;
                    token.lexeme = String::from("false");
                }
            }
            TokenType::LESS_EQUAL => {
                if !Self::num_check(left_token.token_type, right_token.token_type) {
                    return Err(());
                }

                let (num1, num2) = Self::parse_nums(left_token.literal, right_token.literal);
                if num1 <= num2 {
                    token.token_type = true_type;
                    token.lexeme = String::from("true");
                } else {
                    token.token_type = false_type;
                    token.lexeme = String::from("false");
                }
            }
            _ => return Err(()),
        }

        Ok(token)
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
