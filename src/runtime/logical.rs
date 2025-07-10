use crate::runtime::Runtime;
use crate::runtime::exceptions::RuntimeError;
use crate::runtime::value::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub fn logical_and(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    let b = runtime.mem_stack.pop().unwrap();
    let a = runtime.mem_stack.pop().unwrap();
    let result = a.borrow().is_truthy() && b.borrow().is_truthy();
    runtime
        .mem_stack
        .push(Rc::new(RefCell::new(Value::bool(result))));
    Ok(())
}

pub fn logical_or(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    let b = runtime.mem_stack.pop().unwrap();
    let a = runtime.mem_stack.pop().unwrap();
    let result = a.borrow().is_truthy() || b.borrow().is_truthy();
    runtime
        .mem_stack
        .push(Rc::new(RefCell::new(Value::bool(result))));
    Ok(())
}
