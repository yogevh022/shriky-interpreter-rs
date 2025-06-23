use crate::runtime::values::RuntimeValue;
use crate::runtime::environment::Counter;
use super::error::AccessError;
use super::traits::*;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ObjectValue {
    pub id: usize,
    pub properties: indexmap::IndexMap<RuntimeValue, usize>, // usize is memory address
}

impl ObjectValue {
    pub fn get_property(&self, key: &RuntimeValue) -> Result<Option<&usize>, AccessError> {
        match key {
            RuntimeValue::Int(_)
            | RuntimeValue::Float(_)
            | RuntimeValue::Bool(_)
            | RuntimeValue::String(_) => Ok(self.properties.get(key)),
            _ => Err(AccessError::InvalidAddress)
        }
    }
}

impl Default for ObjectValue {
    fn default() -> Self {
        ObjectValue {
            id: Counter.next(),
            properties: indexmap::IndexMap::new(),
        }
    }
}

impl HasId for ObjectValue {
    fn id(&self) -> usize { self.id }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ListValue {
    pub id: usize,
    pub elements: Vec<usize>, // usize is memory address
}

impl ListValue {
    pub fn get_element(&self, index: &RuntimeValue) -> Result<Option<&usize>, AccessError> {
        match index {
            RuntimeValue::Int(i) => Ok(self.elements.get(i.value as usize)),
            _ => Err(AccessError::InvalidAddress)
        }
    }
}

impl Default for ListValue {
    fn default() -> Self {
        ListValue {
            id: Counter.next(),
            elements: Vec::new()
        }
    }
}

impl HasId for ListValue {
    fn id(&self) -> usize { self.id }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ReferenceValue {
    pub id: usize,
    pub memory_address: usize,
}

impl HasId for ReferenceValue {
    fn id(&self) -> usize { self.id }
}
