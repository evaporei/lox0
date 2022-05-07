use std::fmt;

#[derive(Clone, Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Minus,
    Plus,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
    And,
    Or,
    True,
    False,
    Class,
    Super,
    This,
    Var,
    Return,
    If,
    Else,
    While,
    For,
    Print,

    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let t = match self {
            // Single-character tokens.
            Self::LeftParen => "(",
            Self::RightParen => ")",
            Self::LeftBrace => "{",
            Self::RightBrace => "}",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::Semicolon => ";",
            Self::Minus => "-",
            Self::Plus => "+",
            Self::Slash => "/",
            Self::Star => "*",

            // One or two character tokens.
            Self::Bang => "!",
            Self::BangEqual => "!=",
            Self::Equal => "=",
            Self::EqualEqual => "==",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
            Self::Less => "<",
            Self::LessEqual => "<=",

            // Literals.
            Self::Identifier(s) => s,
            Self::String(s) => s,
            Self::Number(n) => {
                write!(f, "{}", n.to_string())?;
                // FIXME: workaround to avoid let binding of n.to_string()
                // "temporary value..."
                ""
            }

            // Keywords.
            Self::And => "and",
            Self::Or => "or",
            Self::True => "true",
            Self::False => "false",
            Self::Class => "class",
            Self::Super => "super",
            Self::This => "this",
            Self::Var => "var",
            Self::Return => "return",
            Self::If => "if",
            Self::Else => "else",
            Self::While => "while",
            Self::For => "for",
            Self::Print => "print",

            Self::EOF => "EOF",
        };

        write!(f, "{t}")
    }
}

#[derive(Debug)]
pub struct Token {
    ty: TokenType,
    lexeme: String,
    #[allow(dead_code)]
    line: usize,
}

impl Token {
    pub fn new(ty: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            ty,
            lexeme,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} {}", self.ty, self.lexeme)
    }
}
