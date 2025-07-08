use crate::runtime::values::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::runtime::call::get_function_runtime_frame;
use crate::runtime::Runtime;
use crate::runtime::utils::{extract_class, extract_function};

pub fn make_map(runtime: &mut Runtime, property_count: usize) {
    let properties_kv: Vec<Rc<RefCell<Value>>> = runtime.mem_stack
        .drain(runtime.mem_stack.len() - property_count..)
        .collect();

    let mut properties = indexmap::IndexMap::new();
    for kv in properties_kv.chunks(2) {
        match kv {
            [k, v] => {
                properties.insert(k.borrow().clone(), v.clone());
            }
            _ => unreachable!("Map key without a value"),
        }
    }
    runtime.mem_stack.push(Rc::new(RefCell::new(Value::map(properties))));
}

pub fn make_list(runtime: &mut Runtime, list_size: usize) {
    let list_items = runtime.mem_stack
        .drain(runtime.mem_stack.len() - list_size..)
        .rev()
        .collect();
    runtime.mem_stack.push(Rc::new(RefCell::new(Value::list(list_items))));
}

pub fn make_class(runtime: &mut Runtime, is_inheriting: bool) {
    let maybe_class_value = runtime.mem_stack.pop().unwrap();
    let superclass_ref = is_inheriting.then(|| runtime.mem_stack.pop().unwrap().clone());
    let class_code_obj = match &*maybe_class_value.borrow() {
        Value::Class(class_value) => class_value.body.clone(),
        _ => panic!("Invalid class code object"),
    };
    runtime.mem_stack.push(Rc::new(RefCell::new(Value::class(
        superclass_ref,
        class_code_obj,
    ))));
}

pub(crate) fn make_instance(runtime: &mut Runtime, value_cls: Rc<RefCell<Value>>, mut args: Vec<Rc<RefCell<Value>>>) {
    let class_value = extract_class(&value_cls);
    let class_code_object = class_value.body;
    let instance = Rc::new(RefCell::new(Value::instance(value_cls, HashMap::new())));
    let frame = runtime.get_code_object_frame(&class_code_object);
    if let Some(init_func_index) = class_code_object.variable_index_lookup.get("init") {
        if let Some(init_func) = frame.variables.get(*init_func_index) {
            let init_func_value = extract_function(init_func);
            args.push(instance.clone());
            // execute init if exists
            runtime.execute(
                &init_func_value.body,
                &mut get_function_runtime_frame(&init_func_value, args),
            );
        } else {
            panic!("Invalid class init function index");
        }
    };
    runtime.mem_stack.push(instance);
}