use crate::lexer::TokenKind;

mod lexer;

fn main() {
    let source: String = String::from("a = 4; a = 4 + 4; a();");
    let mut lex = lexer::Lexer::new(&source);
    let mut current_token: lexer::Token = lex.next();
    while current_token.kind != TokenKind::EOF {
        println!("token: {}", current_token.value);
        current_token = lex.next();
    }
}
