use crate::runtime::environment::Environment;
use crate::runtime::values::{BoolValue, FloatValue, IntValue, RuntimeValue, StringValue};
use crate::parser::nodes::*;

pub struct Interpreter<'a> {
    env: &'a mut Environment,
}

impl<'a> Interpreter<'a> {
    pub fn new(env: &'a mut Environment) -> Self {
        Self { env }
    }

    pub fn eval(&mut self, expr: ExprNode) -> Result<RuntimeValue, String> {
        todo!();
        // match expr {
        //     ExprNode::Binary(binary) => {},
        //     ExprNode::Assign(assign) => {},
        //     ExprNode::FuncCall(func_call) => {},
        // }
    }
}
