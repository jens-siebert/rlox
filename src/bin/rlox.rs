use clap::Parser as ClapParser;
use rlox::base::parser::Parser;
use rlox::base::scanner::Scanner;
use rlox::interpreter::interpreter::Interpreter;
use std::fs;
use std::io::Write;

struct LoxEnvironment {
    interpreter: Interpreter,
}

impl LoxEnvironment {
    fn new() -> Self {
        LoxEnvironment {
            interpreter: Interpreter::new(),
        }
    }

    fn run(&self, input: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut scanner = Scanner::new(input);
        let tokens = scanner.scan_tokens()?;

        let parser = Parser::new(tokens);
        let statements = parser.parse()?;

        self.interpreter.interpret(&statements)?;

        Ok(())
    }

    fn run_prompt(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Lox interpreter...");
        loop {
            print!("> ");
            let _ = std::io::stdout().flush();
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Unable to read user input");

            if let Err(error) = self.run(input) {
                eprintln!("{}", error)
            }
        }
    }

    fn run_file(&self, script_file: String) -> Result<(), Box<dyn std::error::Error>> {
        let script_content = fs::read_to_string(script_file).expect("Unable to read input file");
        self.run(script_content)
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
        None => environment.run_prompt(),
    }
}
