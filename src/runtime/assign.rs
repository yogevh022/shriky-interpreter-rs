use crate::runtime::frame::RuntimeFrame;
use crate::runtime::utils::extract_string;
use crate::runtime::values::Value;
use std::cell::RefCell;
use std::rc::Rc;
use crate::runtime::Runtime;

pub fn pre_assign(
    runtime: &mut Runtime,
    frame: &RuntimeFrame,
    variable_index: usize,
) {
    let value = runtime.mem_stack.pop().unwrap();
    let var = frame.variables[variable_index].clone();
    let cloned_value = value.borrow().clone();
    *var.borrow_mut() = cloned_value;
}

pub fn assign_subscribe(runtime: &mut Runtime) {
    let value = runtime.mem_stack.pop().unwrap();
    let key = runtime.mem_stack.pop().unwrap();
    let container = runtime.mem_stack.pop().unwrap();
    match &mut *container.borrow_mut() {
        Value::Map(obj) => {
            obj.properties.insert(key.borrow().clone(), value.clone());
        }
        Value::List(list) => {
            if let Value::Int(index) = key.borrow().clone() {
                return list.elements.insert(index as usize, value.clone());
            }
            panic!("Can only subscribe to lists with integers")
        }
        _ => panic!("Invalid type for binary subscribe"),
    }
}

pub fn assign_attribute(runtime: &mut Runtime) {
    let value = runtime.mem_stack.pop().unwrap();
    let attr_name = runtime.mem_stack.pop().unwrap();
    let container = runtime.mem_stack.pop().unwrap();
    match &mut *container.borrow_mut() {
        Value::Instance(instance_value) => {
            instance_value
                .attributes
                .insert(extract_string(&attr_name), value.clone());
        }
        _ => panic!("Invalid attribute access"),
    }
}
