use rlox_lib::base::parser::Parser;
use rlox_lib::base::scanner::Scanner;
use rlox_lib::interpreter::interpreter::Interpreter;
use rlox_lib::interpreter::resolver::Resolver;
use std::cell::RefCell;
use std::rc::Rc;

pub fn interpret(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let buf = Rc::new(RefCell::new(Vec::new()));
    let interpreter = Rc::new(Interpreter::new(Rc::clone(&buf)));

    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens()?;

    let parser = Parser::new(tokens);
    let statements = parser.parse()?;

    let resolver = Resolver::new(Rc::clone(&interpreter));
    resolver.resolve_stmts(&statements)?;

    interpreter.interpret(&statements)?;

    let output = std::str::from_utf8(buf.borrow().as_slice())
        .unwrap()
        .to_string();

    Ok(output)
}
