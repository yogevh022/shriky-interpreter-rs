use crate::runtime::Runtime;
use crate::runtime::exceptions::RuntimeError;
use crate::runtime::frame::RuntimeFrame;
use crate::runtime::make::make_instance;
use crate::runtime::values::*;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn get_function_runtime_frame(
    function_value: &FunctionValue,
    args: Vec<Rc<RefCell<Value>>>,
) -> RuntimeFrame {
    let mut func_runtime_frame = RuntimeFrame::from_co(&function_value.body);
    function_value
        .parameters
        .iter()
        .zip(args.iter().rev())
        .for_each(|(p, v)| {
            func_runtime_frame.variables[function_value.body.variable_index_lookup[p]] = v.clone();
        });
    func_runtime_frame
}

pub(crate) fn expect_args_count(
    arg_count: usize,
    expected_arg_count: usize,
) -> Result<(), RuntimeError> {
    if arg_count != expected_arg_count {
        return Err(RuntimeError::ArgumentCount(format!(
            "Callable expected {} arguments, got {}",
            expected_arg_count, arg_count
        )));
    }
    Ok(())
}

pub(crate) fn call(runtime: &mut Runtime, arg_count: usize) -> Result<(), RuntimeError> {
    let callee = runtime.mem_stack.pop().unwrap();
    let mut args: Vec<Rc<RefCell<Value>>> = (0..arg_count)
        .map(|_| runtime.mem_stack.pop().unwrap())
        .collect();
    match &*callee.borrow() {
        Value::Function(func_value) => {
            expect_args_count(args.len(), func_value.parameters.len())?;
            runtime
                .frames_stack
                .push(get_function_runtime_frame(func_value, args));
            runtime.execute(&func_value.body)?;
            runtime.frames_stack.pop();
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
            expect_args_count(args.len(), method_value.function.parameters.len())?;
            runtime
                .frames_stack
                .push(get_function_runtime_frame(&method_value.function, args));
            runtime.execute(&method_value.function.body)?;
            let return_value = runtime.pop_mem_stack_value_or_null();
            runtime.mem_stack.push(return_value);
        }
        Value::Instance(instance_value) => todo!(),
        Value::Class(_) => make_instance(runtime, callee.clone(), args)?,
        _ => {
            return Err(RuntimeError::InvalidType(
                "Called an uncallable value".to_string(),
            ));
        }
    }
    Ok(())
}
