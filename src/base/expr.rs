use crate::base::scanner::Token;
use crate::base::visitor::{RuntimeError, Visitor};

#[derive(Clone, PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    None,
}

#[derive(Clone, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
}

impl Expr {
    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn call(callee: Expr, arguments: Vec<Expr>) -> Self {
        Expr::Call {
            callee: Box::new(callee),
            arguments,
        }
    }

    pub fn grouping(expression: Expr) -> Self {
        Expr::Grouping {
            expression: Box::new(expression),
        }
    }

    pub fn literal(value: LiteralValue) -> Self {
        Expr::Literal { value }
    }

    pub fn logical(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Logical {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn unary(operator: Token, right: Expr) -> Self {
        Expr::Unary {
            operator,
            right: Box::new(right),
        }
    }

    pub fn variable(name: Token) -> Self {
        Expr::Variable { name }
    }

    pub fn assign(name: Token, value: Expr) -> Self {
        Expr::Assign { name, value: Box::new(value) }
    }

    pub fn accept<R>(&self, visitor: &dyn Visitor<Expr, R>) -> Result<R, RuntimeError> {
        visitor.visit(self)
    }
}
