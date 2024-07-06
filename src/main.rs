use std::env;
use std::fs;
use std::io::{stderr, Write};

mod lexer;
use lexer::lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(stderr(), "Usage: {} <command> <filename>", args[0])
            .expect("Failed to write to stderr");
        std::process::exit(1);
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|err| {
        writeln!(stderr(), "Failed to read file {}: {}", filename, err)
            .expect("Failed to write to stderr");
        String::new()
    });
    let mut lexer = Lexer::new(&file_contents);

    match command.as_str() {
        "tokenize" => {
            let (valid_tokens, invalid_tokens) = lexer.get_tokens();
            for token in invalid_tokens {
                println!("{}", token.tokenized_string());
            }
            for token in valid_tokens {
                println!("{}", token.tokenized_string());
            }
        }

        _ => {
            writeln!(stderr(), "Unknown command: {}", command).expect("Failed to write to stderr");
        }
    }
}
