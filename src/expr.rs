#![allow(dead_code)]

use crate::token::TokenType;

pub trait Expr: std::fmt::Display {
    fn visit(&self) -> Option<TokenType>;
}

pub type BoxExpr = Box<dyn Expr>;

pub struct Binary {
    pub lhs: BoxExpr,
    pub op: TokenType,
    pub rhs: BoxExpr,
}

impl Binary {
    pub fn new(lhs: BoxExpr, op: TokenType, rhs: BoxExpr) -> Self {
        Self { lhs, op, rhs }
    }

    pub fn boxed(lhs: BoxExpr, op: TokenType, rhs: BoxExpr) -> Box<Self> {
        Box::new(Self::new(lhs, op, rhs))
    }
}

impl Expr for Binary {
    fn visit(&self) -> Option<TokenType> {
        let left = self.lhs.visit();
        let right = self.rhs.visit();

        match (left, &self.op, right) {
            // comparisons
            (Some(TokenType::Number(l)), TokenType::Greater, Some(TokenType::Number(r))) => {
                Some(TokenType::Bool(l > r))
            }
            (Some(TokenType::Number(l)), TokenType::GreaterEqual, Some(TokenType::Number(r))) => {
                Some(TokenType::Bool(l >= r))
            }
            (Some(TokenType::Number(l)), TokenType::Less, Some(TokenType::Number(r))) => {
                Some(TokenType::Bool(l < r))
            }
            (Some(TokenType::Number(l)), TokenType::LessEqual, Some(TokenType::Number(r))) => {
                Some(TokenType::Bool(l <= r))
            }
            (Some(l), TokenType::BangEqual, Some(r)) => Some(TokenType::Bool(!l.is_equal(&r))),
            (Some(l), TokenType::EqualEqual, Some(r)) => Some(TokenType::Bool(l.is_equal(&r))),

            // arithmetic
            (Some(TokenType::Number(l)), TokenType::Minus, Some(TokenType::Number(r))) => {
                Some(TokenType::Number(l - r))
            }
            (Some(TokenType::Number(l)), TokenType::Slash, Some(TokenType::Number(r))) => {
                Some(TokenType::Number(l / r))
            }
            (Some(TokenType::Number(l)), TokenType::Star, Some(TokenType::Number(r))) => {
                Some(TokenType::Number(l * r))
            }
            (Some(TokenType::Number(l)), TokenType::Plus, Some(TokenType::Number(r))) => {
                Some(TokenType::Number(l + r))
            }

            // concatenation
            (Some(TokenType::String(s)), TokenType::Plus, Some(TokenType::String(u))) => {
                Some(TokenType::String(s + &u))
            }
            _ => None,
        }
    }
}

pub struct Grouping {
    pub expr: BoxExpr,
}

impl Grouping {
    pub fn new(expr: BoxExpr) -> Self {
        Self { expr }
    }

    pub fn boxed(expr: BoxExpr) -> Box<Self> {
        Box::new(Self::new(expr))
    }
}

impl Expr for Grouping {
    fn visit(&self) -> Option<TokenType> {
        self.expr.visit()
    }
}

pub struct Literal {
    pub expr: TokenType,
}

impl Literal {
    pub fn new(expr: TokenType) -> Self {
        Self { expr }
    }

    pub fn boxed(expr: TokenType) -> Box<Self> {
        Box::new(Self::new(expr))
    }
}

impl Expr for Literal {
    fn visit(&self) -> Option<TokenType> {
        Some(self.expr.clone())
    }
}

pub struct Unary {
    pub op: TokenType,
    pub rhs: BoxExpr,
}

impl Unary {
    pub fn new(op: TokenType, rhs: BoxExpr) -> Self {
        Self { op, rhs }
    }

    pub fn boxed(op: TokenType, rhs: BoxExpr) -> Box<Self> {
        Box::new(Self::new(op, rhs))
    }
}

impl Expr for Unary {
    fn visit(&self) -> Option<TokenType> {
        let right = self.rhs.visit();

        match (&self.op, right) {
            (TokenType::Bang, Some(ty)) => Some(TokenType::Bool(!ty.is_truthy())),
            (TokenType::Minus, Some(TokenType::Number(n))) => Some(TokenType::Number(-n)),
            _ => None,
        }
    }
}
