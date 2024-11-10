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
}

pub fn tokenize(file_contents: String) -> i32 {
    let mut status_code: i32 = 0;
    let mut tokens: Vec<Token> = vec![];
    for (i, line) in file_contents.lines().enumerate() {
        let mut prev_lexeme: char = ' ';
        for c in line.chars() {
            match get_token(&c, &prev_lexeme) {
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
                        prev_lexeme = c;
                    }
                    tokens.push(token);
                }
                Err(_) => {
                    prev_lexeme = ' ';
                    status_code = 65;
                    eprintln!("[line {}] Error: Unexpected character: {}", i + 1, c)
                }
            }
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

fn get_token(lexeme: &char, prev_lexeme: &char) -> Result<Token, Box<dyn Error>> {
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
        '=' => match *prev_lexeme {
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
