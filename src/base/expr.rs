use crate::base::literal::LiteralValue;
use crate::base::scanner::Token;
use crate::base::visitor::{RuntimeError, Visitor};

pub enum Expr<'a> {
    Binary {
        left: ExprRef<'a>,
        operator: &'a Token,
        right: ExprRef<'a>,
    },
    Grouping {
        expression: ExprRef<'a>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: &'a Token,
        right: ExprRef<'a>,
    },
    Variable {
        name: &'a Token,
    },
    Assign {
        name: &'a Token,
        value: ExprRef<'a>,
    },
}

pub type ExprRef<'a> = Box<Expr<'a>>;

impl<'a> Expr<'a> {
    pub fn binary(left: ExprRef<'a>, operator: &'a Token, right: ExprRef<'a>) -> Expr<'a> {
        Expr::Binary {
            left,
            operator,
            right,
        }
    }

    pub fn binary_ref(left: ExprRef<'a>, operator: &'a Token, right: ExprRef<'a>) -> ExprRef<'a> {
        Box::new(Expr::binary(left, operator, right))
    }

    pub fn grouping(expression: ExprRef) -> Expr {
        Expr::Grouping { expression }
    }

    pub fn grouping_ref(expression: ExprRef) -> ExprRef {
        Box::new(Expr::grouping(expression))
    }

    pub fn literal(value: LiteralValue) -> Expr<'a> {
        Expr::Literal { value }
    }

    pub fn literal_ref(value: LiteralValue) -> ExprRef<'a> {
        Box::new(Expr::literal(value))
    }

    pub fn unary(operator: &'a Token, right: ExprRef<'a>) -> Expr<'a> {
        Expr::Unary { operator, right }
    }

    pub fn unary_ref(operator: &'a Token, right: ExprRef<'a>) -> ExprRef<'a> {
        Box::new(Expr::unary(operator, right))
    }

    pub fn variable(name: &'a Token) -> Expr<'a> {
        Expr::Variable { name }
    }

    pub fn variable_ref(name: &'a Token) -> ExprRef<'a> {
        Box::new(Expr::variable(name))
    }

    pub fn assign(name: &'a Token, value: ExprRef<'a>) -> Expr<'a> {
        Expr::Assign { name, value }
    }

    pub fn assign_ref(name: &'a Token, value: ExprRef<'a>) -> ExprRef<'a> {
        Box::new(Expr::assign(name, value))
    }

    pub fn accept<R>(
        &self,
        visitor: &'a (dyn Visitor<Expr<'a>, R> + 'a),
    ) -> Result<R, RuntimeError> {
        visitor.visit(self)
    }
}