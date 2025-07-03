use crate::compiler::Compiler;
use crate::lexer::TokenKind;

mod compiler;
mod lexer;
mod parser;
mod utils;

fn main() {
    // let source: String = String::from("fn rage(arg1, arg2) {}");
    let source = std::fs::read_to_string("input/pik.txt").unwrap();
    let mut lex = lexer::Lexer::new(&source);
    let mut parser = parser::Parser::new(&mut lex);
    let ast = parser.parse(TokenKind::EOF);
    let code_obj = Compiler::compile(ast);
    println!("bytecode: {:?}", code_obj.operations);
    println!(
        "hex: {:?}",
        code_obj
            .operations
            .iter()
            .map(|item| item.hex())
            .collect::<Vec<String>>()
            .join(" ")
    );
    println!("constants: {:?}", code_obj.constants)
}
