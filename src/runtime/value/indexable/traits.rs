use crate::runtime::Runtime;
use crate::runtime::utils::extract_class_ref;
use crate::runtime::value::{ClassValue, RuntimeException, ValueRef};

pub(crate) fn get_class_attr(
    runtime: &mut Runtime,
    class_value: ClassValue,
    attr_string: &String,
) -> Result<Option<ValueRef>, RuntimeException> {
    let code_object = class_value.body;
    if let Some(attr_index) = code_object.variable_index_lookup.get(attr_string) {
        Ok(Some(
            runtime.get_code_object_frame(&code_object)?.variables[*attr_index].clone(),
        ))
    } else if let Some(superclass) = class_value.parent {
        let superclass_value = extract_class_ref(&superclass);
        get_class_attr(runtime, superclass_value, attr_string)
    } else {
        Ok(None)
    }
}

pub trait Subscriptable {
    fn index(
        &mut self,
        runtime: &mut Runtime,
        key: &ValueRef,
    ) -> Result<ValueRef, RuntimeException>;
}

pub trait AttributeAccessible {
    fn get_attr(
        &mut self,
        runtime: &mut Runtime,
        name: &String,
    ) -> Result<ValueRef, RuntimeException>;
}
