use std::error::Error;
use std::fmt;
mod tests;

#[derive(Debug, PartialEq)]
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

    IDENTIFIER,

    EOF,
    INVALID,
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

    fn get_token(lexeme: char, prev_lexeme: char) -> Token {
        let mut token = Token::new(
            TokenType::INVALID,
            String::from(lexeme),
            String::from("null"),
        );
        match lexeme {
            '/' => {
                token.token_type = TokenType::SLASH;
            }
            '(' => {
                token.token_type = TokenType::LEFT_PAREN;
            }
            ')' => {
                token.token_type = TokenType::RIGHT_PAREN;
            }
            '{' => {
                token.token_type = TokenType::LEFT_BRACE;
            }
            '}' => {
                token.token_type = TokenType::RIGHT_BRACE;
            }
            '*' => {
                token.token_type = TokenType::STAR;
            }
            '.' => {
                token.token_type = TokenType::DOT;
            }
            ',' => {
                token.token_type = TokenType::COMMA;
            }
            '+' => {
                token.token_type = TokenType::PLUS;
            }
            '-' => {
                token.token_type = TokenType::MINUS;
            }
            ';' => {
                token.token_type = TokenType::SEMICOLON;
            }
            '!' => {
                token.token_type = TokenType::BANG;
            }
            '<' => {
                token.token_type = TokenType::LESS;
            }
            '>' => {
                token.token_type = TokenType::GREATER;
            }
            '=' => match prev_lexeme {
                '!' => {
                    token.token_type = TokenType::BANG_EQUAL;
                    token.lexeme = String::from("!=");
                }
                '=' => {
                    token.token_type = TokenType::EQUAL_EQUAL;
                    token.lexeme = String::from("==");
                }
                '>' => {
                    token.token_type = TokenType::GREATER_EQUAL;
                    token.lexeme = String::from(">=");
                }
                '<' => {
                    token.token_type = TokenType::LESS_EQUAL;
                    token.lexeme = String::from("<=");
                }
                _ => {
                    token.token_type = TokenType::EQUAL;
                }
            },
            '@' | '#' | '&' | '$' | '%' | '^' | '?' => return token,
            //make token_type invalid if it is not a valid alphabet / '_'
            _ => token.token_type = TokenType::IDENTIFIER,
        };

        return token;
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

    (
        ch,
        Token::new(TokenType::IDENTIFIER, identifier, String::from("null")),
    )
}
