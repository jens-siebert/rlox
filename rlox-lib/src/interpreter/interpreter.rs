use crate::base::expr::{Expr, LiteralValue};
use crate::base::expr_result::{Callable, LoxFunction};
use crate::base::expr_result::{ExprResult, LoxClass};
use crate::base::scanner::{Token, TokenType};
use crate::base::stmt::Stmt;
use crate::base::visitor::Visitor;
use crate::interpreter::environment::Environment;
use crate::interpreter::runtime_error::RuntimeError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::stdout;
use std::io::Write;
use std::rc::Rc;
use uuid::Uuid;

pub struct Interpreter<'a> {
    globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
    locals: RefCell<HashMap<Uuid, usize>>,
    output_stream: Rc<RefCell<dyn Write + 'a>>,
}

impl<'a> Interpreter<'a> {
    pub fn new<OutputWriter>(output_stream: Rc<RefCell<OutputWriter>>) -> Self
    where
        OutputWriter: Write + 'a,
    {
        let globals = Rc::new(RefCell::new(Environment::new()));
        let env = Rc::clone(&globals);
        Self {
            globals,
            environment: env,
            locals: RefCell::new(HashMap::new()),
            output_stream,
        }
    }

    pub fn fork(&self, environment: Rc<RefCell<Environment>>) -> Self {
        Self {
            globals: Rc::clone(&self.globals),
            environment,
            locals: self.locals.clone(),
            output_stream: Rc::clone(&self.output_stream),
        }
    }

    pub fn interpret(&self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    pub fn execute_block(&self, statements: &[Stmt]) -> Result<ExprResult, RuntimeError> {
        for statement in statements {
            if let Err(e) = self.execute(statement) {
                return match e {
                    RuntimeError::Return { ret_val } => Ok(ret_val),
                    _ => Err(e),
                };
            }
        }

        Ok(ExprResult::none())
    }

    pub fn define(&self, name: &Token, value: ExprResult) {
        self.environment.borrow_mut().define(name, value);
    }

    pub fn resolve(&self, uuid: &Uuid, depth: usize) {
        self.locals.borrow_mut().insert(uuid.to_owned(), depth);
    }

    fn execute(&self, stmt: &Stmt) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    fn evaluate(&self, expr: &Expr) -> Result<ExprResult, RuntimeError> {
        expr.accept(self)
    }

    fn lookup_variable(&self, name: &Token, uuid: &Uuid) -> Result<ExprResult, RuntimeError> {
        if let Some(distance) = self.locals.borrow().get(uuid) {
            self.environment.borrow().get_at(distance.to_owned(), name)
        } else {
            self.globals.borrow().get(name)
        }
    }
}

impl Default for Interpreter<'_> {
    fn default() -> Self {
        Interpreter::new(Rc::new(RefCell::new(stdout())))
    }
}

