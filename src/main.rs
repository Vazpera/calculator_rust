pub mod lexer;
pub mod parser;
pub mod token;
use parser::Parser;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please enter a formula!");
    } else {
        let p = Parser::new()
            .input(args[1].to_owned())
            .tokenize()
            .shunt()
            .evaluate();
    }
}
