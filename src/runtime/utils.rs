use crate::runtime::values::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn extract_class(value: &Rc<RefCell<Value>>) -> ClassValue {
    match &*value.borrow() {
        Value::Class(value) => value.clone(),
        _ => panic!("Unexpected value of type {:?}", *value.borrow()),
    }
}

pub fn extract_function(value: &Rc<RefCell<Value>>) -> FunctionValue {
    match &*value.borrow() {
        Value::Function(value) => value.clone(),
        _ => panic!("Unexpected value of type {:?}", *value.borrow()),
    }
}

pub fn extract_string(value: &Rc<RefCell<Value>>) -> String {
    match &*value.borrow() {
        Value::String(value) => value.clone(),
        _ => panic!("Unexpected value of type {:?}", *value.borrow()),
    }
}
