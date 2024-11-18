use codecrafters_interpreter::{Expr, Token};
use std::io::{self, Write};
use std::{env, fs, process::exit};

mod parser;
use parser::Parser;
mod evaluate;
mod runner;
mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

    let command = &args[1];
    let filename = &args[2];
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });
    let mut status_code = 0;

    match command.as_str() {
        "tokenize" => {
            let (tokens, status_code) = tokenize(file_contents);
            for token in tokens {
                println!("{}", token);
            }

            exit(status_code);
        }
        "parse" => {
            if let Ok(expressions) = parse(file_contents) {
                for expr in expressions {
                    println!("{}", expr);
                }
            } else {
                status_code = 65;
            };

            exit(status_code);
        }
        "evaluate" => {
            if let Ok(res) = evaluate(file_contents) {
                res.print();
            } else {
                status_code = 70;
            }

            exit(status_code);
        }
        "run" => {
            if let Ok(_) = run(file_contents) {
            } else {
                status_code = 70;
            }

            exit(status_code);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn tokenize(file_contents: String) -> (Vec<Token>, i32) {
    tokenizer::tokenize(file_contents)
}

fn parse(file_contents: String) -> Result<Vec<Expr>, ()> {
    let (tokens, _) = tokenize(file_contents);
    let mut parser = Parser::new(tokens);
    if let Ok(expressions) = parser.parse() {
        return Ok(expressions);
    }

    Err(())
}

fn evaluate(file_contents: String) -> Result<Token, ()> {
    let expressions = parse(file_contents)?;
    for expr in expressions {
        if let Ok(token) = evaluate::evaluate(expr) {
            return Ok(token);
        } else {
            break;
        }
    }

    Err(())
}

fn run(file_contents: String) -> Result<(), ()> {
    let expressions = parse(file_contents)?;
    if let Ok(()) = runner::run(expressions) {
        return Ok(());
    }

    Err(())
}
