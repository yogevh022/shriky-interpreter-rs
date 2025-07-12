use crate::runtime::Runtime;
use crate::runtime::utils::extract_class_ref;
use crate::runtime::value::exception;
use crate::runtime::value::indexable::{AttributeAccessible, get_class_attr};
use crate::runtime::value::{RuntimeException, ValueRef};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct InstanceValue {
    pub id: usize,
    pub class: ValueRef,
    pub attributes: HashMap<String, ValueRef>,
}

impl AttributeAccessible for InstanceValue {
    fn get_attr(
        &mut self,
        runtime: &mut Runtime,
        name: &String,
    ) -> Result<ValueRef, RuntimeException> {
        if let Some(attr_value) = self.attributes.get(name) {
            return Ok(attr_value.clone());
        }
        let class_value = extract_class_ref(&self.class);
        Ok(
            get_class_attr(runtime, class_value, name)?.ok_or(exception::ATTRIBUTE.runtime(
                format!("Attribute {} not found in class {:?}.", name, self.class),
            ))?,
        )
    }
}

impl Hash for InstanceValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}
