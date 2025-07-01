use crate::runtime::interpreter;
use crate::runtime::values::*;
mod lexer;
mod parser;
mod runtime;
mod compiler;
fn main() {
    let source: String = String::from("a = 3");
    let mut lex = lexer::Lexer::new(&source);
    let mut parser = parser::Parser::new(&mut lex);
    let ast = parser.parse();
    let compiler = compiler::Compiler::new(ast);
    let bytecode = compiler.compile();
}
