use std::env;

mod diagnostic;
mod lexer;
mod parser;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.get(1) {
        Some(path) => path,
        None => {
            println!("Invalid CLI arguments");
            return Err(());
        }
    };

    let raw = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            println!("Unable to open file, reason: {:?}", err);
            return Err(());
        }
    };

    let tokens = lexer::lex(&raw);
    match parser::parse(&tokens) {
        Ok(value) => value,
        Err(parse_error) => {
            if let Some(token) = parse_error.token {
                println!("At {}:{}:{}", file_path, token.line + 1, token.col + 1);
            }

            println!("{}", diagnostic::get_message(&parse_error));
            return Err(());
        }
    };

    return Ok(());
}
