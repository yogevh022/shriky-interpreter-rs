use crate::runtime::values::*;
use crate::runtime::environment::utils::Counter;
mod lexer;
mod parser;
mod runtime;

fn main() {
    let source: String = String::from("a = false");
    let mut lex = lexer::Lexer::new(&source);
    let mut parser = parser::Parser::new(&mut lex);
    let ast =  parser.parse();
    let mut env = runtime::environment::Environment::new();
    let addr =vec![
        RuntimeValue::String(StringValue::from("object")),
        RuntimeValue::String(StringValue::from("prop")),
    ];
    // let z = env.test(&addr);
}
