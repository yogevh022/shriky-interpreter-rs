use crate::runtime::environment::utils::Counter;
use crate::runtime::interpreter;
use crate::runtime::values::*;
mod lexer;
mod parser;
mod runtime;

fn main() {
    let source: String = String::from("true && true");
    let mut lex = lexer::Lexer::new(&source);
    let mut parser = parser::Parser::new(&mut lex);
    let ast = parser.parse();
    for i  in ast.iter() {
        println!("{:?}", i);
    }
}
