use crate::compiler::code_object::CodeObject;
use crate::runtime::Runtime;
use crate::runtime::exceptions::RuntimeError;
use crate::runtime::utils::extract_int;
use crate::runtime::values::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn load_constant(
    runtime: &mut Runtime,
    code_object: &CodeObject,
    constant_index: usize,
) -> Result<(), RuntimeError> {
    let constant_value = code_object.constants[constant_index].clone();
    runtime.mem_stack.push(constant_value);
    Ok(())
}

pub(crate) fn load_local(runtime: &mut Runtime, variable_index: usize) -> Result<(), RuntimeError> {
    let frame = runtime.frames_stack.last().unwrap();
    let var_value = frame.variables[variable_index].clone();
    runtime.mem_stack.push(var_value);
    Ok(())
}

pub(crate) fn load_nonlocal(
    runtime: &mut Runtime,
    nonlocal_index: usize,
) -> Result<(), RuntimeError> {
    let frame_index = extract_int(&runtime.mem_stack.pop().unwrap()) as usize;
    let nonlocal_value =
        runtime.frames_stack.get(frame_index).unwrap().variables[nonlocal_index].clone();
    runtime.mem_stack.push(nonlocal_value);
    Ok(())
}

pub(crate) fn load_scope(runtime: &mut Runtime, code_object_id: usize) -> Result<(), RuntimeError> {
    let scope_frame_index = runtime
        .frames_stack_id_lookup
        .get(&code_object_id)
        .unwrap()
        .last()
        .unwrap();
    let index_value = Value::Int(*scope_frame_index as i64);
    runtime.mem_stack.push(Rc::new(RefCell::new(index_value)));
    Ok(())
}

pub(crate) fn pop_check_truthy(runtime: &mut Runtime) -> bool {
    let condition = runtime.mem_stack.pop().unwrap();
    (&*condition.borrow()).is_truthy()
}

pub(crate) fn apply_bin_op<F>(runtime: &mut Runtime, f: F) -> Result<(), RuntimeError>
where
    F: Fn(&Value, &Value) -> Result<Value, RuntimeError>,
{
    let b = runtime.mem_stack.pop().unwrap();
    let a = runtime.mem_stack.pop().unwrap();
    runtime
        .mem_stack
        .push(Rc::new(RefCell::new(f(&*a.borrow(), &*b.borrow())?)));
    Ok(())
}
