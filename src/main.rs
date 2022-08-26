use std::{env, process};

use parser::Parser;

use crate::{interpreter::Interpreter, lexer::Lexer};

mod interpreter;
mod lexer;
mod parser;

fn run(code: &str) -> Result<f64, String> {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.scan();
    let mut parser = Parser::new(&tokens);
    let interpreter = Interpreter::new(parser.parse()?);
    Ok(interpreter.run())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Expected exactly 1 argument but {} were given", args.len()-1);
        process::exit(1);
    }

    match run(&args[1]) {
        Ok(result) => println!(" \x1b[1;30m{}\x1b[0m = \x1b[1;32m{result}\x1b[1;0m", args[1]),
        Err(e) => {
            eprintln!("Could not compute `{}`: Error: {e}", args[1]);
            process::exit(1);
        }
    }
}
