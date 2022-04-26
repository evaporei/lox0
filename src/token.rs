use std::any::Any;
use std::fmt;

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
    Identifier,
    String,
    Number,

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
            Self::Identifier | Self::String | Self::Number => "tbd",

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

            Self::EOF => "tbd",
        };

        write!(f, "{t}")
    }
}

pub struct Token {
    ty: TokenType,
    lexeme: String,
    literal: Box<dyn Any>,
    line: usize,
}

impl Token {
    pub fn new(ty: TokenType, lexeme: String, literal: Box<dyn Any>, line: usize) -> Self {
        Self {
            ty,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // write!(f, "{} {} {}", self.ty, self.lexeme, self.literal)
        write!(f, "{} {} any", self.ty, self.lexeme)
    }
}
