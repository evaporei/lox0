use crate::expr::{Binary, BoxExpr, Grouping, Literal, Unary};
use crate::token::{Token, TokenType};

/// Simplified grammar:
///
/// expression     → literal
///                | unary
///                | binary
///                | grouping ;
///
/// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
/// grouping       → "(" expression ")" ;
/// unary          → ( "-" | "!" ) expression ;
/// binary         → expression operator expression ;
/// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
///                | "+"  | "-"  | "*" | "/" ;
///
/// "Strict"/complete grammar:
///
/// expression     → equality ;
/// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
/// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
/// term           → factor ( ( "-" | "+" ) factor )* ;
/// factor         → unary ( ( "/" | "*" ) unary )* ;
/// unary          → ( "!" | "-" ) unary
///                | primary ;
/// primary        → "(" expression ")"
///                | literal ;
/// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
///
///
/// expression     → unary ;
/// unary          → ( "!" | "-" ) unary
///                | primary ;
/// primary        → "(" expression ")"
///                | literal ;
/// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
///
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> BoxExpr {
        self.equality()
    }

    fn equality(&mut self) -> BoxExpr {
        let mut expr = self.comparison();

        while self.match_(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().unwrap().ty.clone();
            let right = self.comparison();
            expr = Binary::boxed(expr, operator, right);
        }

        expr
    }

    fn comparison(&mut self) -> BoxExpr {
        let mut expr = self.term();

        while self.match_(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().unwrap().ty.clone();
            let right = self.term();
            expr = Binary::boxed(expr, operator, right);
        }

        expr
    }

    fn term(&mut self) -> BoxExpr {
        let mut expr = self.factor();

        while self.match_(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().unwrap().ty.clone();
            let right = self.factor();
            expr = Binary::boxed(expr, operator, right);
        }

        expr
    }

    fn factor(&mut self) -> BoxExpr {
        let mut expr = self.unary();

        while self.match_(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().unwrap().ty.clone();
            let right = self.unary();
            expr = Binary::boxed(expr, operator, right);
        }

        expr
    }

    fn unary(&mut self) -> BoxExpr {
        if self.match_(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().unwrap().ty.clone();
            let right = self.unary();
            return Unary::boxed(operator, right);
        }

        self.primary()
    }

    fn primary(&mut self) -> BoxExpr {
        if self.match_(&[TokenType::False]) {
            return Literal::boxed(TokenType::Bool(false));
        }
        if self.match_(&[TokenType::True]) {
            return Literal::boxed(TokenType::Bool(true));
        }
        if self.match_(&[TokenType::Nil]) {
            return Literal::boxed(TokenType::Nil);
        }
        if self.is_literal() {
            return Literal::boxed(self.previous().map(|t| t.ty.clone()).unwrap());
        }

        if self.match_(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Grouping::boxed(expr);
        }

        unreachable!("maybe? or just compiler/parser error")
    }

    // match_ for enum variants with values inside (eg: String and Number)
    fn is_literal(&mut self) -> bool {
        if self.is_at_end() {
            false
        } else if self
            .peek()
            .map(|token| token.ty.is_literal())
            .unwrap_or(false)
        {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_(&mut self, types: &[TokenType]) -> bool {
        for ty in types {
            if self.check(ty) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, ty: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().map(|token| token.ty == *ty).unwrap_or(false)
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn consume(&mut self, ty: TokenType, msg: &str) -> Option<&Token> {
        if self.check(&ty) {
            return self.advance();
        }

        self.error(self.peek(), msg);
    }

    fn is_at_end(&self) -> bool {
        self.peek()
            .map(|token| token.ty == TokenType::EOF)
            .unwrap_or(false)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn error(&self, at_token: Option<&Token>, msg: &str) -> ! {
        panic!("parser error at {at_token:?} {msg}")
    }
}

#[allow(unused)]
struct ParseError;
