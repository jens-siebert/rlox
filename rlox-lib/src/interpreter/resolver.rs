use crate::base::expr::{Expr, ExprUuid};
use crate::base::scanner::Token;
use crate::base::stmt::Stmt;
use crate::base::visitor::Visitor;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::runtime_error::RuntimeError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
enum FunctionType {
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Clone, Debug, PartialEq)]
enum ClassType {
    None,
    Class,
}

pub struct Resolver<'a> {
    interpreter: Rc<Interpreter<'a>>,
    scopes: RefCell<Vec<HashMap<String, bool>>>,
    current_function_type: RefCell<FunctionType>,
    current_class_type: RefCell<ClassType>,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: Rc<Interpreter<'a>>) -> Self {
        Self {
            interpreter,
            scopes: RefCell::new(Vec::new()),
            current_function_type: RefCell::new(FunctionType::None),
            current_class_type: RefCell::new(ClassType::None),
        }
    }

    pub fn resolve_stmts(&self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        for statement in statements {
            self.resolve_stmt(statement)?
        }

        Ok(())
    }

    fn resolve_stmt(&self, statement: &Stmt) -> Result<(), RuntimeError> {
        statement.accept(self)
    }

    fn resolve_expr(&self, expression: &Expr) -> Result<(), RuntimeError> {
        expression.accept(self)
    }

    fn begin_scope(&self) {
        self.scopes.borrow_mut().push(HashMap::new());
    }

    fn end_scope(&self) {
        self.scopes.borrow_mut().pop();
    }

    fn declare(&self, name: &Token) -> Result<(), RuntimeError> {
        if let Some(scope) = self.scopes.borrow_mut().last_mut() {
            if scope.contains_key(&name.lexeme) {
                return Err(RuntimeError::VariableAlreadyDefinedInScope { line: name.line });
            } else {
                scope.insert(name.lexeme.to_owned(), false);
            }
        }

        Ok(())
    }

    fn define(&self, name: &Token) {
        if let Some(scope) = self.scopes.borrow_mut().last_mut() {
            scope.insert(name.lexeme.to_owned(), true);
        }
    }

    fn resolve_local(&self, expression: &dyn ExprUuid, name: &Token) -> Result<(), RuntimeError> {
        for i in (0..self.scopes.borrow().len()).rev() {
            if self
                .scopes
                .borrow()
                .get(i)
                .unwrap()
                .contains_key(&name.lexeme)
            {
                self.interpreter
                    .resolve(&expression.uuid(), self.scopes.borrow().len() - 1 - i);
                break;
            }
        }

        Ok(())
    }

    fn resolve_function(
        &self,
        statement: &Stmt,
        function_type: FunctionType,
    ) -> Result<(), RuntimeError> {
        if let Stmt::Function {
            name: _name,
            params,
            body,
        } = statement
        {
            let enclosing_function = self.current_function_type.replace(function_type);
            self.begin_scope();

            for param in params {
                self.declare(param)?;
                self.define(param);
            }

            self.resolve_stmts(body)?;

            self.end_scope();
            self.current_function_type.replace(enclosing_function);
        }

        Ok(())
    }
}

