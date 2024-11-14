use codecrafters_interpreter::{Token, TokenType};

pub fn parse<I>(token_iter: &mut I, is_group: bool) -> (Vec<String>, i32)
where
    I: Iterator<Item = Token>,
{
    let mut status_code = 0;
    let mut parsed_output: Vec<String> = vec![];
    let mut token_op = token_iter.next();

    loop {
        if let Some(token) = token_op {
            match token.token_type {
                TokenType::EOF => break,
                TokenType::TRUE | TokenType::FALSE | TokenType::NIL => {
                    parsed_output.push(token.lexeme.clone())
                }
                TokenType::NUMBER | TokenType::STRING => parsed_output.push(token.literal.clone()),
                TokenType::LEFT_PAREN => {
                    let (group_str, group_status_code) = parse_group(token_iter);
                    if group_status_code != 0 {
                        status_code = group_status_code;
                    }

                    parsed_output.push(format!("(group {})", group_str));
                }
                TokenType::RIGHT_PAREN => {
                    if is_group {
                        return (parsed_output, status_code);
                    } else {
                        status_code = 65;
                    }
                }
                TokenType::MINUS => {
                    let (unary_str, group_status_code) =
                        parse_unary_operations(token_iter, is_group);
                    if group_status_code != 0 {
                        status_code = group_status_code;
                    }

                    parsed_output.push(format!("(- {})", unary_str));
                }
                TokenType::BANG => {
                    let (unary_str, group_status_code) =
                        parse_unary_operations(token_iter, is_group);
                    if group_status_code != 0 {
                        status_code = group_status_code;
                    }

                    parsed_output.push(format!("(! {})", unary_str));
                }
                TokenType::SLASH | TokenType::STAR => {
                    if let Some(prev_val) = parsed_output.pop() {
                        let (op_str, group_status_code) =
                            parse_operations(token_iter, token.lexeme, prev_val);
                        if group_status_code != 0 {
                            status_code = group_status_code;
                        }

                        parsed_output.push(op_str);
                    } else {
                        status_code = 65;
                    }
                }
                _ => {
                    status_code = 65;
                }
            }
        } else {
            break;
        }

        token_op = token_iter.next();
    }

    (parsed_output, status_code)
}

fn parse_group<I>(token_iter: &mut I) -> (String, i32)
where
    I: Iterator<Item = Token>,
{
    let mut status_code = 0;
    let mut group_str = String::new();
    let (parsed_group, group_status_code) = parse(token_iter, true);
    for parsed_line in parsed_group {
        group_str = format!("{}{}", group_str, parsed_line);
    }

    if group_status_code != 0 {
        status_code = group_status_code;
    }

    (group_str, status_code)
}

fn parse_unary_operations<I>(token_iter: &mut I, is_group: bool) -> (String, i32)
where
    I: Iterator<Item = Token>,
{
    let mut status_code = 0;
    let mut unary_str = String::new();
    let (parsed_group, group_status_code) = parse(token_iter, is_group);
    for parsed_line in parsed_group {
        unary_str = format!("{}{}", unary_str, parsed_line);
    }

    if group_status_code != 0 {
        status_code = group_status_code;
    }

    (unary_str, status_code)
}

fn parse_operations<I>(token_iter: &mut I, operator: String, prev_val: String) -> (String, i32)
where
    I: Iterator<Item = Token>,
{
    let mut status_code = 0;
    let mut op_str = String::new();
    if let Some(token) = token_iter.next() {
        op_str = format!("({} {} {})", operator, prev_val, token.literal);
    } else {
        status_code = 65;
    }

    (op_str, status_code)
}
