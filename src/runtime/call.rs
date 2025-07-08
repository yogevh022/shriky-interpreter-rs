use crate::runtime::frame::RuntimeFrame;
use crate::runtime::values::*;
use std::cell::RefCell;
use std::rc::Rc;
use crate::runtime::make::make_instance;
use crate::runtime::Runtime;

pub(crate) fn get_function_runtime_frame(
    function_value: &FunctionValue,
    args: Vec<Rc<RefCell<Value>>>,
) -> RuntimeFrame {
    let mut func_runtime_frame = RuntimeFrame::from_size(function_value.body.variables.len());
    function_value
        .parameters
        .iter()
        .zip(args.iter().rev())
        .for_each(|(p, v)| {
            func_runtime_frame.variables[function_value.body.variable_index_lookup[p]] = v.clone();
        });
    func_runtime_frame
}

pub(crate) fn call(runtime: &mut Runtime, arg_count: usize) {
    let callee = runtime.mem_stack.pop().unwrap();
    let mut args: Vec<Rc<RefCell<Value>>> = (0..arg_count)
        .map(|_| runtime.mem_stack.pop().unwrap())
        .collect();
    match &*callee.borrow() {
        Value::Function(func_value) => {
            runtime.execute(
                &func_value.body,
                &mut get_function_runtime_frame(func_value, args),
            );
            let return_value = runtime.pop_mem_stack_value_or_null();
            runtime.mem_stack.push(return_value);
        }
        Value::Method(method_value) => {
            args.push(
                method_value
                    .caller
                    .clone()
                    .expect("method called without caller"),
            );
            runtime.execute(
                &method_value.function.body,
                &mut get_function_runtime_frame(&method_value.function, args),
            );
            let return_value = runtime.pop_mem_stack_value_or_null();
            runtime.mem_stack.push(return_value);
        }
        Value::Class(_) => make_instance(runtime, callee.clone(), args),
        _ => panic!("Called uncallable value"),
    }
}