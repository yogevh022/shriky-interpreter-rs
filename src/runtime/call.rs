use crate::runtime::Runtime;
use crate::runtime::exceptions::RuntimeError;
use crate::runtime::frame::RuntimeFrame;
use crate::runtime::make::make_instance;
use crate::runtime::value::*;
use std::rc::Rc;

pub(crate) fn get_function_runtime_frame(
    function_value: &FunctionValue,
    args: Vec<ValueRef>,
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

pub(crate) fn call_rust_method(
    function: MethodFn,
    caller: &ValueRef,
    args: &[&ValueRef],
) -> Result<Option<ValueRef>, RuntimeError> {
    function(caller, args)
}

pub(crate) fn call(runtime: &mut Runtime, arg_count: usize) -> Result<(), RuntimeError> {
    let callee = runtime.mem_stack.pop().unwrap();
    let mut owned_args: Vec<ValueRef> = (0..arg_count)
        .map(|_| runtime.mem_stack.pop().unwrap())
        .collect();
    let (frame, co) = {
        match &*callee.borrow() {
            Value::Function(func_value) => {
                expect_args_count(owned_args.len(), func_value.parameters.len())?;
                (
                    get_function_runtime_frame(&func_value, owned_args),
                    func_value.body.clone(),
                ) // EXPENSIVE CLONE TEMP
            }
            Value::Method(method_value) => {
                owned_args.push(
                    method_value
                        .caller
                        .clone()
                        .expect("method called without caller"),
                );
                expect_args_count(owned_args.len(), method_value.function.parameters.len())?;
                (
                    get_function_runtime_frame(&method_value.function, owned_args),
                    method_value.function.body.clone(),
                ) // EXPENSIVE CLONE TEMP
            }
            Value::RustMethod(method_value) => {
                let arg_refs: Vec<&ValueRef> = owned_args.iter().collect();
                let result = call_rust_method(
                    method_value.function,
                    &method_value.caller.clone().unwrap(),
                    &arg_refs,
                )?;
                result.map(|result| runtime.mem_stack.push(result));
                return Ok(());
            }
            Value::Class(_) => {
                make_instance(runtime, callee.clone(), owned_args)?;
                return Ok(());
            }
            _ => panic!(),
        }
    };
    runtime.push_to_frame_stack(frame);
    runtime.execute(&co)?;
    runtime.pop_from_frame_stack();
    let return_value = runtime.pop_mem_stack_value_or_null();
    runtime.mem_stack.push(return_value);
    Ok(())
}
