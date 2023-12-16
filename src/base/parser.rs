use crate::base::scanner::Token;

pub trait Visitor {
    fn visit_binary_expr(&self, expr: &Binary) -> String;
    fn visit_grouping_expr(&self, expr: &Grouping) -> String;
    fn visit_literal_expr(&self, expr: &Literal) -> String;
    fn visit_unary_expr(&self, expr: &Unary) -> String;
}

pub trait Expr {
    fn accept(&self, visitor: &dyn Visitor) -> String;
}

pub type ExprRef = Box<dyn Expr>;

pub struct Binary {
    pub left: ExprRef,
    pub operator: Token,
    pub right: ExprRef,
}

pub struct Grouping {
    pub expression: ExprRef,
}

pub struct Literal {
    pub value: Option<Box<dyn ToString>>,
}

pub struct Unary {
    pub operator: Token,
    pub right: ExprRef,
}

impl Binary {
    pub fn new(left: ExprRef, operator: Token, right: ExprRef) -> Box<Self> {
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
    pub fn new(expression: ExprRef) -> Box<Self> {
        Box::new(Grouping { expression })
    }
}

impl Expr for Grouping {
    fn accept(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_grouping_expr(self)
    }
}

impl Literal {
    pub fn new(value: Option<Box<dyn ToString>>) -> Box<Self> {
        Box::new(Literal { value })
    }
}

impl Expr for Literal {
    fn accept(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_literal_expr(self)
    }
}

impl Unary {
    pub fn new(operator: Token, right: ExprRef) -> Box<Self> {
        Box::new(Unary { operator, right })
    }
}

impl Expr for Unary {
    fn accept(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_unary_expr(self)
    }
}
