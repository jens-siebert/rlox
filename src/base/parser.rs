use crate::base::scanner::Token;
use std::fmt::Display;

pub trait Visitor {
    fn visit_binary_expr(&self, expr: &Binary) -> String;
    fn visit_grouping_expr(&self, expr: &Grouping) -> String;
    fn visit_literal_expr(&self, expr: &Literal) -> String;
    fn visit_unary_expr(&self, expr: &Unary) -> String;
}

pub trait Expr {
    fn accept(&self, visitor: &dyn Visitor) -> String;
}

pub struct Binary {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

pub struct Grouping {
    pub expression: Box<dyn Expr>,
}

pub struct Literal {
    pub value: Option<Box<dyn Display>>,
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Binary {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Box<Self> {
        Box::new(Binary {
            left,
            operator,
            right,
        })
    }
}

impl Expr for Binary {
    fn accept(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_binary_expr(self)
    }
}

impl Grouping {
    pub fn new(expression: Box<dyn Expr>) -> Box<Self> {
        Box::new(Grouping { expression })
    }
}

impl Expr for Grouping {
    fn accept(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_grouping_expr(self)
    }
}

impl Literal {
    pub fn new(value: Option<Box<dyn Display>>) -> Box<Self> {
        Box::new(Literal { value })
    }
}

impl Expr for Literal {
    fn accept(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_literal_expr(self)
    }
}

impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expr>) -> Box<Self> {
        Box::new(Unary { operator, right })
    }
}

impl Expr for Unary {
    fn accept(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_unary_expr(self)
    }
}
