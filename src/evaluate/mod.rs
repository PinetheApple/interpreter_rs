use codecrafters_interpreter::{
    BinaryExpr, Expr, GroupingExpr, LiteralExpr, Token, TokenType, UnaryExpr,
};

pub fn evaluate(expr: Expr) -> Result<Token, ()> {
    let res: Token;
    match expr {
        Expr::Literal(lit_expr) => res = evaluate_literal_expr(lit_expr),
        Expr::Unary(unary_expr) => res = evaluate_unary_expr(unary_expr)?,
        Expr::Grouping(group_expr) => res = evaluate_group_expr(group_expr)?,
        //Expr::Binary(binary_expr) => res = evaluate_binary_expr(binary_expr)?,
        _ => res = Token::new(TokenType::INVALID, String::new(), String::new(), 0),
    }

    Ok(res)
}

fn evaluate_literal_expr(expr: LiteralExpr) -> Token {
    let mut token = Token::new(expr.literal_type, String::new(), expr.val.clone(), 0);
    match expr.literal_type {
        TokenType::STRING => token.lexeme = format!("\"{}\"", expr.val),
        TokenType::NUMBER => token.lexeme = expr.val.parse::<f32>().unwrap().to_string(),
        _ => token.lexeme = expr.val,
    }

    token
}

fn evaluate_unary_expr(expr: UnaryExpr) -> Result<Token, ()> {
    let mut token = Token::new(TokenType::INVALID, String::new(), String::new(), 0);
    let right = evaluate(*expr.val)?;
    match expr.operator.token_type {
        TokenType::MINUS => {
            if right.token_type != TokenType::NUMBER {
                eprintln!("Operand must be a number.\n[line {}]", right.line_num);
                return Err(());
            }
            token.token_type = TokenType::NUMBER;
            token.lexeme = format!("-{}", right.lexeme);
            token.literal = format!("-{}", right.literal);
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

fn evaluate_group_expr(expr: GroupingExpr) -> Result<Token, ()> {
    evaluate(*expr.expression)
}

//pub struct Token {
//    pub token_type: TokenType,
//    pub lexeme: String,
//    pub literal: String,
//    pub line_num: u32,
//}

//pub struct UnaryExpr {
//    pub operator: Token,
//    pub val: Box<Expr>,
//}
//
//pub struct LiteralExpr {
//    pub literal_type: TokenType,
//    pub val: String,
//}
//
//pub struct BinaryExpr {
//    pub left_val: Box<Expr>,
//    pub operator: Token,
//    pub right_val: Box<Expr>,
//}
//
//STRING  lexeme-"foo baz"  literal-foo baz
//NUMBER  lexeme-42  literal-42.0

//
//
//fn evaluate_binary_expr(expr: BinaryExpr) -> Token {}
