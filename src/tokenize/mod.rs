use std::error::Error;
use std::fmt;

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,

    STAR,
    DOT,
    COMMA,
    SEMICOLON,
    PLUS,
    MINUS,
    BANG,

    SLASH,

    EQUAL,
    EQUAL_EQUAL,
    BANG_EQUAL,
    LESS,
    LESS_EQUAL,
    GREATER,
    GREATER_EQUAL,

    STRING,
    NUMBER,

    EOF,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: String) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
        }
    }

    fn get_token(lexeme: char, prev_lexeme: char) -> Result<Token, Box<dyn Error>> {
        match lexeme {
            '/' => {
                return Ok(Token::new(
                    TokenType::SLASH,
                    String::from('/'),
                    String::from("null"),
                ))
            }
            '(' => {
                return Ok(Token::new(
                    TokenType::LEFT_PAREN,
                    String::from('('),
                    String::from("null"),
                ))
            }
            ')' => {
                return Ok(Token::new(
                    TokenType::RIGHT_PAREN,
                    String::from(')'),
                    String::from("null"),
                ))
            }
            '{' => {
                return Ok(Token::new(
                    TokenType::LEFT_BRACE,
                    String::from('{'),
                    String::from("null"),
                ))
            }
            '}' => {
                return Ok(Token::new(
                    TokenType::RIGHT_BRACE,
                    String::from('}'),
                    String::from("null"),
                ))
            }
            '*' => {
                return Ok(Token::new(
                    TokenType::STAR,
                    String::from('*'),
                    String::from("null"),
                ))
            }
            '.' => {
                return Ok(Token::new(
                    TokenType::DOT,
                    String::from('.'),
                    String::from("null"),
                ))
            }
            ',' => {
                return Ok(Token::new(
                    TokenType::COMMA,
                    String::from(','),
                    String::from("null"),
                ))
            }
            '+' => {
                return Ok(Token::new(
                    TokenType::PLUS,
                    String::from('+'),
                    String::from("null"),
                ))
            }
            '-' => {
                return Ok(Token::new(
                    TokenType::MINUS,
                    String::from('-'),
                    String::from("null"),
                ))
            }
            ';' => {
                return Ok(Token::new(
                    TokenType::SEMICOLON,
                    String::from(';'),
                    String::from("null"),
                ))
            }
            '!' => {
                return Ok(Token::new(
                    TokenType::BANG,
                    String::from('!'),
                    String::from("null"),
                ))
            }
            '<' => {
                return Ok(Token::new(
                    TokenType::LESS,
                    String::from('<'),
                    String::from("null"),
                ))
            }
            '>' => {
                return Ok(Token::new(
                    TokenType::GREATER,
                    String::from('>'),
                    String::from("null"),
                ))
            }
            '=' => match prev_lexeme {
                '!' => {
                    return Ok(Token::new(
                        TokenType::BANG_EQUAL,
                        String::from("!="),
                        String::from("null"),
                    ));
                }
                '=' => {
                    return Ok(Token::new(
                        TokenType::EQUAL_EQUAL,
                        String::from("=="),
                        String::from("null"),
                    ));
                }
                '>' => {
                    return Ok(Token::new(
                        TokenType::GREATER_EQUAL,
                        String::from(">="),
                        String::from("null"),
                    ));
                }
                '<' => {
                    return Ok(Token::new(
                        TokenType::LESS_EQUAL,
                        String::from("<="),
                        String::from("null"),
                    ));
                }
                _ => {
                    return Ok(Token::new(
                        TokenType::EQUAL,
                        String::from("="),
                        String::from("null"),
                    ));
                }
            },
            _ => return Err("invalid token".into()),
        };
    }
}

pub fn tokenize(file_contents: String) -> i32 {
    let mut status_code: i32 = 0;
    let mut tokens: Vec<Token> = vec![];
    for (i, line) in file_contents.lines().enumerate() {
        let (line_tokens, line_status_code) = tokenize_line(i + 1, line);
        tokens.extend(line_tokens);
        if line_status_code == 65 {
            status_code = 65;
        }
    }
    tokens.push(Token::new(
        TokenType::EOF,
        String::from(""),
        String::from("null"),
    ));

    for token in tokens {
        println!("{}", token);
    }
    status_code
}

fn tokenize_line(line_number: usize, line: &str) -> (Vec<Token>, i32) {
    let mut prev_lexeme = ' ';
    let mut line_status_code = 0;
    let mut tokens: Vec<Token> = vec![];

    let mut char_iter = line.chars();

    loop {
        let c = char_iter.next();
        match c {
            None => break,
            Some('\t') | Some(' ') => continue,
            Some('"') => {
                if let Ok(token) = get_string_literal(line_number, &mut char_iter, line) {
                    tokens.push(token);
                } else {
                    line_status_code = 65;
                }
            }
            Some(ch) => {
                if ch.is_ascii_digit() {
                    let token = get_numeric_literal(ch, &mut char_iter, line);
                    tokens.push(token);
                    continue;
                }
                match Token::get_token(ch, prev_lexeme) {
                    Ok(token) => {
                        if token.lexeme == "/" && prev_lexeme == '/' {
                            tokens.pop();
                            break;
                        }
                        if token.lexeme == "=="
                            || token.lexeme == "!="
                            || token.lexeme == ">="
                            || token.lexeme == "<="
                        {
                            tokens.pop();
                            prev_lexeme = ' ';
                        } else {
                            prev_lexeme = ch;
                        }
                        tokens.push(token);
                    }
                    Err(_) => {
                        prev_lexeme = ' ';
                        line_status_code = 65;
                        eprintln!("[line {}] Error: Unexpected character: {}", line_number, ch);
                    }
                };
            }
        }
    }

    (tokens, line_status_code)
}

fn get_string_literal<I>(
    line_number: usize,
    char_iter: &mut I,
    line: &str,
) -> Result<Token, Box<dyn Error>>
where
    I: Iterator<Item = char>,
{
    let mut curr_index = 1;
    let mut c = char_iter.next();
    loop {
        if c == None {
            eprintln!("[line {}] Error: Unterminated string.", line_number);
            return Err("Unterminated string".into());
        }

        if c == Some('"') {
            break;
        }
        c = char_iter.next();
        curr_index += 1;
    }

    let string_literal = &line[..curr_index];
    return Ok(Token::new(
        TokenType::STRING,
        format!("\"{}\"", string_literal),
        string_literal.to_string(),
    ));
}

fn get_numeric_literal<I>(first_digit: char, char_iter: &mut I, line: &str) -> Token
where
    I: Iterator<Item = char>,
{
    let mut curr_index = 1;
    let mut c = char_iter.next();
    loop {
        match c {
            None | Some(' ') => break,
            Some('.') => {}
            Some(ch) => {
                if !ch.is_ascii_digit() {
                    break;
                }
            }
        }
        c = char_iter.next();
        curr_index += 1;
    }

    let numeric_val = format!("{}{}", first_digit, &line[1..curr_index]);
    let mut literal_val = numeric_val.parse::<f32>().unwrap().to_string();

    match literal_val.parse::<i32>() {
        Ok(_) => literal_val = format!("{}.0", literal_val),
        _ => {}
    }

    return Token::new(TokenType::NUMBER, numeric_val, literal_val);
}
