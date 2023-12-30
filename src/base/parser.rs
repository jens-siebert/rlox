use crate::base::scanner::{Token, TokenType};
use crate::base::visitor::{RuntimeError, Visitor};
use std::cell::RefCell;
use std::fmt::Display;
use thiserror::Error;

#[derive(Clone, PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    None,
}

pub type LiteralValueRef = Box<LiteralValue>;

impl LiteralValue {
    pub fn number(value: f64) -> LiteralValue {
        LiteralValue::Number(value)
    }

    pub fn number_ref(value: f64) -> LiteralValueRef {
        Box::new(LiteralValue::number(value))
    }

    pub fn string(value: String) -> LiteralValue {
        LiteralValue::String(value)
    }

    pub fn string_ref(value: String) -> LiteralValueRef {
        Box::new(LiteralValue::string(value))
    }

    pub fn boolean(value: bool) -> LiteralValue {
        LiteralValue::Boolean(value)
    }

    pub fn boolean_ref(value: bool) -> LiteralValueRef {
        Box::new(LiteralValue::boolean(value))
    }

    pub fn none() -> LiteralValue {
        LiteralValue::None
    }

    pub fn none_ref() -> LiteralValueRef {
        Box::new(LiteralValue::none())
    }
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            LiteralValue::Number(value) => value.to_string(),
            LiteralValue::String(value) => value.to_string(),
            LiteralValue::Boolean(value) => value.to_string(),
            LiteralValue::None => String::from("nil"),
        };

        write!(f, "{}", result)
    }
}

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

pub enum Stmt<'a> {
    Block {
        statements: Vec<StmtRef<'a>>,
    },
    Expression {
        expression: ExprRef<'a>,
    },
    Print {
        expression: ExprRef<'a>,
    },
    Var {
        name: &'a Token,
        initializer: ExprRef<'a>,
    },
}

pub type StmtRef<'a> = Box<Stmt<'a>>;

impl<'a> Stmt<'a> {
    pub fn block(statements: Vec<StmtRef>) -> Stmt {
        Stmt::Block { statements }
    }

    pub fn block_ref(statements: Vec<StmtRef>) -> StmtRef {
        Box::new(Stmt::block(statements))
    }

    pub fn expression(expression: ExprRef) -> Stmt {
        Stmt::Expression { expression }
    }

    pub fn expression_ref(expression: ExprRef) -> StmtRef {
        Box::new(Stmt::expression(expression))
    }

    pub fn print(expression: ExprRef) -> Stmt {
        Stmt::Print { expression }
    }

    pub fn print_ref(expression: ExprRef) -> StmtRef {
        Box::new(Stmt::print(expression))
    }

    pub fn var(name: &'a Token, initializer: ExprRef<'a>) -> Stmt<'a> {
        Stmt::Var { name, initializer }
    }

    pub fn var_ref(name: &'a Token, initializer: ExprRef<'a>) -> StmtRef<'a> {
        Box::new(Stmt::var(name, initializer))
    }

    pub fn accept<R>(&self, visitor: &dyn Visitor<Stmt<'a>, R>) -> Result<R, RuntimeError> {
        visitor.visit(self)
    }
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Error while reading token.")]
    TokenReadError,
    #[error("Unknown token detected.")]
    MissingExpression,
    #[error("Expect ')' after expression.")]
    MissingRightParenthesis,
    #[error("Expect '}}' after block.")]
    MissingRightBrace,
    #[error("Expect ';' after value.")]
    MissingSemicolonAfterValue,
    #[error("Expect ';' after expression.")]
    MissingSemicolonAfterExpression,
    #[error("Expect ';' after variable declaration.")]
    MissingSemicolonAfterVariableDeclaration,
    #[error("Expect variable name.")]
    MissingVariableName,
    #[error("Invalid assignment target.")]
    InvalidAssignmentTarget,
}

pub struct Parser<'a> {
    pub tokens: &'a Vec<Token>,
    pub current: RefCell<usize>,
}

