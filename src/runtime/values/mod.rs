use std::hash::Hash;
mod complex;
pub mod error;
mod primitives;
mod traits;

pub use complex::*;
pub use primitives::*;
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
    Identity(IdentityValue),
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
            RuntimeValue::Identity(identity) => identity.id(),
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            RuntimeValue::Int(_)
            | RuntimeValue::Float(_)
            | RuntimeValue::Bool(_)
            | RuntimeValue::String(_) => true,
            _ => false,
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
