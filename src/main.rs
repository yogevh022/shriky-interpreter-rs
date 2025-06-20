mod lexer;
mod parser;
mod runtime;

fn main() {
    let source: String = String::from("a = false");
    let mut lex = lexer::Lexer::new(&source);
    let mut parser = parser::Parser::new(&mut lex);
    let ast =  parser.parse();
    for node in ast {
        println!("{:?}", node);
    }
}
