use codecrafters_interpreter::{Token, TokenType};
use std::error::Error;
mod tests;

pub fn tokenize(file_contents: String) -> (Vec<Token>, i32) {
    let mut status_code: i32 = 0;
    let mut tokens: Vec<Token> = vec![];
    for (i, line) in file_contents.lines().enumerate() {
        let (line_tokens, line_status_code) = tokenize_line(i + 1, line);
        tokens.extend(line_tokens);
        if line_status_code != 0 {
            status_code = line_status_code;
        }
    }

    tokens.push(Token::new(
        TokenType::EOF,
        String::from(""),
        String::from("null"),
    ));

    (tokens, status_code)
}

fn tokenize_line(line_number: usize, line: &str) -> (Vec<Token>, i32) {
    let mut prev_lexeme = ' ';
    let mut line_status_code = 0;
    let mut tokens: Vec<Token> = vec![];

    let mut char_iter = line.chars();
    let mut c = char_iter.next();

    loop {
        match c {
            None => break,
            Some('\t') | Some(' ') => c = char_iter.next(),
            Some('"') => {
                if let Ok(token) = get_string_literal(line_number, &mut char_iter) {
                    tokens.push(token);
                } else {
                    line_status_code = 65;
                }

                c = char_iter.next();
            }
            Some(ch) => {
                if ch.is_ascii_digit() {
                    let (ch, token) = get_numeric_literal(ch, &mut char_iter);
                    tokens.push(token);
                    c = Some(ch);
                    continue;
                }

                let token = Token::get_token(ch, prev_lexeme);
                match token.token_type {
                    TokenType::INVALID => {
                        line_status_code = 65;
                        eprintln!(
                            "[line {line_number}] Error: Unexpected character: {}",
                            token.lexeme
                        );
                        prev_lexeme = ' ';
                        c = char_iter.next();
                        continue;
                    }
                    TokenType::IDENTIFIER => {
                        let (ch, identifier_token) = get_identifier(ch, &mut char_iter);
                        tokens.push(identifier_token);
                        c = Some(ch);
                        continue;
                    }
                    _ => {
                        match token.lexeme.as_str() {
                            "/" => {
                                if prev_lexeme == '/' {
                                    tokens.pop();
                                    break;
                                }
                                prev_lexeme = ch;
                            }
                            "==" | "!=" | ">=" | "<=" => {
                                tokens.pop();
                                prev_lexeme = ' ';
                            }
                            _ => prev_lexeme = ch,
                        };
                        tokens.push(token);

                        c = char_iter.next();
                    }
                }
            }
        }
    }

    (tokens, line_status_code)
}

fn get_string_literal<I>(line_number: usize, char_iter: &mut I) -> Result<Token, Box<dyn Error>>
where
    I: Iterator<Item = char>,
{
    let mut string_literal = String::new();
    let mut c = char_iter.next();
    loop {
        match c {
            None => {
                eprintln!("[line {}] Error: Unterminated string.", line_number);
                return Err("Unterminated string".into());
            }
            Some('"') => break,
            Some(ch) => {
                string_literal = format!("{}{}", string_literal, ch);
            }
        }

        c = char_iter.next();
    }

    return Ok(Token::new(
        TokenType::STRING,
        format!("\"{}\"", string_literal),
        string_literal.to_string(),
    ));
}

fn get_numeric_literal<I>(first_digit: char, char_iter: &mut I) -> (char, Token)
where
    I: Iterator<Item = char>,
{
    let mut c = char_iter.next();
    let mut numeric_val = String::from(first_digit);
    let mut ch = ' ';
    loop {
        match c {
            None | Some(' ') => break,
            Some('.') => {
                numeric_val = format!("{}.", numeric_val);
            }
            Some(val) => {
                if !val.is_ascii_digit() {
                    ch = val;
                    break;
                }

                numeric_val = format!("{}{}", numeric_val, val);
            }
        }

        c = char_iter.next();
    }

    let mut literal_val = numeric_val.parse::<f32>().unwrap().to_string();

    match literal_val.parse::<i32>() {
        Ok(_) => literal_val = format!("{}.0", literal_val),
        _ => {}
    }

    (ch, Token::new(TokenType::NUMBER, numeric_val, literal_val))
}

fn get_identifier<I>(first_char: char, char_iter: &mut I) -> (char, Token)
where
    I: Iterator<Item = char>,
{
    let mut c = char_iter.next();
    let mut ch = ' ';
    let mut identifier = String::from(first_char);
    loop {
        match c {
            None | Some(' ') => break,
            Some(val) => {
                let token = Token::get_token(val, ' ');
                if token.token_type != TokenType::IDENTIFIER {
                    ch = val;
                    break;
                }
                identifier = format!("{}{}", identifier, val);
            }
        }

        c = char_iter.next();
    }
    let mut token = Token::new(TokenType::IDENTIFIER, identifier, String::from("null"));
    token.check_if_reserved();

    (ch, token)
}
