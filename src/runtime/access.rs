use std::cell::RefCell;
use std::rc::Rc;
use crate::runtime::Runtime;
use crate::runtime::utils::{extract_class, extract_string};
use crate::runtime::values::{ClassValue, Value};

pub(crate) fn binary_subscribe(runtime: &mut Runtime) {
    let constant = runtime.mem_stack.pop().unwrap();
    let container = runtime.mem_stack.pop().unwrap();
    let constant_ref = constant.borrow();
    let container_ref = container.borrow();
    let result = match &*container_ref {
        Value::Map(obj) => obj.properties.get(&*constant_ref).unwrap(),
        Value::List(list) => {
            if let Value::Int(index) = constant_ref.clone() {
                list.elements.get(index as usize).unwrap()
            } else {
                panic!("Can only subscribe to lists with integers")
            }
        }
        _ => panic!("Invalid type for binary subscribe"),
    };
    runtime.mem_stack.push(result.clone());
}

pub(crate) fn get_inherited_attr(
    runtime: &mut Runtime,
    class_value: ClassValue,
    attr_string: String,
) -> Option<Rc<RefCell<Value>>> {
    let code_object = class_value.body;
    if let Some(attr_index) = code_object.variable_index_lookup.get(&attr_string) {
        Some(runtime.get_code_object_frame(&code_object).variables[*attr_index].clone())
    } else if let Some(superclass) = class_value.parent {
        let superclass_value = extract_class(&superclass);
        get_inherited_attr(runtime, superclass_value, attr_string)
    } else {
        None
    }
}

pub(crate) fn access_attr(runtime: &mut Runtime) {
    let attr = runtime.mem_stack.pop().unwrap();
    let container = runtime.mem_stack.pop().unwrap();
    let attr_string = extract_string(&attr);
    let maybe_attr_value = match &*container.borrow() {
        Value::Instance(instance_value) => {
            if let Some(attr_value) = instance_value.attributes.get(&attr_string) {
                Some(attr_value.clone())
            } else {
                let class_value = extract_class(&instance_value.class);
                get_inherited_attr(runtime, class_value, attr_string)
            }
        }
        _ => panic!("Invalid type for attribute access"),
    };
    let attr_value = maybe_attr_value.expect("Attribute not found");
    if let Value::Method(method_value) = &mut *attr_value.borrow_mut() {
        method_value.caller = Some(container.clone())
    }
    runtime.mem_stack.push(attr_value);
}