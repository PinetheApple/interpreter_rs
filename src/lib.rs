use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line_num: u32,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line_num: u32) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line_num,
        }
    }

    pub fn get_token(lexeme: char, prev_lexeme: char, line_num: u32) -> Self {
        let mut token = Token::new(
            TokenType::INVALID,
            String::from(lexeme),
            String::from("null"),
            line_num,
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

    pub fn print(&self) {
        match self.token_type {
            TokenType::STRING => println!("{}", self.literal),
            _ => println!("{}", self.lexeme),
        }
    }
}

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
    Unary(Token, Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Stmt(Statement),
    Scope(Vec<Expr>),
}

pub enum Statement {
    PrintStmt(Box<Expr>),
    DeclarationStmt(Token, Option<Box<Expr>>),
    AssignmentStmt(Token, Box<Expr>),
    IfStmt(Vec<Conditional>),
    ForStmt(Conditional),
    WhileStmt(Conditional),
}

pub struct Conditional(pub Box<Expr>, pub Box<Expr>);

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Unary(operator, val) => {
                write!(f, "({} {})", operator.lexeme, val)
            }
            Expr::Literal(token) => match token.token_type {
                TokenType::STRING | TokenType::NUMBER => write!(f, "{}", token.literal),
                _ => write!(f, "{}", token.lexeme),
            },
            Expr::Binary(left_val, operator, right_val) => {
                write!(f, "({} {} {})", operator.lexeme, left_val, right_val)
            }
            Expr::Logical(left_val, operator, right_val) => {
                write!(f, "{} {} {}", left_val, operator.lexeme, right_val)
            }
            Expr::Grouping(expression) => {
                write!(f, "(group {})", expression)
            }
            Expr::Stmt(statement) => write!(f, "{}", statement),
            Expr::Scope(exprs) => {
                write!(f, "scoped \n{{\n")?;
                for expr in exprs {
                    write!(f, "{}\n", expr)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::PrintStmt(expr) => write!(f, "print {}", expr),
            Statement::DeclarationStmt(variable, value) => match value {
                Some(val_expr) => write!(f, "declare {} = {}", variable.lexeme, val_expr),
                None => write!(f, "declare {} = nil", variable.lexeme),
            },
            Statement::AssignmentStmt(variable, value) => {
                write!(f, "assign {} with {}", variable.lexeme, value)
            }
            Statement::IfStmt(conditionals) => {
                write!(f, "if {}", conditionals[0])?;
                let blocks = conditionals.len();
                if blocks > 1 {
                    for i in 1..blocks {
                        match *conditionals[i].0 {
                            Expr::Literal(Token {
                                token_type: TokenType::TRUE,
                                ..
                            }) => {
                                write!(f, "\nelse\nstatement(s):\n{}", conditionals[i].1)?;
                                break;
                            }
                            _ => write!(f, "\nelse if {}", conditionals[i])?,
                        }
                    }
                }

                write!(f, "\nend if")
            }
            Statement::WhileStmt(conditional) => write!(f, "while {}", conditional),
            Statement::ForStmt(conditional) => write!(f, "for loop: \n{}", conditional),
        }
    }
}

impl fmt::Display for Conditional {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "condition: {}\nstatement(s):\n{}\nend block",
            self.0, self.1
        )
    }
}
