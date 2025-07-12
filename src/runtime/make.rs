use crate::runtime::Runtime;
use crate::runtime::call::{expect_args_count, get_function_runtime_frame};
use crate::runtime::utils::{extract_class_ref, extract_function_ref};
use crate::runtime::value::{RuntimeException, Value, ValueRef};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) fn make_map(
    runtime: &mut Runtime,
    property_count: usize,
) -> Result<(), RuntimeException> {
    let properties_kv: Vec<ValueRef> = runtime
        .mem_stack
        .drain(runtime.mem_stack.len() - property_count..)
        .collect();

    let mut properties = HashMap::new();
    for kv in properties_kv.chunks(2) {
        match kv {
            [k, v] => {
                properties.insert(k.borrow().clone(), v.clone());
            }
            _ => unreachable!("Map key without a value"),
        }
    }
    runtime
        .mem_stack
        .push(Rc::new(RefCell::new(Value::map(properties))));
    Ok(())
}

pub(crate) fn make_list(runtime: &mut Runtime, list_size: usize) -> Result<(), RuntimeException> {
    let list_items = runtime
        .mem_stack
        .drain(runtime.mem_stack.len() - list_size..)
        .rev()
        .collect();
    runtime
        .mem_stack
        .push(Rc::new(RefCell::new(Value::list(list_items))));
    Ok(())
}

pub(crate) fn make_class(
    runtime: &mut Runtime,
    is_inheriting: bool,
) -> Result<(), RuntimeException> {
    let uncasted_class = runtime.mem_stack.pop().unwrap();
    let superclass_ref = is_inheriting.then(|| runtime.mem_stack.pop().unwrap().clone());
    let class_code_obj = match &*uncasted_class.borrow() {
        Value::Class(class_value) => class_value.body.clone(),
        _ => unreachable!("Make class called on non-class value, compiler level error"),
    };
    runtime.mem_stack.push(Rc::new(RefCell::new(Value::class(
        superclass_ref,
        class_code_obj,
    ))));
    Ok(())
}

pub(crate) fn make_instance(
    runtime: &mut Runtime,
    value_cls: ValueRef,
    mut args: Vec<ValueRef>,
) -> Result<(), RuntimeException> {
    let class_value = extract_class_ref(&value_cls);
    let class_code_object = class_value.body;
    let instance = Rc::new(RefCell::new(Value::instance(value_cls, HashMap::new())));
    let frame = runtime.get_code_object_frame(&class_code_object)?;
    // execute init if exists
    if let Some(init_func_index) = class_code_object.variable_index_lookup.get("init") {
        if let Some(init_func) = frame.variables.get(*init_func_index) {
            let init_func_value = extract_function_ref(init_func);
            args.push(instance.clone());
            expect_args_count(args.len(), init_func_value.parameters.len())?;
            runtime.push_to_frame_stack(get_function_runtime_frame(&init_func_value, args));
            runtime.execute(&init_func_value.body)?;
            runtime.pop_from_frame_stack();
        } else {
            panic!("Invalid class init function index, compiler level error");
        }
    };
    runtime.mem_stack.push(instance);
    Ok(())
}
