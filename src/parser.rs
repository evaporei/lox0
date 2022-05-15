use crate::expr::{Binary, BoxExpr, Grouping, Literal, Unary};
use crate::token::{Token, TokenType};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn expression(&self) -> BoxExpr {
        self.equality()
    }

    fn equality(&self) -> BoxExpr {
        let mut expr = self.comparison();

        while self.match_(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let rhs = self.comparison();
            expr = Binary::boxed(expr, op, rhs);
        }

        expr
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

    fn check(&self, ty: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().map(|token| token.ty == ty).unwrap_or(false)
        }
    }

    fn advance(&self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
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

    fn comparison(&self) -> BoxExpr {
        let mut expr = self.term();

        while self.match_(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous();
            let rhs = self.term();
            expr = Binary::boxed(expr, op, rhs);
        }

        expr
    }

    fn term(&self) -> BoxExpr {
        let mut expr = self.term();

        while self.match_(&[
            TokenType::Plus,
            TokenType::Minus,
        ]) {
            let op = self.previous();
            let rhs = self.factor();
            expr = Binary::boxed(expr, op, rhs);
        }

        expr
    }

    fn factor(&self) -> BoxExpr {
        let mut expr = self.term();

        while self.match_(&[
            TokenType::Slash,
            TokenType::Star,
        ]) {
            let op = self.previous();
            let rhs = self.unary();
            expr = Binary::boxed(expr, op, rhs);
        }

        expr
    }

    fn unary(&self) -> BoxExpr {
        if self.match_(&[
            TokenType::Bang,
            TokenType::Minus,
        ]) {
            let op = self.previous();
            let rhs = self.unary();
            return Unary::boxed(op, rhs);
        }

        self.primary()
    }

    fn primary(&self) -> BoxExpr {
        if self.match_(&[TokenType::False]) {
            return Literal::boxed(Token::new(TokenType::False, "false", 1));
        }

        if self.match_(&[TokenType::True]) {
            return Literal::boxed(Token::new(TokenType::True, "true", 1));
        }

        if self.match_(&[TokenType::Nil]) {
            return Literal::boxed(Token::new(TokenType::Nil, "nil", 1));
        }

        if self.match_(&[TokenType::Number(1.0)]) {
            return Literal::boxed(Token::new(TokenType::Number(1.0), "1", 1));
        }

        if self.match_(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Grouping::boxed(expr);
        }

        unreachable!("boom")
    }

    fn consume(&self, ty: TokenType, msg: &str) -> Token {
        if self.
    }
}
