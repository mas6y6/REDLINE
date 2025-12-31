use std::env;
use std::fs;
use std::process;

mod codegen;
mod lexer;
mod parser;
mod ast;

use lexer::{Lexer};
use parser::{Parser, ParserError};
use ast::{Program};

use codegen::generate;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: redline-core <file.rl>");
        process::exit(1);
    }

    let content = match fs::read_to_string(&args[1]) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file [{}]: {}", &args[1], e);
            process::exit(1);
        }
    };

    let tokens = match Lexer::new(content).tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer Error: {}", e);
            process::exit(1);
        }
    };

    let program = match Parser::new(&tokens).parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parser Error: {}", e);
            process::exit(1);
        }
    };

    match generate(&program) {
        Ok(cpp_code) => {
            print!("{}", cpp_code);
        }
        Err(e) => {
            eprintln!("Codegen Error: {}", e);
            process::exit(1);
        }
    }
}

