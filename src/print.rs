use crate::expr::{Binary, Grouping, Literal, Unary};
use crate::token::{Token, TokenType};
use std::fmt;

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            // Single-character tokens.
            Self::LeftParen => write!(f, "("),
            Self::RightParen => write!(f, ")"),
            Self::LeftBrace => write!(f, "{{"),
            Self::RightBrace => write!(f, "}}"),
            Self::Comma => write!(f, ","),
            Self::Dot => write!(f, "."),
            Self::Semicolon => write!(f, ";"),
            Self::Minus => write!(f, "-"),
            Self::Plus => write!(f, "+"),
            Self::Slash => write!(f, "/"),
            Self::Star => write!(f, "*"),

            // One or two character tokens.
            Self::Bang => write!(f, "!"),
            Self::BangEqual => write!(f, "!="),
            Self::Equal => write!(f, "="),
            Self::EqualEqual => write!(f, "=="),
            Self::Greater => write!(f, ">"),
            Self::GreaterEqual => write!(f, ">="),
            Self::Less => write!(f, "<"),
            Self::LessEqual => write!(f, "<="),

            // Literals.
            Self::Identifier(s) => write!(f, "{}", s),
            Self::String(s) => write!(f, "{}", s),
            Self::Number(n) => write!(f, "{}", n.to_string()),

            // Keywords.
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Class => write!(f, "class"),
            Self::Super => write!(f, "super"),
            Self::This => write!(f, "this"),
            Self::Var => write!(f, "var"),
            Self::Fun => write!(f, "fun"),
            Self::Return => write!(f, "return"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::While => write!(f, "while"),
            Self::For => write!(f, "for"),
            Self::Print => write!(f, "print"),

            Self::EOF => write!(f, "EOF"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.lexeme)
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({} {} {})", self.op, self.lhs, self.rhs)
    }
}

impl fmt::Display for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "(group {})", self.expr)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.expr)
    }
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({} {})", self.op, self.rhs)
    }
}

#[test]
fn test_print() {
    let expr = Binary::new(
        Unary::boxed(
            Token::new(TokenType::Minus, "-".into(), 1),
            Literal::boxed(Token::new(TokenType::Number(123.0), "123".into(), 1)),
        ),
        Token::new(TokenType::Star, "*".into(), 1),
        Grouping::boxed(Literal::boxed(Token::new(
            TokenType::Number(45.67),
            "45.67".into(),
            1,
        ))),
    );

    assert_eq!(expr.to_string(), "(* (- 123) (group 45.67))");
}
