use crate::lexer;

pub fn get_line(content: &str, line_number: usize) -> Option<&str> {
    for (line_no, line) in content.split_terminator('\n').enumerate() {
        if line_no == line_number {
            return Some(line);
        }
    }

    return None;
}

pub fn print_location(content: &str, token: &lexer::Token) {
    let line = match get_line(content, token.line) {
        None => return,
        Some(line) => line,
    };

    let hint_carrot = " ".repeat(token.col) + "^";

    println!("{}", line);
    println!("{}", hint_carrot);
}
