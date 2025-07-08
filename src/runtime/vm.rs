use std::cell::RefCell;
use std::rc::Rc;
use crate::compiler::code_object::CodeObject;
use crate::runtime::frame::RuntimeFrame;
use crate::runtime::Runtime;
use crate::runtime::values::Value;

pub(crate) fn load_constant(runtime: &mut Runtime, code_object: &CodeObject, constant_index: usize) {
    let constant_value = code_object.constants[constant_index].clone();
    runtime.mem_stack.push(constant_value);
}

pub(crate) fn load_variable(runtime: &mut Runtime, frame: &RuntimeFrame, variable_index: usize) {
    let var_value = frame.variables[variable_index].clone();
    runtime.mem_stack.push(var_value);
}

pub(crate) fn pop_check_truthy(runtime: &mut Runtime) -> bool {
    let condition = runtime.mem_stack.pop().unwrap();
    (&*condition.borrow()).is_truthy()
}

pub(crate) fn apply_bin_op<F>(runtime: &mut Runtime, f: F)
where
    F: Fn(&Value, &Value) -> Value,
{
    let b = runtime.mem_stack.pop().unwrap();
    let a = runtime.mem_stack.pop().unwrap();
    runtime.mem_stack
        .push(Rc::new(RefCell::new(f(&*a.borrow(), &*b.borrow()))));
}