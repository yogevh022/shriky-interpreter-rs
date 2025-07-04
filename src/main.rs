use crate::compiler::Compiler;
use crate::lexer::TokenKind;
mod compiler;
mod lexer;
mod parser;
mod runtime;
mod utils;

fn main() {
    // let source: String = String::from("fn rage(arg1, arg2) {}");
    let source = std::fs::read_to_string("input/pik.txt").unwrap();
    let mut lex = lexer::Lexer::new(&source);
    let mut parser = parser::Parser::new(&mut lex);
    let ast = parser.parse(TokenKind::EOF);
    let mut compiler = Compiler::new();
    let code_obj = compiler.compile(ast);
    let mut runtime = runtime::Runtime::new();
    runtime.print_code_object(code_obj.clone());
    runtime.run(code_obj);
}
