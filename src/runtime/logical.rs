use crate::runtime::values::Value;
use std::cell::RefCell;
use std::rc::Rc;
use crate::runtime::Runtime;

pub fn logical_and(runtime: &mut Runtime) {
    let b = runtime.mem_stack.pop().unwrap();
    let a = runtime.mem_stack.pop().unwrap();
    let result = a.borrow().is_truthy() && b.borrow().is_truthy();
    runtime.mem_stack.push(Rc::new(RefCell::new(Value::Bool(result))));
}

pub fn logical_or(runtime: &mut Runtime) {
    let b = runtime.mem_stack.pop().unwrap();
    let a = runtime.mem_stack.pop().unwrap();
    let result = a.borrow().is_truthy() || b.borrow().is_truthy();
    runtime.mem_stack.push(Rc::new(RefCell::new(Value::Bool(result))));
}
