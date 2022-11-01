extern crate argh;
use crate::args::JoxideSubcommand;
use std::process::ExitCode;

mod args;
mod diagnostic;
mod formatter;
mod lexer;
mod parser;
mod pretty;

fn main() -> ExitCode {
    let args: args::JoxideArgs = argh::from_env();

    let file_path = match args.sub_command {
        JoxideSubcommand::Validate(ref validate_args) => &validate_args.file,
        JoxideSubcommand::Format(ref format_args) => &format_args.file,
    };

    let raw = match std::fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(err) => {
            println!("Unable to open file, reason: {}", err.to_string());
            return ExitCode::FAILURE;
        }
    };

    let tokens = lexer::lex(&raw);

    let value = match parser::parse(&tokens) {
        Ok(value) => value,
        Err(parse_error) => {
            if let Some(token) = parse_error.token {
                println!("At {}:{}:{}", file_path, token.line + 1, token.col + 1);
                pretty::print_location(&raw, token);
            }

            println!("{}", diagnostic::get_message(&parse_error));
            return ExitCode::FAILURE;
        }
    };

    if let JoxideSubcommand::Format(ref format_args) = args.sub_command {
        let formatted = formatter::format_json(value, format_args.indent_length);

        if format_args.write {
            if let Err(err) = std::fs::write(file_path, formatted) {
                println!("Unable to write to file, reason: {}", err.to_string());
                return ExitCode::FAILURE;
            }
        } else {
            println!("{}", formatted);
        }
    }

    return ExitCode::SUCCESS;
}
