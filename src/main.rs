mod lexer;
mod parser;

fn main() {
    let tokens = lexer::lex("hello world");
    let _value = parser::parse(&tokens);
    println!("Hello, world!");
}
