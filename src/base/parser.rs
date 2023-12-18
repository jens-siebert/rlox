use crate::base::scanner::Token;

pub trait Visitor<R> {
    fn visit(&self, expr: &Expr) -> R;
}

pub enum Expr {
    Binary(ExprRef, Token, ExprRef),
    Grouping(ExprRef),
    Literal(Option<Box<dyn ToString>>),
    Unary(Token, ExprRef),
}

impl Expr {
    pub fn binary(left: ExprRef, operator: Token, right: ExprRef) -> ExprRef {
        Box::new(Expr::Binary(left, operator, right))
    }

    pub fn grouping(expression: ExprRef) -> ExprRef {
        Box::new(Expr::Grouping(expression))
    }

    pub fn literal(value: Option<Box<dyn ToString>>) -> ExprRef {
        Box::new(Expr::Literal(value))
    }

    pub fn unary(operator: Token, right: ExprRef) -> ExprRef {
        Box::new(Expr::Unary(operator, right))
    }

    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        visitor.visit(self)
    }
}

pub type ExprRef = Box<Expr>;
