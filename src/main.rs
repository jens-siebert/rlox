use std::fs;

use clap::Parser;
use thiserror::Error;

use crate::scanner::Scanner;

mod scanner;

#[derive(Parser, Debug)]
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

    println!("{:#?}", tokens);

    Ok(())
}
