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
    ];
    let val = RuntimeValue::String(StringValue::from("test complete!"));
    env.set_value(&addr, &val);
    let z = env.get_value(&addr);
    println!("{:?}", env.tt());
}
