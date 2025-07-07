use crate::runtime::frame::RuntimeFrame;
use crate::runtime::values::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub fn pre_assign(
    memory_stack: &mut Vec<Rc<RefCell<Value>>>,
    frame: &RuntimeFrame,
    variable_index: usize,
) {
    let value = memory_stack.pop().unwrap();
    let var = frame.variables[variable_index].clone();
    let cloned_value = value.borrow().clone();
    *var.borrow_mut() = cloned_value;
}

pub fn assign_subscribe(memory_stack: &mut Vec<Rc<RefCell<Value>>>) {
    let value = memory_stack.pop().unwrap();
    let key = memory_stack.pop().unwrap();
    let container = memory_stack.pop().unwrap();
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

pub fn assign_attribute(memory_stack: &mut Vec<Rc<RefCell<Value>>>) {
    let value = memory_stack.pop().unwrap();
    let attr = memory_stack.pop().unwrap();
    let container = memory_stack.pop().unwrap();
    let attr_name = match &*attr.borrow() {
        Value::String(attr_name) => attr_name.clone(),
        _ => panic!("Invalid type for attribute name"),
    };
    match &mut *container.borrow_mut() {
        Value::Instance(instance_value) => {
            instance_value.attributes.insert(attr_name, value.clone());
        }
        _ => panic!("Invalid attribute access"),
    }
}
