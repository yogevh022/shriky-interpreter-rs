use std::collections::HashMap;
use crate::runtime::environment::Environment;
use crate::runtime::values::{BoolValue, FloatValue, IntValue, RuntimeValue, StringValue};
use crate::parser::nodes::{ExprKind, ExprNode};
use std::mem;
use crate::runtime::environment::Counter;

pub struct Interpreter<'a> {
    env: &'a mut Environment,
    value_resolution_handlers: HashMap<ExprKind, fn(&'a mut Interpreter<'a>, ExprNode) -> RuntimeValue>,
}

impl<'a> Interpreter<'a> {
    pub fn new(env: &'a mut Environment) -> Self {
        let value_resolution_handlers = HashMap::from([
            (ExprKind::Int, Interpreter::resolve_int as fn(&'a mut Interpreter<'a>, ExprNode) -> RuntimeValue),
            (ExprKind::Float, Interpreter::resolve_float),
            (ExprKind::String, Interpreter::resolve_string),
            (ExprKind::Bool, Interpreter::resolve_bool),
            (ExprKind::Identity, Interpreter::resolve_identity),
            (ExprKind::Reference, Interpreter::resolve_identity)
        ]);
        Self {
            env,
            value_resolution_handlers,
        }
    }

    pub fn resolve_value(&'a mut self, node: ExprNode) -> RuntimeValue {
        if let Some(handler) = self.value_resolution_handlers.get(&node.kind()){
            return handler(self, node)
        }
        panic!("Value resolution failed for {:?}", node);
    }

    pub fn resolve_int(&mut self, node: ExprNode) -> RuntimeValue {
        match node {
            ExprNode::Int(value) => RuntimeValue::Int(IntValue { id: Counter.next(), value }),
            _ => panic!("Expected int, got {:?}", node),
        }
    }

    pub fn resolve_float(&mut self, node: ExprNode) -> RuntimeValue {
        match node {
            ExprNode::Float(value) => RuntimeValue::Float(FloatValue { id: Counter.next(), value }),
            _ => panic!("Expected float, got {:?}", node),
        }
    }

    pub fn resolve_string(&mut self, node: ExprNode) -> RuntimeValue {
        match node {
            ExprNode::String(value) => RuntimeValue::String(StringValue { id: Counter.next(), value }),
            _ => panic!("Expected string, got {:?}", node),
        }
    }

    pub fn resolve_bool(&mut self, node: ExprNode) -> RuntimeValue {
        match node {
            ExprNode::Bool(value) => RuntimeValue::Bool(BoolValue { id: Counter.next(), value }),
            _ => panic!("Expected bool, got {:?}", node),
        }
    }

    pub fn resolve_identity(&mut self, node: ExprNode) -> RuntimeValue {
        match node {
            ExprNode::Identity(identity_node) => {},
            ExprNode::Reference(reference_node) => {},
            _ => panic!("Expected identity or reference, got {:?}", node),
        }
        RuntimeValue::Int(IntValue { id: Counter.next(), value: 1 })
    }
}