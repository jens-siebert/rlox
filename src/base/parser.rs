use crate::base::scanner::Token;

pub trait Visitor<R> {
    fn visit_expr(&self, expr: &Expr) -> R;
}

pub enum Expr<'a> {
    Binary { left: &'a Expr<'a>, operator: &'a Token, right: &'a Expr<'a> },
    Grouping { expression: &'a Expr<'a> },
    Literal { value: Option<Box<dyn ToString>> },
    Unary { operator: &'a Token, right: &'a Expr<'a> },
}

impl Expr<'_> {
    pub fn binary<'a>(left: &'a Expr, operator: &'a Token, right: &'a Expr) -> Expr<'a>{
        Expr::Binary { left, operator, right }
    }

    pub fn grouping<'a>(expression: &'a Expr) -> Expr<'a> {
        Expr::Grouping { expression }
    }

    pub fn literal<'a>(value: Option<Box<dyn ToString>>) -> Expr<'a> {
        Expr::Literal { value }
    }

    pub fn unary<'a>(operator: &'a Token, right: &'a Expr) -> Expr<'a> {
        Expr::Unary { operator, right }
    }

    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        visitor.visit_expr(self)
    }
}
