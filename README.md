[![progress-banner](https://backend.codecrafters.io/progress/interpreter/27549338-6d45-47a1-8fe0-62de6b03a211)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

These are my solutions to the ["Build Your Own Build your own Interpreter" Challenge](https://app.codecrafters.io/courses/interpreter/overview) by the [CodeCrafters](https://codecrafters.io) team.


This challenge follows the book [Crafting Interpreters](https://craftinginterpreters.com/) by Robert Nystrom. While the original book is written in Java and C, this challenge develops the interpreter in Rust. To support testing the same, we basically make a CLI over the core interpreter code, so that the same recognizes commands like `tokenize` and prints the result to the standard streams. 

# Supported Commands

## tokenize
The interpreter supports the `tokenize` command with usage:
```bash
./your_program.sh tokenize <path_to_source_file>
```

This commands tokenizes the source file and prints the tokens to the standard output. Each token is printed on a new line, and has the format:
```
<TokenType> <Lexeme> <Literal>
```
where:
- `TokenType` is the type of the token
- `Lexeme` is the actual text of the token
- `Literal` is the value of the token. It is mostly 'null', but for literals like numbers, strings, etc., it is the actual value that the token represents.

All the parsing errors are logged to the standard error stream, with their line numbers.
