use crate::base::scanner::TokenRef;
use crate::base::visitor::{RuntimeError, Visitor};

pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    None,
}

pub enum Expr {
    Binary {
        left: ExprRef,
        operator: TokenRef,
        right: ExprRef,
    },
    Call {
        callee: ExprRef,
        arguments: Vec<ExprRef>,
    },
    Grouping {
        expression: ExprRef,
    },
    Literal {
        value: LiteralValue,
    },
    Logical {
        left: ExprRef,
        operator: TokenRef,
        right: ExprRef,
    },
    Unary {
        operator: TokenRef,
        right: ExprRef,
    },
    Variable {
        name: TokenRef,
    },
    Assign {
        name: TokenRef,
        value: ExprRef,
    },
}

pub type ExprRef = Box<Expr>;

impl Expr {
    pub fn binary(left: ExprRef, operator: TokenRef, right: ExprRef) -> Self {
        Expr::Binary {
            left,
            operator,
            right,
        }
    }

    pub fn binary_ref(left: ExprRef, operator: TokenRef, right: ExprRef) -> Box<Self> {
        Box::new(Expr::binary(left, operator, right))
    }

    pub fn call(callee: ExprRef, arguments: Vec<ExprRef>) -> Self {
        Expr::Call { callee, arguments }
    }

    pub fn call_ref(callee: ExprRef, arguments: Vec<ExprRef>) -> Box<Self> {
        Box::new(Expr::call(callee, arguments))
    }

    pub fn grouping(expression: ExprRef) -> Self {
        Expr::Grouping { expression }
    }

    pub fn grouping_ref(expression: ExprRef) -> Box<Self> {
        Box::new(Expr::grouping(expression))
    }

    pub fn literal(value: LiteralValue) -> Self {
        Expr::Literal { value }
    }

    pub fn literal_ref(value: LiteralValue) -> Box<Self> {
        Box::new(Expr::literal(value))
    }

    pub fn logical(left: ExprRef, operator: TokenRef, right: ExprRef) -> Self {
        Expr::Logical {
            left,
            operator,
            right,
        }
    }

    pub fn logical_ref(left: ExprRef, operator: TokenRef, right: ExprRef) -> Box<Self> {
        Box::new(Expr::logical(left, operator, right))
    }

    pub fn unary(operator: TokenRef, right: ExprRef) -> Self {
        Expr::Unary { operator, right }
    }

    pub fn unary_ref(operator: TokenRef, right: ExprRef) -> Box<Self> {
        Box::new(Expr::unary(operator, right))
    }

    pub fn variable(name: TokenRef) -> Self {
        Expr::Variable { name }
    }

    pub fn variable_ref(name: TokenRef) -> Box<Self> {
        Box::new(Expr::variable(name))
    }

    pub fn assign(name: TokenRef, value: ExprRef) -> Self {
        Expr::Assign { name, value }
    }

    pub fn assign_ref(name: TokenRef, value: ExprRef) -> Box<Self> {
        Box::new(Expr::assign(name, value))
    }

    pub fn accept<R>(&self, visitor: &dyn Visitor<Expr, R>) -> Result<R, RuntimeError> {
        visitor.visit(self)
    }
}
