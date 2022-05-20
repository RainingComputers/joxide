enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}

#[derive(Debug)]
pub struct ParseError {
    line: usize,
    col: usize,
}

impl ParseError {
    pub fn new(line: usize, col: usize) -> ParseError {
        ParseError {
            line: line,
            col: col,
        }
    }
}
