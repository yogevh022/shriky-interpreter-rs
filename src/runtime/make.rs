use crate::runtime::values::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub fn make_map(memory_stack: &mut Vec<Rc<RefCell<Value>>>, property_count: usize) {
    let properties_kv: Vec<Rc<RefCell<Value>>> = memory_stack
        .drain(memory_stack.len() - property_count..)
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
    memory_stack.push(Rc::new(RefCell::new(Value::map(properties))));
}

pub fn make_list(memory_stack: &mut Vec<Rc<RefCell<Value>>>, list_size: usize) {
    let list_items = memory_stack
        .drain(memory_stack.len() - list_size..)
        .rev()
        .collect();
    memory_stack.push(Rc::new(RefCell::new(Value::list(list_items))));
}

pub fn make_class(memory_stack: &mut Vec<Rc<RefCell<Value>>>, is_inheriting: bool) {
    let maybe_class_value = memory_stack.pop().unwrap();
    let superclass_ref = if is_inheriting {
        let superclass = memory_stack.pop().unwrap();
        Some(superclass.clone())
    } else {
        None
    };
    let class_code_obj = match &*maybe_class_value.borrow() {
        Value::Class(class_value) => class_value.body.clone(),
        _ => panic!("Invalid class code object"),
    };
    memory_stack.push(Rc::new(RefCell::new(Value::class(
        superclass_ref,
        class_code_obj,
    ))));
}
