use crate::runtime::interpreter;
use crate::runtime::values::*;
mod compiler;
mod lexer;
mod parser;
mod runtime;
fn main() {
    let source: String = String::from("a = 3");
    let mut lex = lexer::Lexer::new(&source);
    let mut parser = parser::Parser::new(&mut lex);
    let ast = parser.parse();
    let compiler = compiler::Compiler::new(ast);
    let bytecode = compiler.compile();
}
