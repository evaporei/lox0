#![allow(dead_code)]

use crate::token::Token;

trait Expr {}

type BoxedExpr = Box<dyn Expr>;

struct Binary {
    left: BoxedExpr,
    operator: Token,
    right: BoxedExpr,
}

impl Binary {
    fn new(left: BoxedExpr, operator: Token, right: BoxedExpr) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl Expr for Binary {}

struct Grouping {
    expression: BoxedExpr,
}

impl Grouping {
    fn new(expression: BoxedExpr) -> Self {
        Self { expression }
    }
}

impl Expr for Grouping {}

struct Literal {
    value: BoxedExpr,
}

impl Literal {
    fn new(value: BoxedExpr) -> Self {
        Self { value }
    }
}

impl Expr for Literal {}

struct Unary {
    operator: Token,
    right: BoxedExpr,
}

impl Unary {
    fn new(operator: Token, right: BoxedExpr) -> Self {
        Self { operator, right }
    }
}

impl Expr for Unary {}
