use codecrafters_interpreter::{Token, TokenType};

pub fn tokenize(file_contents: String) -> (Vec<Token>, i32) {
    let mut status_code: i32 = 0;
    let mut tokens: Vec<Token> = vec![];
    let mut line_number = 1;
    let mut prev_lexeme = ' ';
    let mut char_iter = file_contents.chars();
    let mut c = char_iter.next();

    loop {
        match c {
            None => break,
            Some('\t') | Some(' ') => c = char_iter.next(),
            Some('\n') => {
                c = char_iter.next();
                line_number += 1;
                prev_lexeme = ' ';
            }
            Some('"') => {
                if let Ok(token) = get_string_literal(line_number, &mut char_iter) {
                    // for string
                    tokens.push(token);
                } else {
                    status_code = 65;
                }

                c = char_iter.next();
            }
            Some(ch) => {
                if ch.is_ascii_digit() {
                    // for numbers
                    let (ch, token) = get_numeric_literal(ch, &mut char_iter, line_number);
                    tokens.push(token);
                    c = Some(ch);
                    prev_lexeme = ch;
                    continue;
                }

                let token = Token::get_token(ch, prev_lexeme, line_number);
                match token.token_type {
                    TokenType::INVALID => {
                        status_code = 65;
                        eprintln!(
                            "[line {line_number}] Error: Unexpected character: {}",
                            token.lexeme
                        );
                        prev_lexeme = ' ';
                        c = char_iter.next();
                        continue;
                    }
                    TokenType::IDENTIFIER => {
                        let (ch, identifier_token) =
                            get_identifier(ch, &mut char_iter, line_number);
                        tokens.push(identifier_token);
                        c = Some(ch);
                        continue;
                    }
                    TokenType::SLASH => {
                        if prev_lexeme == '/' {
                            // for single-line comments
                            tokens.pop();
                            loop {
                                c = char_iter.next();
                                if c == Some('\n') || c == None {
                                    prev_lexeme = ' ';
                                    break;
                                }
                            }
                        } else {
                            prev_lexeme = ch;
                            tokens.push(token);
                            c = char_iter.next();
                        }
                    }
                    _ => {
                        match token.lexeme.as_str() {
                            "==" | "!=" | ">=" | "<=" => {
                                // for comparisons
                                tokens.pop();
                                prev_lexeme = ' ';
                            }
                            _ => {
                                prev_lexeme = ch;
                            }
                        };

                        tokens.push(token);
                        c = char_iter.next();
                    }
                }
            }
        }
    }

    tokens.push(Token::new(
        TokenType::EOF,
        String::from(""),
        String::from("null"),
        0,
    ));

    (tokens, status_code)
}

fn get_string_literal<I>(line_number: u32, char_iter: &mut I) -> Result<Token, ()>
where
    I: Iterator<Item = char>,
{
    let mut string_literal = String::new();
    let mut c = char_iter.next();
    loop {
        match c {
            None => {
                eprintln!("[line {}] Error: Unterminated string.", line_number);
                return Err(());
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
        line_number,
    ));
}

fn get_numeric_literal<I>(first_digit: char, char_iter: &mut I, line_number: u32) -> (char, Token)
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

    numeric_val = numeric_val.parse::<f32>().unwrap().to_string();
    let mut literal_val = numeric_val.parse::<f32>().unwrap().to_string();

    match literal_val.parse::<i32>() {
        Ok(_) => literal_val = format!("{}.0", literal_val),
        _ => {}
    }

    (
        ch,
        Token::new(TokenType::NUMBER, numeric_val, literal_val, line_number),
    )
}

fn get_identifier<I>(first_char: char, char_iter: &mut I, line_number: u32) -> (char, Token)
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
                let token = Token::get_token(val, ' ', line_number);
                if token.token_type != TokenType::IDENTIFIER {
                    ch = val;
                    break;
                }
                identifier = format!("{}{}", identifier, val);
            }
        }

        c = char_iter.next();
    }
    let mut token = Token::new(
        TokenType::IDENTIFIER,
        identifier,
        String::from("null"),
        line_number,
    );
    token.check_if_reserved();

    (ch, token)
}
