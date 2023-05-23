#![allow(dead_code)]

use crate::token::Token;

pub trait Expr: std::fmt::Display {}

pub type BoxExpr = Box<dyn Expr>;

pub struct Binary {
    pub lhs: BoxExpr,
    pub op: Token,
    pub rhs: BoxExpr,
}

impl Binary {
    pub fn new(lhs: BoxExpr, op: Token, rhs: BoxExpr) -> Self {
        Self { lhs, op, rhs }
    }

    pub fn boxed(lhs: BoxExpr, op: Token, rhs: BoxExpr) -> Box<dyn Expr> {
        Box::new(Self::new(lhs, op, rhs))
    }
}

impl Expr for Binary {}

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

impl Expr for Grouping {}

pub struct Literal {
    pub expr: Token,
}

impl Literal {
    pub fn new(expr: Token) -> Self {
        Self { expr }
    }

    pub fn boxed(expr: Token) -> Box<Self> {
        Box::new(Self::new(expr))
    }
}

impl Expr for Literal {}

pub struct Unary {
    pub op: Token,
    pub rhs: BoxExpr,
}

impl Unary {
    pub fn new(op: Token, rhs: BoxExpr) -> Self {
        Self { op, rhs }
    }

    pub fn boxed(op: Token, rhs: BoxExpr) -> Box<Self> {
        Box::new(Self::new(op, rhs))
    }
}

impl Expr for Unary {}
