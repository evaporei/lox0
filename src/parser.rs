use crate::expr::{Binary, BoxExpr, Grouping, Literal, Unary};
use crate::token::{Token, TokenType};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&self) -> BoxExpr {
        self.equality()
    }

    fn equality(&self) -> BoxExpr {
        let mut expr = self.comparison();

        while self.match_(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Binary::boxed(expr, operator, right);
        }

        expr
    }

    fn comparison(&self) -> BoxExpr {
        let mut expr = self.term();

        while self.match_(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Binary::boxed(expr, operator, right);
        }

        expr
    }

    fn term(&self) -> BoxExpr {
        let mut expr = self.unary();

        while self.match_(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Binary::boxed(expr, operator, right);
        }

        expr
    }

    fn factor(&self) -> BoxExpr {
        let mut expr = self.unary();

        while self.match_(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Binary::boxed(expr, operator, right);
        }

        expr
    }

    fn unary(&self) -> BoxExpr {
        if self.match_(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Unary::boxed(operator, right);
        }

        self.primary()
    }

    fn primary(&self) -> BoxExpr {
        if self.match_(&[TokenType::False]) {
            return Literal::boxed(false);
        }
        if self.match_(&[TokenType::True]) {
            return Literal::boxed(true);
        }
        if self.match_(&[TokenType::Nil]) {
            return Literal::boxed(todo!("nil?"));
        }

        if self.match_(&[todo!("remove values from variants?")]) {
            return Literal::boxed(self.previous().map(|t| t.literal));
        }

        if self.match_(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.".into());
            return Grouping::boxed(expr);
        }

        unreachable!("maybe? or just compiler/parser error")
    }

    fn match_(&self, types: &[TokenType]) -> bool {
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

    fn advance(&self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn consume(&self, ty: TokenType, msg: String) -> Option<&Token> {
        if self.check(&ty) {
            return self.advance();
        }

        self.error();
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

    fn error(&self) -> ! {
        panic!("parser error")
    }
}

#[allow(unused)]
struct ParseError;
