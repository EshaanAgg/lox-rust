use std::env;
use std::fs;
use std::io::{stderr, Write};

mod lexer;
use lexer::lexer::Lexer;

const EXIT_FILE_ERROR: i32 = 1;
const EXIT_LEXICAL_ERROR: i32 = 65;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(stderr(), "Usage: {} <command> <filename>", args[0])
            .expect("Failed to write to stderr");
        std::process::exit(EXIT_FILE_ERROR);
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

            invalid_tokens.iter().for_each(|token| {
                println!("{}", token.tokenized_string());
            });
            valid_tokens.iter().for_each(|token| {
                println!("{}", token.tokenized_string());
            });

            if !invalid_tokens.is_empty() {
                std::process::exit(EXIT_LEXICAL_ERROR);
            }
        }

        _ => {
            writeln!(stderr(), "Unknown command: {}", command).expect("Failed to write to stderr");
        }
    }
}