impl Visitor<Expr, ExprResult, RuntimeError> for Interpreter<'_> {
    fn visit(&self, input: &Expr) -> Result<ExprResult, RuntimeError> {
        match input {
            Expr::Assign { uuid, name, value } => {
                let v = self.evaluate(value)?;

                if let Some(distance) = self.locals.borrow().get(uuid) {
                    self.environment
                        .borrow_mut()
                        .assign_at(distance.to_owned(), name, &v)?;
                } else {
                    self.globals.borrow_mut().assign(name, &v)?;
                }

                Ok(v)
            }
            Expr::Binary {
                uuid: _uuid,
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                match &operator.token_type {
                    TokenType::Greater => match (left, right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::boolean(v1 > v2))
                        }
                        _ => Err(RuntimeError::NumberExpected {
                            line: operator.line,
                        }),
                    },
                    TokenType::GreaterEqual => match (left, right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::boolean(v1 >= v2))
                        }
                        _ => Err(RuntimeError::NumberExpected {
                            line: operator.line,
                        }),
                    },
                    TokenType::Less => match (left, right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::boolean(v1 < v2))
                        }
                        _ => Err(RuntimeError::NumberExpected {
                            line: operator.line,
                        }),
                    },
                    TokenType::LessEqual => match (left, right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::boolean(v1 <= v2))
                        }
                        _ => Err(RuntimeError::NumberExpected {
                            line: operator.line,
                        }),
                    },
                    TokenType::BangEqual => Ok(ExprResult::boolean(left != right)),
                    TokenType::EqualEqual => Ok(ExprResult::boolean(left == right)),
                    TokenType::Minus => match (left, right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::number(v1 - v2))
                        }
                        _ => Err(RuntimeError::NumberExpected {
                            line: operator.line,
                        }),
                    },
                    TokenType::Slash => match (left, right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::number(v1 / v2))
                        }
                        _ => Err(RuntimeError::NumberExpected {
                            line: operator.line,
                        }),
                    },
                    TokenType::Star => match (left, right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::number(v1 * v2))
                        }
                        _ => Err(RuntimeError::NumberExpected {
                            line: operator.line,
                        }),
                    },
                    TokenType::Plus => match (left, right) {
                        (ExprResult::Number(v1), ExprResult::Number(v2)) => {
                            Ok(ExprResult::number(v1 + v2))
                        }
                        (ExprResult::String(v1), ExprResult::String(v2)) => {
                            Ok(ExprResult::string(v1.clone() + v2.clone().as_str()))
                        }
                        _ => Err(RuntimeError::NumberExpected {
                            line: operator.line,
                        }),
                    },
                    _ => Err(RuntimeError::InvalidValue {
                        line: operator.line,
                    }),
                }
            }
            Expr::Call {
                uuid: _uuid,
                paren,
                callee,
                arguments,
            } => {
                let call = self.evaluate(callee)?;

                if let ExprResult::Function(function) = call {
                    if arguments.len() != function.arity() {
                        return Err(RuntimeError::NonMatchingNumberOfArguments {
                            line: paren.line,
                        });
                    }

                    let mut args = vec![];
                    for argument in arguments {
                        args.push(self.evaluate(argument)?);
                    }

                    function.call(self, &args)
                } else if let ExprResult::Class(class) = call {
                    class.call(self, &[])
                } else {
                    Err(RuntimeError::UndefinedCallable { line: paren.line })
                }
            }
            Expr::Get {
                uuid: _uuid,
                object,
                name,
            } => {
                let obj = self.evaluate(object)?;
                if let ExprResult::Instance(instance) = obj {
                    instance.get(name)
                } else {
                    Err(RuntimeError::InvalidPropertyAccess { line: name.line })
                }
            }
            Expr::Grouping {
                uuid: _uuid,
                expression,
            } => self.evaluate(expression),
            Expr::Literal { uuid: _uuid, value } => match value {
                LiteralValue::Number(value) => Ok(ExprResult::number(value.into_inner())),
                LiteralValue::String(value) => Ok(ExprResult::string(value.clone())),
                LiteralValue::Boolean(value) => Ok(ExprResult::boolean(*value)),
                LiteralValue::None => Ok(ExprResult::none()),
            },
            Expr::Logical {
                uuid: _uuid,
                left,
                operator,
                right,
            } => {
                let left_expr = self.evaluate(left)?;

                if operator.token_type == TokenType::Or {
                    if left_expr.is_truthy() {
                        return Ok(left_expr);
                    }
                } else if !left_expr.is_truthy() {
                    return Ok(left_expr);
                }

                self.evaluate(right)
            }
            Expr::Set {
                uuid: _uuid,
                object,
                name,
                value,
            } => {
                let obj = self.evaluate(object)?;
                if let ExprResult::Instance(instance) = obj {
                    let v = self.evaluate(value)?;
                    instance.set(name, v.to_owned());

                    Ok(v)
                } else {
                    Err(RuntimeError::InvalidFieldAccess { line: name.line })
                }
            }
            Expr::This { uuid, keyword } => self.lookup_variable(keyword, uuid),
            Expr::Unary {
                uuid: _uuid,
                operator,
                right,
            } => {
                let right = self.evaluate(right)?;

                match &operator.token_type {
                    TokenType::Minus => match right {
                        ExprResult::Number(value) => Ok(ExprResult::number(-value)),
                        _ => Err(RuntimeError::NumberExpected {
                            line: operator.line,
                        }),
                    },
                    TokenType::Bang => Ok(ExprResult::boolean(!right.is_truthy())),
                    _ => Err(RuntimeError::InvalidValue {
                        line: operator.line,
                    }),
                }
            }
            Expr::Variable { uuid, name } => self.lookup_variable(name, uuid),
        }
    }
}

impl Visitor<Stmt, (), RuntimeError> for Interpreter<'_> {
    fn visit(&self, input: &Stmt) -> Result<(), RuntimeError> {
        match input {
            Stmt::Block { statements } => {
                let scoped_interpreter =
                    self.fork(Environment::new_enclosing(Rc::clone(&self.environment)));
                scoped_interpreter.execute_block(statements)?;
            }
            Stmt::Class { name, methods } => {
                self.environment
                    .borrow_mut()
                    .define(name, ExprResult::none());

                let mut functions = HashMap::new();
                for method in methods {
                    if let Stmt::Function { name, params, body } = method {
                        let function = LoxFunction::new(
                            *name.to_owned(),
                            params.to_owned(),
                            body.to_owned(),
                            Rc::clone(&self.environment),
                        );

                        functions.insert(name.lexeme.to_owned(), function);
                    }
                }

                let class = LoxClass::new(*name.to_owned(), functions);

                self.environment
                    .borrow_mut()
                    .assign(name, &ExprResult::class(class))?;
            }
            Stmt::Expression { expression } => {
                self.evaluate(expression)?;
            }
            Stmt::Function { name, params, body } => {
                self.environment
                    .borrow_mut()
                    .define(name, ExprResult::none());

                let function = LoxFunction::new(
                    *name.to_owned(),
                    params.to_owned(),
                    body.to_owned(),
                    Rc::clone(&self.environment),
                );

                self.environment
                    .borrow_mut()
                    .assign(name, &ExprResult::function(function.clone()))?;
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_result = self.evaluate(condition)?;

                if condition_result.is_truthy() {
                    self.execute(then_branch)?
                } else if let Some(branch) = *else_branch.to_owned() {
                    self.execute(&branch)?
                }
            }
            Stmt::Print { expression } => {
                let value = self.evaluate(expression)?;
                let mut stream = self.output_stream.borrow_mut();
                writeln!(stream, "{}", value).map_err(|_| RuntimeError::OutputError)?;
                stream.flush().map_err(|_| RuntimeError::OutputError)?;
            }
            Stmt::Return {
                keyword: _keyword,
                value,
            } => {
                if let Some(expr) = *value.to_owned() {
                    let ret_val = self.evaluate(&expr)?;
                    return Err(RuntimeError::Return { ret_val });
                }
            }
            Stmt::Var { name, initializer } => {
                let value = self.evaluate(initializer)?;
                self.environment.borrow_mut().define(name, value);
            }
            Stmt::While { condition, body } => {
                while self.evaluate(condition)?.is_truthy() {
                    self.execute(body)?;
                }
            }
        }

        Ok(())
    }
}
