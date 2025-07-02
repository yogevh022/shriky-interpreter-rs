use crate::runtime::interpreter;
use crate::runtime::values::*;
mod compiler;
mod lexer;
mod parser;
mod runtime;
mod utils;

fn main() {
    let source: String = String::from("a=b--");
    let mut lex = lexer::Lexer::new(&source);
    let mut parser = parser::Parser::new(&mut lex);
    let ast = parser.parse();
    for a in ast.iter() {
        println!("{:?}", a);
    }
    // let compiler = compiler::Compiler::new(ast);
    // let bytecode = compiler.compile();
}
