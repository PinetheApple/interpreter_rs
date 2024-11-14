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
                    let mut group_str = String::new();
                    let (parsed_group, group_status_code) = parse(token_iter, true);
                    for parsed_line in parsed_group {
                        group_str = format!("{}{}", group_str, parsed_line);
                    }

                    if group_status_code != 0 {
                        status_code = group_status_code;
                    }

                    group_str = format!("(group {})", group_str);
                    parsed_output.push(group_str);
                }
                TokenType::RIGHT_PAREN => {
                    if is_group {
                        return (parsed_output, status_code);
                    } else {
                        status_code = 65;
                    }
                }
                TokenType::MINUS | TokenType::BANG => {
                    let negated_str = format!("({} ", token.lexeme);
                    let needs_literal = token.token_type == TokenType::MINUS;
                    token_op = token_iter.next();
                    match token_op {
                        Some(neg_token) => {
                            if needs_literal {
                                parsed_output
                                    .push(format!("{}{})", negated_str, neg_token.literal));
                            } else {
                                parsed_output.push(format!("{}{})", negated_str, neg_token.lexeme));
                            }
                        }
                        None => status_code = 65,
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
