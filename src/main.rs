use crate::runtime::environment::utils::Counter;
use crate::runtime::interpreter;
use crate::runtime::values::*;
mod lexer;
mod parser;
mod runtime;

fn main() {
    let source: String = String::from("a=4+2;b=5-4;");
    let mut lex = lexer::Lexer::new(&source);
    let mut parser = parser::Parser::new(&mut lex);
    let ast = parser.parse();
    let expr_obj = ast.first().unwrap().to_owned();
    let mut env = runtime::environment::Environment::new();
    let interpreter = interpreter::Interpreter::new(&mut env);

    let iden = IdentityValue {
        address: vec![RuntimeValue::String(StringValue::from("hi"))],
        id: Counter.next(),
    };
    println!("{:?}", expr_obj);
}
