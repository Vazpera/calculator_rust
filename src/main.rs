pub mod lexer;
pub mod parser;
pub mod token;

use parser::Parser;
fn main() {
    // base equation A=P*(1+r/n)^(n*t)
    let p = Parser::new()
        .input("1*(1+1/999)^(999*1)".to_owned())
        .tokenize()
        .shunt()
        .evaluate();
    println!("{}", p.stack[0]);
}
