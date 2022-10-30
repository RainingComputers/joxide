mod lexer;
mod parser;

fn main() {
    let tokens = lexer::lex("hello world");
    let _ = parser::parse(&tokens);
    println!("Hello, world!");
}
