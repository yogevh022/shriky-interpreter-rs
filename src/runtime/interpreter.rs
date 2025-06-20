use std::collections::HashMap;
use crate::runtime::environment::Environment;
use crate::runtime::values::RuntimeValue;
use crate::parser::nodes::ExprNode;
use std::mem;
pub struct Interpreter<'a> {
    env: &'a mut Environment,
    value_resolution_handlers: HashMap<mem::Discriminant<ExprNode>, fn(ExprNode) -> RuntimeValue>,
}

impl<'a> Interpreter<'a> {
    pub fn new(env: &'a mut Environment) -> Self {
        let value_resolution_handlers: HashMap<mem::Discriminant<ExprNode>, fn(ExprNode) -> RuntimeValue> = HashMap::from([
            ()
        ]);
        Self {
            env,
            value_resolution_handlers,
        }
    }

    pub fn resolve_value(&mut self, node: ExprNode) -> RuntimeValue {
        if let Some(handler) = self.value_resolution_handlers.get(&mem::discriminant(&node)){
            return handler(node)
        }
        panic!("Value resolution failed for {:?}", node);
    }

    pub fn resolve_int(&mut self, node: ExprNode) -> RuntimeValue {
        match node {
            ExprNode::Int(value) => RuntimeValue::Int(value),
            _ => panic!("Expected int, got {:?}", node),
        }
    }

    pub fn resolve_float(&mut self, node: ExprNode) -> RuntimeValue {
        match node {
            ExprNode::Float(value) => RuntimeValue::Float(value),
            _ => panic!("Expected float, got {:?}", node),
        }
    }

    pub fn resolve_string(&mut self, node: ExprNode) -> RuntimeValue {
        match node {
            ExprNode::String(value) => RuntimeValue::String(value),
            _ => panic!("Expected string, got {:?}", node),
        }
    }

    pub fn resolve_bool(&mut self, node: ExprNode) -> RuntimeValue {
        match node {
            ExprNode::Bool(value) => RuntimeValue::Bool(value),
            _ => panic!("Expected bool, got {:?}", node),
        }
    }

    pub fn resolve_identity(&mut self, node: ExprNode) -> RuntimeValue {
        // todo
    }

    
}