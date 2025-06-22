use std::hash::Hash;
mod primitives;
mod complex;
mod traits;
pub mod error;

pub use primitives::*;
pub use complex::*;
pub use traits::*;
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum RuntimeValue {
    Int(IntValue),
    Float(FloatValue),
    Bool(BoolValue),
    String(StringValue),
    Object(ObjectValue),
    List(ListValue),
    Reference(ReferenceValue),
}

impl RuntimeValue {
    pub fn id(&self) -> usize {
        match self {
            RuntimeValue::Object(obj) => obj.id(),
            RuntimeValue::List(list) => list.id(),
            RuntimeValue::Reference(reference) => reference.id(),
            RuntimeValue::Int(int) => int.id(),
            RuntimeValue::Float(float) => float.id(),
            RuntimeValue::Bool(bool) => bool.id(),
            RuntimeValue::String(str) => str.id(),
        }
    }
}

impl Hash for RuntimeValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            RuntimeValue::Int(i) => i.value.hash(state),
            RuntimeValue::Float(f) => f.value.to_bits().hash(state),
            RuntimeValue::Bool(b) => b.value.hash(state),
            RuntimeValue::String(s) => s.value.hash(state),
            _ => panic!("Unhashable type {:?}", self),
        }
    }
}