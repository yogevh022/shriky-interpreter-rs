use crate::compiler::byte_operations::OpIndex;
use crate::parser::nodes::FunctionNode;
use crate::runtime::values::Value;
use crate::utils::counter::Counter;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct CodeObject {
    pub id: usize,
    pub operations: Vec<OpIndex>,
    pub constants: Vec<Rc<RefCell<Value>>>, // these are never mutated, RefCell for uniformity
    pub variables: Vec<String>,
    pub constant_index_lookup: HashMap<usize, usize>, // constant ExprNode id -> constant index
    pub variable_index_lookup: HashMap<String, usize>, // variable name -> variable index
}

impl CodeObject {
    pub fn from_function(function_node: &FunctionNode) -> Self {
        Self {
            id: CODE_OBJECT_ID.next(),
            operations: Vec::new(),
            constants: Vec::new(),
            variables: function_node.arguments.clone(),
            constant_index_lookup: HashMap::new(),
            variable_index_lookup: function_node
                .arguments
                .iter()
                .enumerate()
                .map(|(i, v)| (v.clone(), i))
                .collect(),
        }
    }
}

impl Hash for CodeObject {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unreachable!("Shouldn't be hashing a code object")
    }
}

impl PartialEq for CodeObject {
    fn eq(&self, other: &Self) -> bool {
        unreachable!("Shouldn't be comparing code objects")
    }
}

impl Eq for CodeObject {
    fn assert_receiver_is_total_eq(&self) {
        unreachable!("Shouldn't be comparing code objects")
    }
}

static CODE_OBJECT_ID: Counter = Counter::new();

impl Default for CodeObject {
    fn default() -> Self {
        Self {
            id: CODE_OBJECT_ID.next(),
            operations: Vec::new(),
            constants: Vec::new(),
            variables: Vec::new(),
            constant_index_lookup: HashMap::new(),
            variable_index_lookup: HashMap::new(),
        }
    }
}
