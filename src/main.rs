use std::io::{self, Write};
use std::{env, fs, process::exit};

mod parser;
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
            let (parsed_output, status_code) = parser::parse(&mut tokens.into_iter(), false);
            for parsed_line in parsed_output {
                println!("{}", parsed_line);
            }

            exit(status_code);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
