use crate::runtime::Runtime;
use crate::runtime::utils::value_to_ref;
use crate::runtime::value::exception;
use crate::runtime::value::indexable::{AttributeAccessible, Subscriptable};
use crate::runtime::value::methods::MethodProvider;
use crate::runtime::value::{RuntimeException, Value, ValueRef};
use std::hash::Hash;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct MapValue {
    pub properties: indexmap::IndexMap<Value, ValueRef>,
}

impl AttributeAccessible for MapValue {
    fn get_attr(
        &mut self,
        runtime: &mut Runtime,
        name: &String,
    ) -> Result<ValueRef, RuntimeException> {
        let func = MapValue::get_method(name.as_str())
            .ok_or(exception::ATTRIBUTE.runtime(format!("Attribute not found: {}", name)))?;
        Ok(value_to_ref(Value::rust_method(func, None)))
    }
}

impl Subscriptable for MapValue {
    fn index(
        &mut self,
        runtime: &mut Runtime,
        key: &ValueRef,
    ) -> Result<ValueRef, RuntimeException> {
        Ok(self
            .properties
            .get(&*key.borrow())
            .ok_or(exception::KEY_ERROR.runtime(format!("Key {:?} does not exist in map", key)))?
            .clone())
    }
}

impl Hash for MapValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unreachable!("Shouldn't be hashing a map value")
    }
}
