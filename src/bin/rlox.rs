use std::cell::RefCell;
use std::fs;
use std::io::Write;

use clap::Parser as ClapParser;
use rlox::base::parser::Parser;

use rlox::base::scanner::Scanner;
use rlox::interpreter::interpreter::Interpreter;

#[derive(ClapParser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg()]
    script: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.script {
        Some(script_file) => run_file(script_file),
        None => run(),
    }
}

fn run_string(input: String) -> Result<(), Box<dyn std::error::Error>> {
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
    let expression = match parser.parse() {
        Ok(expression) => Ok(expression),
        Err(error) => {
            eprintln!("{}", error);
            Err(error)
        }
    }?;

    let interpreter = Interpreter {};
    match interpreter.evaluate(&expression) {
        Ok(result) => {
            println!("{}", result);
        }
        Err(error) => {
            eprintln!("{}", error);
        }
    };

    Ok(())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Lox interpreter...");
    loop {
        print!("> ");
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Unable to read user input!");

        run_string(input)?;
    }
}

fn run_file(script_file: String) -> Result<(), Box<dyn std::error::Error>> {
    let script_content = fs::read_to_string(script_file)?;

    run_string(script_content)
}
