use crate::compiler::byte_operations::OperationIndex;
use crate::parser::ExprNode;
use crate::parser::nodes::FloatNode;
use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Value {
    Int(i64),
    Float(OrderedFloat<f64>),
    String(String),
    Bool(bool),
    Object(ObjectValue),
    List(ListValue),
    Function(CodeObject),
    Null,
}

#[derive(Eq, PartialEq, Debug)]
pub struct ObjectValue {
    pub properties: indexmap::IndexMap<Value, Value>,
}

impl Hash for ObjectValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unreachable!("Shouldn't be hashing an object value")
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct ListValue {
    pub elements: Vec<Value>,
}

impl Hash for ListValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unreachable!("Shouldn't be hashing a list value")
    }
}

impl Value {
    pub fn int(value: i64) -> Value {
        Value::Int(value)
    }

    pub fn float<T: Into<OrderedFloat<f64>>>(value: T) -> Value {
        Value::Float(value.into())
    }

    pub fn string(value: String) -> Value {
        Value::String(value)
    }

    pub fn bool(value: bool) -> Value {
        Value::Bool(value)
    }

    pub fn object(properties: indexmap::IndexMap<Value, Value>) -> Value {
        Value::Object(ObjectValue { properties })
    }

    pub fn list(elements: Vec<Value>) -> Value {
        Value::List(ListValue { elements })
    }

    pub fn null() -> Value {
        Value::Null
    }
    pub fn from_expr(expr: ExprNode) -> Value {
        match expr {
            ExprNode::Int(int_node) => Value::int(int_node.value),
            ExprNode::Float(float_node) => Value::float(float_node.value),
            ExprNode::String(string_node) => Value::string(string_node.value),
            ExprNode::Bool(bool_node) => Value::bool(bool_node.value),
            ExprNode::Object(object_node) => {
                let mut obj_props = indexmap::IndexMap::new();
                object_node.properties.into_iter().for_each(|obj_prop| {
                    obj_props.insert(
                        Value::from_expr(obj_prop.key),
                        Value::from_expr(obj_prop.value),
                    );
                });
                Value::object(obj_props)
            }
            ExprNode::List(list_node) => {
                let list_elements = list_node
                    .elements
                    .into_iter()
                    .map(|list_item| Value::from_expr(list_item))
                    .collect();
                Value::list(list_elements)
            }
            _ => panic!("Invalid expression node type {:?}", expr),
        }
    }
}

#[derive(Debug)]
pub struct CodeObject {
    pub operations: Vec<OperationIndex>,
    pub constants: Vec<Value>,
    pub variables: Vec<String>,
    pub constant_index_lookup: HashMap<usize, usize>, // constant ExprNode id -> constant index
    pub variable_index_lookup: HashMap<String, usize>, // variable name -> variable index
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

impl Default for CodeObject {
    fn default() -> Self {
        Self {
            operations: Vec::new(),
            constants: Vec::new(),
            variables: Vec::new(),
            constant_index_lookup: HashMap::new(),
            variable_index_lookup: HashMap::new(),
        }
    }
}
