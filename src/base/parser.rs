use crate::base::scanner::TokenRef;

pub trait Visitor<R> {
    fn visit_expr(&self, expr: &Expr) -> R;
}

pub enum Expr {
    Binary { left: ExprRef, operator: TokenRef, right: ExprRef },
    Grouping { expression: ExprRef },
    Literal { value: Option<Box<dyn ToString>> },
    Unary { operator: TokenRef, right: ExprRef },
}

pub type ExprRef = Box<Expr>;

impl Expr {
    pub fn binary(left: ExprRef, operator: TokenRef, right: ExprRef) -> Expr {
        Expr::Binary { left, operator, right }
    }

    pub fn binary_ref(left: ExprRef, operator: TokenRef, right: ExprRef) -> ExprRef {
        Box::new(Expr::binary(left, operator, right))
    }

    pub fn grouping(expression: ExprRef) -> Expr {
        Expr::Grouping { expression }
    }

    pub fn grouping_ref(expression: ExprRef) -> ExprRef {
        Box::new(Expr::grouping(expression))
    }

    pub fn literal(value: Option<Box<dyn ToString>>) -> Expr {
        Expr::Literal { value }
    }

    pub fn literal_ref(value: Option<Box<dyn ToString>>) -> ExprRef {
        Box::new(Expr::literal(value))
    }

    pub fn unary(operator: TokenRef, right: ExprRef) -> Expr {
        Expr::Unary { operator, right }
    }

    pub fn unary_ref(operator: TokenRef, right: ExprRef) -> ExprRef {
        Box::new(Expr::unary(operator, right))
    }

    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        visitor.visit_expr(self)
    }
}
