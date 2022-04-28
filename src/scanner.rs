use crate::error::error;
use crate::token::{Token, TokenType};
use std::any::Any;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

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
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen, Box::new(())),
            ')' => self.add_token(TokenType::RightParen, Box::new(())),
            '{' => self.add_token(TokenType::LeftBrace, Box::new(())),
            '}' => self.add_token(TokenType::RightBrace, Box::new(())),
            ',' => self.add_token(TokenType::Comma, Box::new(())),
            '.' => self.add_token(TokenType::Dot, Box::new(())),
            '-' => self.add_token(TokenType::Minus, Box::new(())),
            '+' => self.add_token(TokenType::Plus, Box::new(())),
            ';' => self.add_token(TokenType::Semicolon, Box::new(())),
            '*' => self.add_token(TokenType::Star, Box::new(())),
            _ => error(self.line, "Unexpected character."),
        }
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
}
