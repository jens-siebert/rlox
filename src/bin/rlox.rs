use std::cell::RefCell;
use std::fs;

use clap::Parser as ClapParser;
use thiserror::Error;
use rlox::base::parser::{Expr, ExprRef, Parser, Visitor};

use rlox::base::scanner::Scanner;

#[derive(ClapParser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg()]
    script: Option<String>,
}

#[derive(Error, Debug)]
enum LoxError {
    #[error("No script file was given!")]
    NoScriptFile,
}

struct AstPrinter {}

impl AstPrinter {
    fn new() -> Self {
        AstPrinter {}
    }

    fn print(&self, expr: ExprRef) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, expressions: &[&ExprRef]) -> String {
        let mut result = String::new();
        result.push('(');
        result.push_str(name);
        for expr in expressions {
            result.push(' ');
            result.push_str(expr.accept(self).as_str());
        }
        result.push(')');

        result
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary { left, operator, right } => {
                self.parenthesize(&operator.lexeme, &[&left, &right])
            }
            Expr::Grouping { expression } => self.parenthesize("group", &[&expression]),
            Expr::Literal { value } => match &value {
                None => String::from("nil"),
                Some(v) => v.to_string(),
            },
            Expr::Unary { operator, right } => self.parenthesize(&operator.lexeme, &[&right]),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.script {
        Some(script_file) => run(script_file),
        None => Err(Box::new(LoxError::NoScriptFile)),
    }
}

fn run(script_file: String) -> Result<(), Box<dyn std::error::Error>> {
    let script_content = fs::read_to_string(script_file)?;

    let mut scanner = Scanner::new(script_content);
    let tokens = match scanner.scan_tokens() {
        Ok(tokens) => Ok(tokens),
        Err(error) => {
            eprintln!("{}", error);
            Err(error)
        }
    }?;

    let parser = Parser { tokens, current: RefCell::new(0) };
    let expression = match parser.parse() {
        Ok(expression) => { Ok(expression) }
        Err(error) => {
            eprintln!("{}", error);
            Err(error)
        }
    }?;

    let ast_printer = AstPrinter::new();
    println!("{}", ast_printer.print(expression));

    Ok(())
}
