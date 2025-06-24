use crate::runtime::environment::utils::Counter;
use crate::runtime::values::*;
mod lexer;
mod parser;
mod runtime;

fn main() {
    let source: String = String::from("{'key1': {'nested_key': 'nested_value'}}");
    let mut lex = lexer::Lexer::new(&source);
    let mut parser = parser::Parser::new(&mut lex);
    let ast = parser.parse();
    let expr_obj = ast.first().unwrap().to_owned();
    let mut env = runtime::environment::Environment::new();
    let new_obj = env.memory_dump(expr_obj, 999);
    let iden = IdentityValue {
        address: vec![RuntimeValue::String(StringValue::from("hi"))],
        id: Counter.next(),
    };

    println!("iden:: {:?}", env.get_value(&iden.address));

    env.assign_value_to_identity(
        &iden,
        &RuntimeValue::Reference(ReferenceValue {
            memory_address: 1,
            id: Counter.next(),
        }),
    )
    .ok();
    println!("iden:: {:?}", env.get_value(&iden.address));

    env.decrement_refs_if_exists(&1);

    println!("1:: {:?}", env.get_value_by_addr(&1));
    println!("3:: {:?}", env.get_value_by_addr(&3));
    println!("5:: {:?}", env.get_value_by_addr(&5));
}
