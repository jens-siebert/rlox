use crate::base::scanner::{Token, TokenType};
use std::cell::RefCell;
use thiserror::Error;

pub trait Visitor<R> {
    fn visit_expr(&self, expr: &Expr) -> R;
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
        value: Option<Box<dyn ToString>>,
    },
    Unary {
        operator: &'a Token,
        right: ExprRef<'a>,
    },
}

pub type ExprRef<'a> = Box<Expr<'a>>;

impl Expr<'_> {
    pub fn binary<'a>(left: ExprRef<'a>, operator: &'a Token, right: ExprRef<'a>) -> Expr<'a> {
        Expr::Binary {
            left,
            operator,
            right,
        }
    }

    pub fn binary_ref<'a>(
        left: ExprRef<'a>,
        operator: &'a Token,
        right: ExprRef<'a>,
    ) -> ExprRef<'a> {
        Box::new(Expr::binary(left, operator, right))
    }

    pub fn grouping(expression: ExprRef) -> Expr {
        Expr::Grouping { expression }
    }

    pub fn grouping_ref(expression: ExprRef) -> ExprRef {
        Box::new(Expr::grouping(expression))
    }

    pub fn literal<'a>(value: Option<Box<dyn ToString>>) -> Expr<'a> {
        Expr::Literal { value }
    }

    pub fn literal_ref<'a>(value: Option<Box<dyn ToString>>) -> ExprRef<'a> {
        Box::new(Expr::literal(value))
    }

    pub fn unary<'a>(operator: &'a Token, right: ExprRef<'a>) -> Expr<'a> {
        Expr::Unary { operator, right }
    }

    pub fn unary_ref<'a>(operator: &'a Token, right: ExprRef<'a>) -> ExprRef<'a> {
        Box::new(Expr::unary(operator, right))
    }

    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        visitor.visit_expr(self)
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
}

pub struct Parser<'a> {
    pub tokens: &'a Vec<Token>,
    pub current: RefCell<usize>,
}

impl Parser<'_> {
    pub fn parse(&self) -> Result<ExprRef, ParserError> {
        self.expression()
    }

    fn expression(&self) -> Result<ExprRef, ParserError> {
        self.equality()
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
            return Ok(Expr::literal_ref(Some(Box::new(false))));
        }
        if self.match_token_types(&[TokenType::True])? {
            return Ok(Expr::literal_ref(Some(Box::new(true))));
        }
        if self.match_token_types(&[TokenType::Nil])? {
            return Ok(Expr::literal_ref(Some(Box::new("null"))));
        }

        match &self.peek()?.token_type {
            TokenType::Number { value } => {
                self.advance()?;
                return Ok(Expr::literal_ref(Some(Box::new(*value))))
            },
            TokenType::String { value } => {
                self.advance()?;
                return Ok(Expr::literal_ref(Some(Box::new(value.clone()))))
            }
            _ => {}
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
