use crate::base::scanner::Token;
use crate::base::visitor::Visitor;
use ordered_float::OrderedFloat;
use uuid::Uuid;

pub trait ExprUuid {
    fn uuid(&self) -> Uuid;
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralValue {
    Number(OrderedFloat<f64>),
    String(String),
    Boolean(bool),
    None,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Binary {
        uuid: Uuid,
        left: Box<Expr>,
        operator: Box<Token>,
        right: Box<Expr>,
    },
    Call {
        uuid: Uuid,
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    Grouping {
        uuid: Uuid,
        expression: Box<Expr>,
    },
    Literal {
        uuid: Uuid,
        value: LiteralValue,
    },
    Logical {
        uuid: Uuid,
        left: Box<Expr>,
        operator: Box<Token>,
        right: Box<Expr>,
    },
    Unary {
        uuid: Uuid,
        operator: Box<Token>,
        right: Box<Expr>,
    },
    Variable {
        uuid: Uuid,
        name: Box<Token>,
    },
    Assign {
        uuid: Uuid,
        name: Box<Token>,
        value: Box<Expr>,
    },
}

impl Expr {
    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Binary {
            uuid: Uuid::new_v4(),
            left: Box::new(left),
            operator: Box::new(operator),
            right: Box::new(right),
        }
    }

    pub fn call(callee: Expr, arguments: Vec<Expr>) -> Self {
        Expr::Call {
            uuid: Uuid::new_v4(),
            callee: Box::new(callee),
            arguments,
        }
    }

    pub fn grouping(expression: Expr) -> Self {
        Expr::Grouping {
            uuid: Uuid::new_v4(),
            expression: Box::new(expression),
        }
    }

    pub fn literal(value: LiteralValue) -> Self {
        Expr::Literal {
            uuid: Uuid::new_v4(),
            value,
        }
    }

    pub fn logical(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Logical {
            uuid: Uuid::new_v4(),
            left: Box::new(left),
            operator: Box::new(operator),
            right: Box::new(right),
        }
    }

    pub fn unary(operator: Token, right: Expr) -> Self {
        Expr::Unary {
            uuid: Uuid::new_v4(),
            operator: Box::new(operator),
            right: Box::new(right),
        }
    }

    pub fn variable(name: Token) -> Self {
        Expr::Variable {
            uuid: Uuid::new_v4(),
            name: Box::new(name),
        }
    }

    pub fn assign(name: Token, value: Expr) -> Self {
        Expr::Assign {
            uuid: Uuid::new_v4(),
            name: Box::new(name),
            value: Box::new(value),
        }
    }

    pub fn accept<R, E>(&self, visitor: &dyn Visitor<Expr, R, E>) -> Result<R, E> {
        visitor.visit(self)
    }
}

impl ExprUuid for Expr {
    fn uuid(&self) -> Uuid {
        *match &self {
            Expr::Binary {
                uuid,
                left: _left,
                operator: _operator,
                right: _right,
            } => uuid,
            Expr::Call {
                uuid,
                callee: _callee,
                arguments: _arguments,
            } => uuid,
            Expr::Grouping {
                uuid,
                expression: _expression,
            } => uuid,
            Expr::Literal {
                uuid,
                value: _value,
            } => uuid,
            Expr::Logical {
                uuid,
                left: _left,
                operator: _operator,
                right: _right,
            } => uuid,
            Expr::Unary {
                uuid,
                operator: _operator,
                right: _right,
            } => uuid,
            Expr::Variable { uuid, name: _name } => uuid,
            Expr::Assign {
                uuid,
                name: _name,
                value: _value,
            } => uuid,
        }
    }
}
