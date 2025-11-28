use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use laminax_kpl::{Lexer, Parser, Interpreter, KyaroError};

fn run_repl() {
    println!("Laminax Kyaro Programming Language v1.0 (Rust)");
    println!("By Laminax (https://laminax.org)");
    println!("Type 'exit()' to quit\n");
    
    let mut interpreter = Interpreter::new();
    
    loop {
        print!("kyaro> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let line = input.trim();
                
                if line.is_empty() {
                    continue;
                }
                
                if line == "exit()" {
                    println!("Goodbye!");
                    break;
                }
                
                match execute_code(line, &mut interpreter) {
                    Ok(Some(result)) => {
                        println!("{}", result.to_string());
                    }
                    Ok(None) => {}
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
}

fn run_file(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(source) => {
            let mut interpreter = Interpreter::new();
            if let Err(e) = execute_code(&source, &mut interpreter) {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: Could not read file '{}': {}", filename, e);
            process::exit(1);
        }
    }
}

fn execute_code(source: &str, interpreter: &mut Interpreter) -> Result<Option<laminax_kpl::Value>, KyaroError> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    interpreter.interpret(ast)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => run_repl(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: {} [filename.kyaro]", args[0]);
            process::exit(1);
        }
    }
}
