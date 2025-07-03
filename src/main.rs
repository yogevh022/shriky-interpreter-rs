mod compiler;
mod lexer;
mod parser;
mod utils;

fn main() {
    let source: String = String::from("a=2");
    let mut lex = lexer::Lexer::new(&source);
    let mut parser = parser::Parser::new(&mut lex);
    let ast = parser.parse();
    for a in ast.iter() {
        println!("{:?}", a);
    }
    let mut compiler = compiler::Compiler::new();
    let bytecode = compiler.compile_expr(ast.get(0).unwrap().clone());
    println!("bytecode: {:?}", bytecode);
}
