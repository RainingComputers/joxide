use std::path::Path;

use crate::{diagnostic, lexer, parser::ParseError};

fn get_line(content: &str, line_number: usize) -> Option<&str> {
    for (line_no, line) in content.split_terminator('\n').enumerate() {
        if line_no == line_number {
            return Some(line);
        }
    }

    None
}

fn eprint_location(token: &lexer::Token, content: &str) {
    let line = match get_line(content, token.line) {
        None => return,
        Some(line) => line,
    };

    let hint_carrot = " ".repeat(token.col) + "^";

    eprintln!("{}", line);
    eprintln!("{}", hint_carrot);
}

pub fn eprint_parse_error(parse_error: ParseError, content: &str, file_path: &Path) {
    match parse_error.token {
        Some(token) => {
            eprintln!(
                "At {}:{}:{}",
                file_path.display(),
                token.line + 1,
                token.col + 1
            );
            eprint_location(token, content);
        }
        None => eprintln!("At {}", file_path.display()),
    }

    eprintln!("{}", diagnostic::get_message(&parse_error));
}
