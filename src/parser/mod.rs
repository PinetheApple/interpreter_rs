use codecrafters_interpreter::{Token, TokenType};

pub fn parse(tokens: Vec<Token>) -> (Vec<String>, i32) {
    let mut status_code = 0;
    let mut parsed_output: Vec<String> = vec![];

    for token in tokens {
        match token.token_type {
            TokenType::EOF => {}
            TokenType::TRUE | TokenType::FALSE | TokenType::NIL => parsed_output.push(token.lexeme),
            TokenType::NUMBER => parsed_output.push(token.literal),
            _ => {
                status_code = 65;
            }
        }
    }

    (parsed_output, status_code)
}
