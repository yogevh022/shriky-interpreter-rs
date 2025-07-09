use crate::runtime::Runtime;
use crate::runtime::exceptions::RuntimeError;
use crate::runtime::utils::{extract_class, extract_string};
use crate::runtime::values::{ClassValue, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn binary_subscribe(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    let constant = runtime.mem_stack.pop().unwrap();
    let container = runtime.mem_stack.pop().unwrap();
    let constant_ref = constant.borrow();
    let container_ref = container.borrow();
    let result = match &*container_ref {
        Value::Map(obj) => {
            obj.properties
                .get(&*constant_ref)
                .ok_or(RuntimeError::EntryNotFound(format!(
                    "Key {:?} does not exist in map",
                    &*constant_ref
                )))?
        }
        Value::List(list) => {
            if let Value::Int(index) = constant_ref.clone() {
                list.elements
                    .get(index as usize)
                    .ok_or(RuntimeError::OutOfBounds(format!(
                        "List index {} is out of bounds",
                        index
                    )))?
            } else {
                Err(RuntimeError::InvalidType(
                    "Lists can only be subscribed to with integers".to_string(),
                ))?
            }
        }
        _ => Err(RuntimeError::InvalidType(
            "Attempted subscription to an unsubscribable type".to_string(),
        ))?,
    };
    runtime.mem_stack.push(result.clone());
    Ok(())
}

pub(crate) fn get_inherited_attr(
    runtime: &mut Runtime,
    class_value: ClassValue,
    attr_string: String,
) -> Result<Option<Rc<RefCell<Value>>>, RuntimeError> {
    let code_object = class_value.body;
    if let Some(attr_index) = code_object.variable_index_lookup.get(&attr_string) {
        Ok(Some(
            runtime.get_code_object_frame(&code_object)?.variables[*attr_index].clone(),
        ))
    } else if let Some(superclass) = class_value.parent {
        let superclass_value = extract_class(&superclass);
        get_inherited_attr(runtime, superclass_value, attr_string)
    } else {
        Ok(None)
    }
}

pub(crate) fn access_attr(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    let attr = runtime.mem_stack.pop().unwrap();
    let container = runtime.mem_stack.pop().unwrap();
    let attr_string = extract_string(&attr);
    let maybe_attr_value = match &*container.borrow() {
        Value::Instance(instance_value) => {
            if let Some(attr_value) = instance_value.attributes.get(&attr_string) {
                Some(attr_value.clone())
            } else {
                let class_value = extract_class(&instance_value.class);
                get_inherited_attr(runtime, class_value, attr_string)?
            }
        }
        _ => {
            return Err(RuntimeError::InvalidOperation(
                "Cannot access attributes of a non-instance type".to_string(),
            ));
        }
    };
    let attr_value = maybe_attr_value.expect("Attribute not found");
    if let Value::Method(method_value) = &mut *attr_value.borrow_mut() {
        method_value.caller = Some(container.clone())
    }
    runtime.mem_stack.push(attr_value);
    Ok(())
}
