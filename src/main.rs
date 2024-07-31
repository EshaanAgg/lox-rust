use std::env;
use std::fs;
use std::io::{stderr, Write};

mod ast;
mod lexer;

use ast::interpreter::Interpreter;
use ast::printer::AstPrinter;
use ast::syntax_tree::SyntaxTree;
use lexer::lexer::Lexer;

const EXIT_FILE_ERROR: i32 = 1;
const EXIT_LEXICAL_ERROR: i32 = 65;
const RUNTIME_ERROR: i32 = 70;

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
            let mut has_lexical_error = false;

            lexer.get_tokens().iter().for_each(|token| {
                if token.is_error() {
                    writeln!(stderr(), "{}", token.tokenized_string())
                        .expect("Failed to write to stderr");
                    has_lexical_error = true;
                } else {
                    println!("{}", token.tokenized_string());
                }
            });

            if has_lexical_error {
                std::process::exit(EXIT_LEXICAL_ERROR);
            }
        }

        "parse" => {
            let tokens = lexer.get_tokens();
            let mut parser = SyntaxTree::new(tokens);

            match parser.expression() {
                Ok(expr) => println!("{}", AstPrinter::print(expr)),
                Err(err) => {
                    writeln!(stderr(), "[line {}] {}", err.line, err.message)
                        .expect("Failed to write to stderr");
                    std::process::exit(EXIT_LEXICAL_ERROR);
                }
            }
        }

        "evaluate" => {
            let tokens = lexer.get_tokens();
            let mut parser = SyntaxTree::new(tokens);
            let expr = parser.expression();

            if let Err(err) = expr {
                writeln!(stderr(), "[line {}] {}", err.line, err.message)
                    .expect("Failed to write to stderr");
                std::process::exit(EXIT_LEXICAL_ERROR);
            }

            let interpreter = Interpreter::new(expr.unwrap());
            match interpreter.evaluate() {
                Ok(val) => println!("{}", val),
                Err(err) => {
                    writeln!(stderr(), "{}", err).expect("Failed to write to stderr");
                    std::process::exit(RUNTIME_ERROR);
                }
            }
        }

        _ => {
            writeln!(stderr(), "Unknown command: {}", command).expect("Failed to write to stderr");
        }
    }
}
