use codecrafters_interpreter::{
    BinaryExpr, Expr, GroupingExpr, LiteralExpr, Token, TokenType, UnaryExpr,
};

pub fn evaluate(expr: Expr) -> Token {
    let res: Token;
    match expr {
        Expr::Literal(lit_expr) => res = evaluate_literal_expr(lit_expr),
        //Expr::Unary(unary_expr) => res = evaluate_unary_expr(unary_expr),
        Expr::Grouping(group_expr) => res = evaluate_group_expr(group_expr),
        //Expr::Binary(binary_expr) => res = evaluate_binary_expr(binary_expr),
        _ => res = Token::new(TokenType::INVALID, String::new(), String::new(), 0),
    }

    res
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

fn evaluate_group_expr(expr: GroupingExpr) -> Token {
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

//fn evaluate_unary_expr(expr: UnaryExpr) -> Token {
//    let right = evaluate(*expr.val);
//    match expr.operator.token_type {
//        TokenType::MINUS => {
//            res = format!("-{}", right);
//        }
//        TokenType::BANG => match right.as_str() {
//            _ => {}
//        },
//        _ => {}
//    }
//}
//
//
//fn evaluate_binary_expr(expr: BinaryExpr) -> Token {}
