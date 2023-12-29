use std::cell::RefCell;
use std::fs;
use std::io::Write;

use clap::Parser as ClapParser;
use rlox::base::parser::Parser;

use rlox::base::scanner::Scanner;
use rlox::interpreter::interpreter::Interpreter;

struct LoxEnvironment {
    interpreter: Interpreter,
}

impl LoxEnvironment {
    fn new() -> Self {
        LoxEnvironment {
            interpreter: Interpreter::new(),
        }
    }

    fn run_string(&self, input: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut scanner = Scanner::new(input);
        let tokens = match scanner.scan_tokens() {
            Ok(tokens) => Ok(tokens),
            Err(error) => {
                eprintln!("{}", error);
                Err(error)
            }
        }?;

        let parser = Parser {
            tokens,
            current: RefCell::new(0),
        };
        let statements = match parser.parse() {
            Ok(statements) => Ok(statements),
            Err(error) => {
                eprintln!("{}", error);
                Err(error)
            }
        }?;

        self.interpreter.interpret(statements);

        Ok(())
    }

    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Lox interpreter...");
        loop {
            print!("> ");
            let _ = std::io::stdout().flush();
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Unable to read user input!");

            self.run_string(input)?;
        }
    }

    fn run_file(&self, script_file: String) -> Result<(), Box<dyn std::error::Error>> {
        let script_content = fs::read_to_string(script_file)?;

        self.run_string(script_content)
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
    let environment = LoxEnvironment::new();

    match args.script {
        Some(script_file) => environment.run_file(script_file),
        None => environment.run(),
    }
}