impl Parser<'_> {
    pub fn parse(&self) -> Result<Vec<StmtRef>, ParserError> {
        let mut statements: Vec<StmtRef> = vec![];

        while !self.is_at_end()? {
            statements.push(self.declaration()?)
        }

        Ok(statements)
    }

    fn declaration(&self) -> Result<StmtRef, ParserError> {
        if self.match_token_types(&[TokenType::Var])? {
            self.variable_declaration()
        } else {
            self.statement()
        }
    }

    fn variable_declaration(&self) -> Result<StmtRef, ParserError> {
        let name = self.consume(TokenType::Identifier, ParserError::MissingVariableName)?;
        let initializer = if self.match_token_types(&[TokenType::Equal])? {
            self.expression()?
        } else {
            Expr::literal_ref(LiteralValue::None)
        };

        self.consume(
            TokenType::Semicolon,
            ParserError::MissingSemicolonAfterVariableDeclaration,
        )?;

        Ok(Stmt::var_ref(name, initializer))
    }

    fn statement(&self) -> Result<StmtRef, ParserError> {
        if self.match_token_types(&[TokenType::Print])? {
            self.print_statement()
        } else if self.match_token_types(&[TokenType::LeftBrace])? {
            self.block()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&self) -> Result<StmtRef, ParserError> {
        let value = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            ParserError::MissingSemicolonAfterValue,
        )?;
        Ok(Stmt::print_ref(value))
    }

    fn block(&self) -> Result<StmtRef, ParserError> {
        let mut statements: Vec<StmtRef> = vec![];

        while !self.check(&TokenType::RightBrace)? && !self.is_at_end()? {
            statements.push(self.declaration()?)
        }

        self.consume(TokenType::RightBrace, ParserError::MissingRightBrace)?;

        Ok(Stmt::block_ref(statements))
    }

    fn expression_statement(&self) -> Result<StmtRef, ParserError> {
        let value = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            ParserError::MissingSemicolonAfterExpression,
        )?;
        Ok(Stmt::expression_ref(value))
    }

    fn expression(&self) -> Result<ExprRef, ParserError> {
        self.assignment()
    }

    fn assignment(&self) -> Result<ExprRef, ParserError> {
        let expr = self.equality()?;

        if self.match_token_types(&[TokenType::Equal])? {
            let value = self.assignment()?;

            return match *expr {
                Expr::Variable { name } => Ok(Expr::assign_ref(name, value)),
                _ => Err(ParserError::InvalidAssignmentTarget),
            };
        }

        Ok(expr)
    }

    fn equality(&self) -> Result<ExprRef, ParserError> {
        let mut expr = self.comparison()?;

        while self.match_token_types(&[TokenType::BangEqual, TokenType::EqualEqual])? {
            let operator = self.previous()?;
            let right = self.comparison()?;
            expr = Expr::binary_ref(expr, operator, right)
        }

        Ok(expr)
    }

    fn comparison(&self) -> Result<ExprRef, ParserError> {
        let mut expr = self.term()?;

        while self.match_token_types(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ])? {
            let operator = self.previous()?;
            let right = self.term()?;
            expr = Expr::binary_ref(expr, operator, right)
        }

        Ok(expr)
    }

    fn term(&self) -> Result<ExprRef, ParserError> {
        let mut expr = self.factor()?;

        while self.match_token_types(&[TokenType::Minus, TokenType::Plus])? {
            let operator = self.previous()?;
            let right = self.factor()?;
            expr = Expr::binary_ref(expr, operator, right)
        }

        Ok(expr)
    }

    fn factor(&self) -> Result<ExprRef, ParserError> {
        let mut expr = self.unary()?;

        while self.match_token_types(&[TokenType::Slash, TokenType::Star])? {
            let operator = self.previous()?;
            let right = self.unary()?;
            expr = Expr::binary_ref(expr, operator, right)
        }

        Ok(expr)
    }

    fn unary(&self) -> Result<ExprRef, ParserError> {
        if self.match_token_types(&[TokenType::Bang, TokenType::Minus])? {
            let operator = self.previous()?;
            let right = self.unary()?;
            return Ok(Expr::unary_ref(operator, right));
        }

        self.primary()
    }

    fn primary(&self) -> Result<ExprRef, ParserError> {
        if self.match_token_types(&[TokenType::False])? {
            return Ok(Expr::literal_ref(LiteralValue::Boolean(false)));
        }
        if self.match_token_types(&[TokenType::True])? {
            return Ok(Expr::literal_ref(LiteralValue::Boolean(true)));
        }
        if self.match_token_types(&[TokenType::Nil])? {
            return Ok(Expr::literal_ref(LiteralValue::None));
        }

        match &self.peek()?.token_type {
            TokenType::Number { value } => {
                self.advance()?;
                return Ok(Expr::literal_ref(LiteralValue::Number(*value)));
            }
            TokenType::String { value } => {
                self.advance()?;
                return Ok(Expr::literal_ref(LiteralValue::String(value.clone())));
            }
            _ => {}
        }

        if self.match_token_types(&[TokenType::Identifier])? {
            return Ok(Expr::variable_ref(self.previous()?));
        }

        if self.match_token_types(&[TokenType::LeftParen])? {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, ParserError::MissingRightParenthesis)?;
            return Ok(Expr::grouping_ref(expr));
        }

        Err(ParserError::MissingExpression)
    }

    fn peek(&self) -> Result<&Token, ParserError> {
        match self.tokens.get(*self.current.borrow()) {
            None => Err(ParserError::TokenReadError),
            Some(token) => Ok(token),
        }
    }

    fn previous(&self) -> Result<&Token, ParserError> {
        match self.tokens.get(*self.current.borrow() - 1) {
            None => Err(ParserError::TokenReadError),
            Some(token) => Ok(token),
        }
    }

    fn advance(&self) -> Result<&Token, ParserError> {
        if !self.is_at_end()? {
            *self.current.borrow_mut() += 1
        }
        self.previous()
    }

    fn consume(&self, token_type: TokenType, error: ParserError) -> Result<&Token, ParserError> {
        if self.check(&token_type)? {
            self.advance()
        } else {
            Err(error)
        }
    }

    fn is_at_end(&self) -> Result<bool, ParserError> {
        match self.peek() {
            Ok(token) => Ok(token.token_type == TokenType::Eof),
            Err(err) => Err(err),
        }
    }

    fn check(&self, token_type: &TokenType) -> Result<bool, ParserError> {
        if self.is_at_end()? {
            Ok(false)
        } else {
            match self.peek() {
                Ok(token) => Ok(token.token_type == *token_type),
                Err(err) => Err(err),
            }
        }
    }

    fn match_token_types(&self, token_types: &[TokenType]) -> Result<bool, ParserError> {
        for token_type in token_types {
            if self.check(token_type)? {
                self.advance()?;
                return Ok(true);
            }
        }

        Ok(false)
    }
}
