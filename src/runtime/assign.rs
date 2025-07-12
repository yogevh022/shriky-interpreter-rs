use crate::runtime::Runtime;
use crate::runtime::utils::extract_string_ref;
use crate::runtime::value::exception;
use crate::runtime::value::{RuntimeException, Value};

pub fn pre_assign(runtime: &mut Runtime, variable_index: usize) -> Result<(), RuntimeException> {
    let frame = runtime.frames_stack.last().unwrap();
    let value = runtime.mem_stack.pop().unwrap();
    let var = frame.variables[variable_index].clone();
    let cloned_value = value.borrow().clone();
    *var.borrow_mut() = cloned_value;
    Ok(())
}

pub fn assign_subscribe(runtime: &mut Runtime) -> Result<(), RuntimeException> {
    let value = runtime.mem_stack.pop().unwrap();
    let key = runtime.mem_stack.pop().unwrap();
    let container = runtime.mem_stack.pop().unwrap();
    match &mut *container.borrow_mut() {
        Value::Map(obj) => {
            obj.properties.insert(key.borrow().clone(), value.clone());
            Ok(())
        }
        Value::List(list) => {
            if let Value::Int(index) = key.borrow().clone() {
                list.elements.insert(index.0 as usize, value.clone());
                return Ok(());
            }
            Err(exception::TYPE
                .runtime("Lists can only be subscribed to with integers".to_string()))
        }
        _ => {
            Err(exception::TYPE
                .runtime("Attempted subscription to an unsubscribable type".to_string()))
        }
    }
}

pub fn assign_attribute(runtime: &mut Runtime) -> Result<(), RuntimeException> {
    let value = runtime.mem_stack.pop().unwrap();
    let attr_name = runtime.mem_stack.pop().unwrap();
    let container = runtime.mem_stack.pop().unwrap();
    match &mut *container.borrow_mut() {
        Value::Instance(instance_value) => {
            instance_value
                .attributes
                .insert(extract_string_ref(&attr_name), value.clone());
            Ok(())
        }
        _ => {
            Err(exception::TYPE
                .runtime("Cannot access attributes of a non-instance type".to_string()))
        }
    }
}
