use codecrafters_interpreter::{Expr, Token};
use std::io::{self, Write};
use std::{env, fs, process::exit};

mod parser;
use parser::Parser;
mod evaluate;
use evaluate::{Eval, Stateless};
mod runner;
use runner::State;
mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

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
            match parse_expression(file_contents) {
                Ok(expr) => println!("{}", expr),
                Err(_) => status_code = 65,
            }

            exit(status_code);
        }
        "evaluate" => {
            match evaluate(file_contents) {
                Ok(token) => token.print(),
                Err(code) => status_code = code,
            }

            exit(status_code);
        }
        "run" => {
            match run(file_contents) {
                Err(code) => status_code = code,
                _ => {}
            }

            exit(status_code);
        }
        "test-parse" => {
            match parse(file_contents) {
                Ok(exprs) => {
                    for expr in exprs {
                        println!("{expr}");
                    }
                }
                Err(code) => status_code = code,
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

fn parse(file_contents: String) -> Result<Vec<Expr>, i32> {
    let (tokens, status_code) = tokenize(file_contents);
    if status_code != 0 {
        return Err(65);
    }
    let mut parser = Parser::new(tokens);
    if let Ok(expressions) = parser.parse() {
        return Ok(expressions);
    }

    Err(65)
}

fn parse_expression(file_contents: String) -> Result<Expr, ()> {
    let (tokens, _) = tokenize(file_contents);
    let mut parser = Parser::new(tokens);

    parser.parse_assignment()
}

fn evaluate(file_contents: String) -> Result<Token, i32> {
    if let Ok(expr) = parse_expression(file_contents) {
        if let Ok(token) = Stateless::evaluate(&mut Stateless, expr) {
            return Ok(token);
        }
    }

    Err(70)
}

fn run(file_contents: String) -> Result<(), i32> {
    let expressions = parse(file_contents)?;
    let mut program_state = State::new();
    if let Ok(_) = program_state.run(expressions) {
        return Ok(());
    }

    Err(70)
}
