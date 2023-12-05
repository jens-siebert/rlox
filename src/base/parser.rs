use crate::base::scanner::Token;
use std::any::Any;

trait Visitor<R> {
    fn visit_binary_expr(&self, expr: &Binary) -> R;
    fn visit_grouping_expr(&self, expr: &Grouping) -> R;
    fn visit_literal_expr(&self, expr: &Literal) -> R;
    fn visit_unary_expr(&self, expt: &Unary) -> R;
}

trait Expr {
    fn accept<R>(&self, visitor: Box<dyn Visitor<R>>) -> R;
}

struct Binary {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>,
}

struct Grouping {
    expression: Box<dyn Expr>,
}

struct Literal {
    value: Box<dyn Any>,
}

struct Unary {
    operator: Token,
    expr: Box<dyn Expr>,
}

impl Binary {
    fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }
}

impl Expr for Binary {
    fn accept<R>(&self, visitor: Box<dyn Visitor<R>>) -> R {
        visitor.visit_binary_expr(self)
    }
}

impl Grouping {
    fn new(expression: Box<dyn Expr>) -> Self {
        Grouping { expression }
    }
}

impl Expr for Grouping {
    fn accept<R>(&self, visitor: Box<dyn Visitor<R>>) -> R {
        visitor.visit_grouping_expr(self)
    }
}

impl Literal {
    fn new(value: Box<dyn Any>) -> Self {
        Literal { value }
    }
}

impl Expr for Literal {
    fn accept<R>(&self, visitor: Box<dyn Visitor<R>>) -> R {
        visitor.visit_literal_expr(self)
    }
}

impl Unary {
    fn new(operator: Token, expr: Box<dyn Expr>) -> Self {
        Unary { operator, expr }
    }
}

impl Expr for Unary {
    fn accept<R>(&self, visitor: Box<dyn Visitor<R>>) -> R {
        visitor.visit_unary_expr(self)
    }
}
