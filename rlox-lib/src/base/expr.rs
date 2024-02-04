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
    Assign {
        uuid: Uuid,
        name: Box<Token>,
        value: Box<Expr>,
    },
    Binary {
        uuid: Uuid,
        left: Box<Expr>,
        operator: Box<Token>,
        right: Box<Expr>,
    },
    Call {
        uuid: Uuid,
        paren: Box<Token>,
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    Get {
        uuid: Uuid,
        object: Box<Expr>,
        name: Box<Token>,
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
    Set {
        uuid: Uuid,
        object: Box<Expr>,
        name: Box<Token>,
        value: Box<Expr>,
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
}

impl Expr {
    pub fn assign(name: Token, value: Expr) -> Self {
        Expr::Assign {
            uuid: Uuid::new_v4(),
            name: Box::new(name),
            value: Box::new(value),
        }
    }

    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Binary {
            uuid: Uuid::new_v4(),
            left: Box::new(left),
            operator: Box::new(operator),
            right: Box::new(right),
        }
    }

    pub fn call(paren: Token, callee: Expr, arguments: Vec<Expr>) -> Self {
        Expr::Call {
            uuid: Uuid::new_v4(),
            paren: Box::new(paren),
            callee: Box::new(callee),
            arguments,
        }
    }

    pub fn get(object: Expr, name: Token) -> Self {
        Expr::Get {
            uuid: Uuid::new_v4(),
            object: Box::new(object),
            name: Box::new(name),
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

    pub fn set(object: Expr, name: Token, value: Expr) -> Self {
        Expr::Set {
            uuid: Uuid::new_v4(),
            object: Box::new(object),
            name: Box::new(name),
            value: Box::new(value),
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

    pub fn accept<R, E>(&self, visitor: &dyn Visitor<Expr, R, E>) -> Result<R, E> {
        visitor.visit(self)
    }
}

impl ExprUuid for Expr {
    fn uuid(&self) -> Uuid {
        *match &self {
            Expr::Assign {
                uuid,
                name: _name,
                value: _value,
            } => uuid,
            Expr::Binary {
                uuid,
                left: _left,
                operator: _operator,
                right: _right,
            } => uuid,
            Expr::Call {
                uuid,
                paren: _paren,
                callee: _callee,
                arguments: _arguments,
            } => uuid,
            Expr::Get {
                uuid,
                object: _object,
                name: _name,
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
            Expr::Set {
                uuid,
                object: _object,
                name: _name,
                value: _value,
            } => uuid,
            Expr::Unary {
                uuid,
                operator: _operator,
                right: _right,
            } => uuid,
            Expr::Variable { uuid, name: _name } => uuid,
        }
    }
}
