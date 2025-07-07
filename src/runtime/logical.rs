use crate::runtime::values::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub fn logical_and(memory_stack: &mut Vec<Rc<RefCell<Value>>>) {
    let b = memory_stack.pop().unwrap();
    let a = memory_stack.pop().unwrap();
    let result = a.borrow().is_truthy() && b.borrow().is_truthy();
    memory_stack.push(Rc::new(RefCell::new(Value::Bool(result))));
}

pub fn logical_or(memory_stack: &mut Vec<Rc<RefCell<Value>>>) {
    let b = memory_stack.pop().unwrap();
    let a = memory_stack.pop().unwrap();
    let result = a.borrow().is_truthy() || b.borrow().is_truthy();
    memory_stack.push(Rc::new(RefCell::new(Value::Bool(result))));
}
