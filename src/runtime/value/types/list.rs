use crate::runtime::Runtime;
use crate::runtime::exceptions::RuntimeError;
use crate::runtime::utils::{extract_int_ref, value_to_ref};
use crate::runtime::value::indexable::{AttributeAccessible, Subscriptable};
use crate::runtime::value::methods::MethodProvider;
use crate::runtime::value::{MapValue, Value, ValueRef};
use std::hash::Hash;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ListValue {
    pub elements: Vec<ValueRef>,
}

impl AttributeAccessible for ListValue {
    fn get_attr(&mut self, runtime: &mut Runtime, name: &String) -> Result<ValueRef, RuntimeError> {
        let func = ListValue::get_method(name.as_str()).ok_or(RuntimeError::AttributeError(
            format!("Attribute not found: {}", name),
        ))?;
        Ok(value_to_ref(Value::rust_method(func, None)))
    }
}

impl Subscriptable for ListValue {
    fn index(&mut self, runtime: &mut Runtime, key: &ValueRef) -> Result<ValueRef, RuntimeError> {
        let index = match &*key.borrow() {
            Value::Int(index) => index.0 as usize,
            _ => Err(RuntimeError::InvalidType(
                "Lists can only be subscribed to with integers".to_string(),
            ))?,
        };
        Ok(self.elements[index].clone())
    }
}

impl Hash for ListValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unreachable!("Shouldn't be hashing a list value")
    }
}
