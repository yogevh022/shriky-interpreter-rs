use crate::runtime::frame::RuntimeFrame;
use crate::runtime::values::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn get_function_runtime_frame(
    function_value: &FunctionValue,
    args: Vec<Rc<RefCell<Value>>>,
) -> RuntimeFrame {
    let mut func_runtime_frame = RuntimeFrame::from_size(
        function_value.body.variables.len() + function_value.parameters.len(),
    );
    function_value
        .parameters
        .iter()
        .zip(args.iter().rev())
        .for_each(|(p, v)| {
            func_runtime_frame.variables[function_value.body.variable_index_lookup[p]] = v.clone();
        });
    func_runtime_frame
}
