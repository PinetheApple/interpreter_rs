use std::io::{self, Write};
use std::{env, fs, process::exit};

mod parser;
use parser::Parser;
mod evaluate;
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
            let (tokens, status_code) = tokenizer::tokenize(file_contents);
            for token in tokens {
                println!("{}", token);
            }

            exit(status_code);
        }
        "parse" => {
            let (tokens, _) = tokenizer::tokenize(file_contents);
            let mut parser = Parser::new(tokens);
            if let Ok(parsed_expr) = parser.parse() {
                println!("{}", parsed_expr);
            } else {
                status_code = 65;
            };

            exit(status_code);
        }
        "evaluate" => {
            let (tokens, _) = tokenizer::tokenize(file_contents);
            let mut parser = Parser::new(tokens);
            if let Ok(expr) = parser.parse() {
                let res = evaluate::evaluate(expr);
                res.print();
            } else {
                status_code = 65;
            }

            exit(status_code);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
