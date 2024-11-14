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
                TokenType::EOF => {}
                TokenType::TRUE | TokenType::FALSE | TokenType::NIL => {
                    parsed_output.push(token.lexeme.clone())
                }
                TokenType::NUMBER | TokenType::STRING => parsed_output.push(token.literal.clone()),
                TokenType::LEFT_BRACE => {
                    let (_, group_status_code) = parse(token_iter, true);
                    if group_status_code != 0 {
                        status_code = group_status_code;
                    }
                }
                TokenType::RIGHT_BRACE => {
                    if is_group {
                        return (parsed_output, status_code);
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