impl Visitor<Stmt, (), RuntimeError> for Resolver<'_> {
    fn visit(&self, input: &Stmt) -> Result<(), RuntimeError> {
        match input {
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve_stmts(statements)?;
                self.end_scope()
            }
            Stmt::Class {
                name,
                superclass,
                methods,
            } => {
                let enclosing_class = self.current_class_type.replace(ClassType::Class);

                self.declare(name)?;
                self.define(name);

                if let Some(sc) = superclass.as_ref() {
                    if let Expr::Variable {
                        uuid: _uuid,
                        name: sc_name,
                    } = sc
                    {
                        if name.lexeme == sc_name.lexeme {
                            return Err(RuntimeError::SuperclassSelfInheritance {
                                line: name.line,
                            });
                        }
                    }

                    self.resolve_expr(sc)?;
                }

                self.begin_scope();
                if let Some(scope) = self.scopes.borrow_mut().last_mut() {
                    scope.insert(String::from("this"), true);
                }

                for method in methods {
                    if let Stmt::Function {
                        name,
                        params: _params,
                        body: _body,
                    } = method
                    {
                        let declaration = if name.lexeme.eq("init") {
                            FunctionType::Initializer
                        } else {
                            FunctionType::Method
                        };

                        self.resolve_function(method, declaration)?;
                    }
                }

                self.end_scope();
                self.current_class_type.replace(enclosing_class);
            }
            Stmt::Expression { expression } => {
                self.resolve_expr(expression)?;
            }
            Stmt::Function {
                name,
                params: _params,
                body: _body,
            } => {
                self.declare(name)?;
                self.define(name);
                self.resolve_function(input, FunctionType::Function)?;
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(then_branch)?;
                if let Some(branch) = else_branch.as_ref() {
                    self.resolve_stmt(branch)?;
                }
            }
            Stmt::Print { expression } => {
                self.resolve_expr(expression)?;
            }
            Stmt::Return { keyword, value } => {
                if *self.current_function_type.borrow() == FunctionType::None {
                    return Err(RuntimeError::TopLevelReturn { line: keyword.line });
                }

                if let Some(expr) = value.as_ref() {
                    if *self.current_function_type.borrow() == FunctionType::Initializer {
                        return Err(RuntimeError::ReturnValueFromInitializer {
                            line: keyword.line,
                        });
                    }

                    self.resolve_expr(expr)?;
                }
            }
            Stmt::Var { name, initializer } => {
                self.declare(name)?;
                self.resolve_expr(initializer)?;
                self.define(name);
            }
            Stmt::While { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(body)?;
            }
        }

        Ok(())
    }
}

impl Visitor<Expr, (), RuntimeError> for Resolver<'_> {
    fn visit(&self, input: &Expr) -> Result<(), RuntimeError> {
        match input {
            Expr::Assign {
                uuid: _uuid,
                name,
                value,
            } => {
                self.resolve_expr(value)?;
                self.resolve_local(input, name)?;
            }
            Expr::Binary {
                uuid: _uuid,
                left,
                operator: _operator,
                right,
            } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            Expr::Call {
                uuid: _uuid,
                paren: _parent,
                callee,
                arguments,
            } => {
                self.resolve_expr(callee)?;
                for argument in arguments {
                    self.resolve_expr(argument)?;
                }
            }
            Expr::Get {
                uuid: _uuid,
                object,
                name: _name,
            } => {
                self.resolve_expr(object)?;
            }
            Expr::Grouping {
                uuid: _uuid,
                expression,
            } => {
                self.resolve_expr(expression)?;
            }
            Expr::Literal { .. } => {}
            Expr::Logical {
                uuid: _uuid,
                left,
                operator: _operator,
                right,
            } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            Expr::Set {
                uuid: _uuid,
                object,
                name: _name,
                value,
            } => {
                self.resolve_expr(value)?;
                self.resolve_expr(object)?;
            }
            Expr::This {
                uuid: _uuid,
                keyword,
            } => {
                if *self.current_class_type.borrow() == ClassType::None {
                    return Err(RuntimeError::ThisOutsideClass { line: keyword.line });
                }

                self.resolve_local(input, keyword)?;
            }
            Expr::Unary {
                uuid: _uuid,
                operator: _operator,
                right,
            } => {
                self.resolve_expr(right)?;
            }
            Expr::Variable { uuid: _uuid, name } => {
                if let Some(scope) = self.scopes.borrow().last() {
                    if let Some(definition) = scope.get(&name.lexeme) {
                        if !definition {
                            return Err(RuntimeError::VariableNotDefined { line: name.line });
                        }
                    }
                }

                self.resolve_local(input, name)?;
            }
        }

        Ok(())
    }
}
