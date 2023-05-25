#[derive(Clone, Debug, PartialEq)]
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
    Bool(bool),
    Nil,

    // Keywords.
    And,
    Or,
    True,
    False,
    Class,
    Super,
    This,
    Var,
    Fun,
    Return,
    If,
    Else,
    While,
    For,
    Print,

    EOF,
}

impl TokenType {
    pub fn is_literal(&self) -> bool {
        matches!(self, TokenType::String(_) | TokenType::Number(_))
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    #[allow(dead_code)]
    line: usize,
}

impl Token {
    pub fn new(ty: TokenType, lexeme: String, line: usize) -> Self {
        Self { ty, lexeme, line }
    }
}
