use super::error::AccessError;
use super::traits::*;
use crate::runtime::environment::Counter;
use crate::runtime::values::RuntimeValue;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ObjectValue {
    pub id: u64,
    pub properties: indexmap::IndexMap<RuntimeValue, u64>, // u64 is memory address
}

impl ObjectValue {
    pub fn get_property(&self, key: &RuntimeValue) -> Result<Option<&u64>, AccessError> {
        match key {
            RuntimeValue::Int(_)
            | RuntimeValue::Float(_)
            | RuntimeValue::Bool(_)
            | RuntimeValue::String(_) => Ok(self.properties.get(key)),
            _ => Err(AccessError::InvalidAddress),
        }
    }

    pub fn set_property(&mut self, key: &RuntimeValue, value: u64) {
        self.properties.insert(key.clone(), value);
    }
}

impl From<indexmap::IndexMap<RuntimeValue, u64>> for ObjectValue {
    fn from(properties: indexmap::IndexMap<RuntimeValue, u64>) -> Self {
        Self {
            properties,
            id: Counter.next(),
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
    fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ListValue {
    pub id: u64,
    pub elements: Vec<u64>, // u64 is memory address
}

impl ListValue {
    pub fn get_element(&self, index: &RuntimeValue) -> Result<Option<&u64>, AccessError> {
        match index {
            RuntimeValue::Int(i) => Ok(self.elements.get(i.value as usize)),
            _ => Err(AccessError::InvalidAddress),
        }
    }

    pub fn set_element(&mut self, index: &RuntimeValue, value: u64) -> Result<(), AccessError> {
        match index {
            RuntimeValue::Int(i) => Ok(self.elements[i.value as usize] = value),
            _ => Err(AccessError::InvalidAddress),
        }
    }
}

impl Default for ListValue {
    fn default() -> Self {
        ListValue {
            id: Counter.next(),
            elements: Vec::new(),
        }
    }
}

impl HasId for ListValue {
    fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct IdentityValue {
    pub id: u64,
    pub address: Vec<RuntimeValue>,
}

impl HasId for IdentityValue {
    fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ReferenceValue {
    pub id: u64,
    pub memory_address: u64,
}

impl HasId for ReferenceValue {
    fn id(&self) -> u64 {
        self.id
    }
}
