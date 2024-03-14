use std::cell::RefCell;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Write};
use std::rc::Rc;

use clap::Parser as ClapParser;

use rlox_lib::base::parser::Parser;
use rlox_lib::base::scanner::Scanner;
use rlox_lib::interpreter::interpreter::Interpreter;
use rlox_lib::interpreter::resolver::Resolver;

struct LoxRuntime<'a> {
    interpreter: Rc<Interpreter<'a>>,
}

impl LoxRuntime<'_> {
    fn new() -> Self {
        LoxRuntime {
            interpreter: Rc::new(Interpreter::new(Rc::new(RefCell::new(stdout())))),
        }
    }

    fn run(&self, input: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut scanner = Scanner::new(input);
        let tokens = scanner
            .scan_tokens()
            .map_err(|error| eprintln!("{}", error))
            .unwrap_or_default();

        let parser = Parser::new(tokens);
        let statements = parser
            .parse()
            .map_err(|error| eprintln!("{}", error))
            .unwrap_or_default();

        let resolver = Resolver::new(Rc::clone(&self.interpreter));
        if let Err(error) = resolver.resolve_stmts(&statements) {
            eprintln!("{}", error)
        };

        if let Err(error) = self.interpreter.interpret(&statements) {
            eprintln!("{}", error)
        }

        Ok(())
    }

    fn run_prompt(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Lox interpreter...");
        loop {
            print!("> ");
            let _ = stdout().flush();
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Unable to read user input");

            if let Err(error) = self.run(input.as_str()) {
                eprintln!("{}", error)
            }
        }
    }

    fn run_file(&self, script_file: String) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(script_file).expect("Unable to read input file");
        let buffer = BufReader::new(file);

        for line in buffer.lines().map_while(Result::ok) {
            if let Err(error) = self.run(line.as_str()) {
                eprintln!("{}", error)
            }
        }

        Ok(())
    }
}

#[derive(ClapParser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg()]
    script: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let environment = LoxRuntime::new();

    match args.script {
        Some(script_file) => environment.run_file(script_file),
        None => environment.run_prompt(),
    }
}
