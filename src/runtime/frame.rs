use crate::compiler::code_object::CodeObject;
use crate::runtime::values::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct RuntimeFrame {
    pub code_object_id: usize,
    pub variables: Vec<Rc<RefCell<Value>>>,
    pub variable_index_lookup: HashMap<String, usize>,
}

impl RuntimeFrame {
    pub fn from_co(code_object: &CodeObject) -> Self {
        Self {
            code_object_id: code_object.id,
            variables: (0..code_object.variables.len())
                .map(|_| Rc::new(RefCell::new(Value::Null)))
                .collect(),
            variable_index_lookup: code_object.variable_index_lookup.clone(),
        }
    }
}
