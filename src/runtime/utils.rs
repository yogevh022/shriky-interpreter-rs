use crate::runtime::value::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn value_to_ref(value: Value) -> ValueRef {
    Rc::new(RefCell::new(value))
}

pub fn extract_class_ref(value: &ValueRef) -> ClassValue {
    match &*value.borrow() {
        Value::Class(value) => value.clone(),
        _ => panic!("Unexpected value of type {:?}", *value.borrow()),
    }
}

pub fn extract_function_ref(value: &ValueRef) -> FunctionValue {
    match &*value.borrow() {
        Value::Function(value) => value.clone(),
        _ => panic!("Unexpected value of type {:?}", *value.borrow()),
    }
}

pub fn extract_string_ref(value: &ValueRef) -> String {
    match &*value.borrow() {
        Value::String(value) => value.clone().0,
        _ => panic!("Unexpected value of type {:?}", *value.borrow()),
    }
}

pub fn extract_int_ref(value: &ValueRef) -> i64 {
    match &*value.borrow() {
        Value::Int(value) => (*value).0,
        _ => panic!("Unexpected value of type {:?}", *value.borrow()),
    }
}
