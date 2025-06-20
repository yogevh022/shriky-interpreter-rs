use std::hash::Hash;
use ordered_float::OrderedFloat;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum RuntimeValue {
    Int(i64),
    Float(OrderedFloat<f64>),
    Bool(bool),
    String(String),
    Object(ObjectValue),
    List(ListValue),
}

impl Hash for RuntimeValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            RuntimeValue::Int(i) => i.hash(state),
            RuntimeValue::Float(f) => f.to_bits().hash(state),
            RuntimeValue::Bool(b) => b.hash(state),
            RuntimeValue::String(s) => s.hash(state),
            _ => panic!("Unhashable type {:?}", self),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ObjectValue {
    pub properties: indexmap::IndexMap<RuntimeValue, RuntimeValue>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ListValue {
    pub elements: Vec<RuntimeValue>,
}
