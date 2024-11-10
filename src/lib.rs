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

    EQUAL,
    EQUAL_EQUAL,
    BANG_EQUAL,

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
    let mut prev_lexeme: char = ' ';
    for (i, line) in file_contents.lines().enumerate() {
        for c in line.chars() {
            match get_token(&c, &prev_lexeme) {
                Ok(token) => {
                    if token.lexeme == "==" || token.lexeme == "!=" {
                        tokens.pop();
                        tokens.push(token);
                        prev_lexeme = ' ';
                        continue;
                    }
                    tokens.push(token);
                    prev_lexeme = c;
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
        '=' => {
            if *prev_lexeme == '!' {
                return Ok(Token::new(
                    TokenType::BANG_EQUAL,
                    String::from("!="),
                    String::from("null"),
                ));
            }
            if *prev_lexeme == '=' {
                return Ok(Token::new(
                    TokenType::EQUAL_EQUAL,
                    String::from("=="),
                    String::from("null"),
                ));
            }
            return Ok(Token::new(
                TokenType::EQUAL,
                String::from('='),
                String::from("null"),
            ));
        }
        _ => return Err("invalid token".into()),
    };
}
