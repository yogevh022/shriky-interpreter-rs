use crate::lexer::TokenKind;

mod lexer;
mod parser;

fn main() {
    let source: String = String::from("4 * (1 + 1)");
    let mut lex = lexer::Lexer::new(&source);
    let mut parser = parser::Parser::new(&mut lex);
    let ast =  parser.parse();
    for node in ast {
        println!("{:?}", node);
    }
}
