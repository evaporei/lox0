use crate::error;
use crate::token::{Token, TokenType};
use std::any::Any;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

use std::collections::HashMap;

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    // FIXME: make const (I don't want to use lazy_static ;-;)
    fn keywords(&self) -> HashMap<&'a str, TokenType> {
        let mut k = HashMap::new();
        k.insert("and", TokenType::And);
        k.insert("or", TokenType::Or);
        k.insert("true", TokenType::True);
        k.insert("false", TokenType::False);
        k.insert("class", TokenType::Class);
        k.insert("super", TokenType::Super);
        k.insert("this", TokenType::This);
        k.insert("var", TokenType::Var);
        k.insert("return", TokenType::Return);
        k.insert("if", TokenType::If);
        k.insert("else", TokenType::Else);
        k.insert("while", TokenType::While);
        k.insert("for", TokenType::For);
        k.insert("print", TokenType::Print);
        k
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".into(),
            Box::new(()),
            self.line,
        ));

        &self.tokens
    }

    fn scan_token(&mut self) {
        let ty = match self.advance() {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            '!' => {
                if self.match_('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            '=' => {
                if self.match_('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '<' => {
                if self.match_('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '>' => {
                if self.match_('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            '/' => {
                if self.match_('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    return;
                } else {
                    TokenType::Slash
                }
            }
            // Ignore whitespace
            ' ' | '\r' | '\t' => {
                return;
            }
            '\n' => {
                self.line += 1;
                return;
            }
            '"' => TokenType::String(self.string()),
            c => {
                if self.is_digit(c) {
                    TokenType::Number(self.number())
                } else if self.is_alpha(c) {
                    self.identifier()
                } else {
                    self.error("Unexpected character.")
                }
            }
        };

        self.add_token(ty, Box::new(()));
    }

    fn identifier(&mut self) -> TokenType {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        match self.keywords().get(text) {
            Some(ty) => ty.clone(),
            None => TokenType::Identifier,
        }
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) -> f64 {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        self.source[self.start..self.current].parse().unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn string(&mut self) -> String {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            self.error("Unterminated string.");
        }

        self.advance();

        (&self.source[self.start + 1..self.current - 1]).into()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().next().unwrap()
        }
    }

    fn match_(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().next().unwrap() != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().next().unwrap()
    }

    fn add_token(&mut self, ty: TokenType, literal: Box<dyn Any>) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(ty, text.into(), literal, self.line));
    }

    fn error(&self, msg: &str) -> ! {
        error::error(self.line, msg)
    }
}
