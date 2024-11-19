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
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(Token),
    Unary(UnaryExpr),
    PrintStatement(Box<Expr>),
    DeclarationStatment(VarDefinition),
}

pub struct UnaryExpr {
    pub operator: Token,
    pub val: Box<Expr>,
}

pub struct BinaryExpr {
    pub left_val: Box<Expr>,
    pub operator: Token,
    pub right_val: Box<Expr>,
}

pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

pub struct VarDefinition {
    pub variable: Token,
    pub value: Option<Box<Expr>>,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Unary(expr) => {
                write!(f, "({} {})", expr.operator.lexeme, expr.val)
            }
            Expr::Literal(token) => match token.token_type {
                TokenType::STRING | TokenType::NUMBER => write!(f, "{}", token.literal),
                _ => write!(f, "{}", token.lexeme),
            },
            Expr::Binary(expr) => {
                write!(
                    f,
                    "({} {} {})",
                    expr.operator.lexeme, expr.left_val, expr.right_val
                )
            }
            Expr::Grouping(expr) => {
                write!(f, "(group {})", expr.expression)
            }
            Expr::PrintStatement(expr) => write!(f, "print {}", expr),
            Expr::DeclarationStatment(expr) => match &expr.value {
                Some(val_expr) => write!(f, "declare {} = {}", expr.variable.lexeme, val_expr),
                None => write!(f, "declare {} = nil", expr.variable.lexeme),
            },
        }
    }
}

impl UnaryExpr {
    pub fn new(operator: Token, val: Expr) -> Self {
        UnaryExpr {
            operator,
            val: Box::new(val),
        }
    }
}

impl BinaryExpr {
    pub fn new(left_val: Expr, operator: Token, right_val: Expr) -> Self {
        BinaryExpr {
            left_val: Box::new(left_val),
            operator,
            right_val: Box::new(right_val),
        }
    }
}

impl GroupingExpr {
    pub fn new(expression: Expr) -> Self {
        GroupingExpr {
            expression: Box::new(expression),
        }
    }
}

impl VarDefinition {
    pub fn new(variable: Token, value: Option<Expr>) -> Self {
        match value {
            None => VarDefinition {
                variable,
                value: None,
            },
            Some(expr) => VarDefinition {
                variable,
                value: Some(Box::new(expr)),
            },
        }
    }
}
