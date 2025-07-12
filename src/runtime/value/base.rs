use crate::runtime::value::binary::traits::Binary;
use crate::runtime::value::indexable::{AttributeAccessible, Subscriptable};
use crate::runtime::value::types::bool::BoolValue;
use crate::runtime::value::types::float::FloatValue;
use crate::runtime::value::types::int::IntValue;
use crate::runtime::value::types::rust_method::RustMethodValue;
use crate::runtime::value::types::string::StringValue;
use crate::runtime::value::types::exception::ExceptionValue;
use crate::runtime::value::*;
use crate::utils::counter::Counter;
use std::fmt::Debug;
use std::hash::Hash;

pub(crate) static RUNTIME_VALUE_ID: Counter = Counter::new();

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum Value {
    Int(IntValue),
    Float(FloatValue),
    String(StringValue),
    Bool(BoolValue),
    Map(MapValue),
    List(ListValue),
    Function(FunctionValue),
    Method(MethodValue),
    RustMethod(RustMethodValue),
    Class(ClassValue),
    Instance(InstanceValue),
    Exception(ExceptionValue),
    Null,
}

impl Binary for Value {
    fn add(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match self {
            Value::Int(value) => value.add(other),
            Value::Float(value) => value.add(other),
            Value::String(value) => value.add(other),
            Value::Bool(value) => value.add(other),
            _ => unimplemented!(),
        }
    }

    fn sub(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match self {
            Value::Int(value) => value.sub(other),
            Value::Float(value) => value.sub(other),
            Value::String(value) => value.sub(other),
            Value::Bool(value) => value.sub(other),
            _ => unimplemented!(),
        }
    }

    fn mul(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match self {
            Value::Int(value) => value.mul(other),
            Value::Float(value) => value.mul(other),
            Value::String(value) => value.mul(other),
            Value::Bool(value) => value.mul(other),
            _ => unimplemented!(),
        }
    }

    fn div(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match self {
            Value::Int(value) => value.div(other),
            Value::Float(value) => value.div(other),
            Value::String(value) => value.div(other),
            Value::Bool(value) => value.div(other),
            _ => unimplemented!(),
        }
    }

    fn int_div(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match self {
            Value::Int(value) => value.int_div(other),
            Value::Float(value) => value.int_div(other),
            Value::String(value) => value.int_div(other),
            Value::Bool(value) => value.int_div(other),
            _ => unimplemented!(),
        }
    }

    fn modulus(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match self {
            Value::Int(value) => value.modulus(other),
            Value::Float(value) => value.modulus(other),
            Value::String(value) => value.modulus(other),
            Value::Bool(value) => value.modulus(other),
            _ => unimplemented!(),
        }
    }

    fn pow(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match self {
            Value::Int(value) => value.pow(other),
            Value::Float(value) => value.pow(other),
            Value::String(value) => value.pow(other),
            Value::Bool(value) => value.pow(other),
            _ => unimplemented!(),
        }
    }

    fn equals(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match self {
            Value::Int(value) => value.equals(other),
            Value::Float(value) => value.equals(other),
            Value::String(value) => value.equals(other),
            Value::Bool(value) => value.equals(other),
            _ => unimplemented!(),
        }
    }

    fn greater(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match self {
            Value::Int(value) => value.greater(other),
            Value::Float(value) => value.greater(other),
            Value::String(value) => value.greater(other),
            Value::Bool(value) => value.greater(other),
            _ => unimplemented!(),
        }
    }

    fn greater_equals(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match self {
            Value::Int(value) => value.greater_equals(other),
            Value::Float(value) => value.greater_equals(other),
            Value::String(value) => value.greater_equals(other),
            Value::Bool(value) => value.greater_equals(other),
            _ => unimplemented!(),
        }
    }

    fn less(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match self {
            Value::Int(value) => value.less(other),
            Value::Float(value) => value.less(other),
            Value::String(value) => value.less(other),
            Value::Bool(value) => value.less(other),
            _ => unimplemented!(),
        }
    }

    fn less_equals(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match self {
            Value::Int(value) => value.less_equals(other),
            Value::Float(value) => value.less_equals(other),
            Value::String(value) => value.less_equals(other),
            Value::Bool(value) => value.less_equals(other),
            _ => unimplemented!(),
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => b.0,
            Value::Int(i) => i.0 != 0,
            Value::Float(f) => f.0 != 0.0,
            Value::String(s) => !s.0.is_empty(),
            Value::Null => false,
            Value::Map(o) => !o.properties.is_empty(),
            Value::List(l) => !l.elements.is_empty(),
            _ => true,
        }
    }

    pub fn subscriptable(&mut self) -> Option<&mut dyn Subscriptable> {
        match self {
            Value::Map(m) => Some(m),
            Value::List(l) => Some(l),
            Value::Instance(i) => todo!(),
            Value::Class(c) => todo!(),
            _ => None,
        }
    }

    pub fn attribute_accessible(&mut self) -> Option<&mut dyn AttributeAccessible> {
        match self {
            Value::Instance(i) => Some(i),
            Value::Map(m) => Some(m),
            Value::List(l) => Some(l),
            Value::Class(c) => todo!(),
            _ => None,
        }
    }
}

pub enum ValueError {
    InvalidOperation,
    InvalidType,
}
