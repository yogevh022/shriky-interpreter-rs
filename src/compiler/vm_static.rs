use crate::compiler::code_object::CodeObject;
use crate::runtime::value::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn cache_constant(
    code_object: &mut CodeObject,
    constant_id: usize,
    runtime_constant: Value,
) -> usize {
    let new_literal_index = code_object.constants.len();
    code_object
        .constant_index_lookup
        .insert(constant_id, new_literal_index);
    code_object
        .constants
        .push(Rc::new(RefCell::new(runtime_constant)));
    new_literal_index
}

pub(crate) fn cache_variable(code_object: &mut CodeObject, name: &String) -> usize {
    if let Some(var_index) = code_object.variable_index_lookup.get(name) {
        *var_index
    } else {
        let new_variable_index = code_object.variables.len();
        code_object
            .variable_index_lookup
            .insert(name.clone(), new_variable_index);
        code_object.variables.push(name.clone());
        new_variable_index
    }
}
