use std::fmt;

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TokenType {
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

    // reserved words
    CLASS,
    SUPER,
    THIS,
    FUN,
    RETURN,
    VAR,
    TRUE,
    FALSE,
    PRINT,
    IF,
    ELSE,
    FOR,
    WHILE,
    AND,
    OR,
    NIL,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
        }
    }

    pub fn get_token(lexeme: char, prev_lexeme: char) -> Token {
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
            _ => {
                if lexeme.is_ascii_alphanumeric() || lexeme == '_' {
                    token.token_type = TokenType::IDENTIFIER
                }
            }
        };

        return token;
    }

    pub fn check_if_reserved(&mut self) {
        match self.lexeme.as_str() {
            "class" => self.token_type = TokenType::CLASS,
            "super" => self.token_type = TokenType::SUPER,
            "this" => self.token_type = TokenType::THIS,
            "fun" => self.token_type = TokenType::FUN,
            "return" => self.token_type = TokenType::RETURN,
            "print" => self.token_type = TokenType::PRINT,
            "var" => self.token_type = TokenType::VAR,
            "true" => self.token_type = TokenType::TRUE,
            "false" => self.token_type = TokenType::FALSE,
            "and" => self.token_type = TokenType::AND,
            "or" => self.token_type = TokenType::OR,
            "if" => self.token_type = TokenType::IF,
            "else" => self.token_type = TokenType::ELSE,
            "for" => self.token_type = TokenType::FOR,
            "while" => self.token_type = TokenType::WHILE,
            "nil" => self.token_type = TokenType::NIL,
            _ => {}
        };
    }
}

#[derive(Debug)]
pub enum ExprType {
    BINARY,
    GROUPING,
    LITERAL,
    UNARY,
}

pub struct Expr {
    pub expr_type: ExprType,
    pub expr_str: String,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.expr_str)
    }
}
