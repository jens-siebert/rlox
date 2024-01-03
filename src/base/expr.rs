use crate::base::scanner::Token;
use crate::base::visitor::{RuntimeError, Visitor};

pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    None,
}

pub enum Expr<'a> {
    Binary {
        left: ExprRef<'a>,
        operator: &'a Token,
        right: ExprRef<'a>,
    },
    Call {
        callee: ExprRef<'a>,
        arguments: Vec<ExprRef<'a>>,
    },
    Grouping {
        expression: ExprRef<'a>,
    },
    Literal {
        value: LiteralValue,
    },
    Logical {
        left: ExprRef<'a>,
        operator: &'a Token,
        right: ExprRef<'a>,
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

    pub fn call(callee: ExprRef<'a>, arguments: Vec<ExprRef<'a>>) -> Expr<'a> {
        Expr::Call { callee, arguments }
    }

    pub fn call_ref(callee: ExprRef<'a>, arguments: Vec<ExprRef<'a>>) -> ExprRef<'a> {
        Box::new(Expr::call(callee, arguments))
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

    pub fn logical(left: ExprRef<'a>, operator: &'a Token, right: ExprRef<'a>) -> Expr<'a> {
        Expr::Logical {
            left,
            operator,
            right,
        }
    }

    pub fn logical_ref(left: ExprRef<'a>, operator: &'a Token, right: ExprRef<'a>) -> ExprRef<'a> {
        Box::new(Expr::logical(left, operator, right))
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
