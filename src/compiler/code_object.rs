use crate::compiler::byte_operations::OpIndex;
use crate::parser::ExprNode;
use crate::parser::nodes::{ListNode, ObjectNode};
use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::rc::Rc;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(OrderedFloat<f64>),
    String(String),
    Bool(bool),
    Object(ObjectValue),
    List(ListValue),
    Function(FunctionValue),
    Null,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ObjectValue {
    pub properties: indexmap::IndexMap<Value, Rc<RefCell<Value>>>,
}

impl Hash for ObjectValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unreachable!("Shouldn't be hashing an object value")
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ListValue {
    pub elements: Vec<Rc<RefCell<Value>>>,
}

impl Hash for ListValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unreachable!("Shouldn't be hashing a list value")
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct FunctionValue {
    pub parameters: Vec<String>,
    pub body: CodeObject,
}

impl Debug for FunctionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}>{:?}",
            self.parameters.join(", "),
            self.body.operations
        )
    }
}

pub enum ValueError {
    InvalidOperation,
    InvalidType,
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

    pub fn object(properties: indexmap::IndexMap<Value, Rc<RefCell<Value>>>) -> Value {
        Value::Object(ObjectValue { properties })
    }

    pub fn list(elements: Vec<Rc<RefCell<Value>>>) -> Value {
        Value::List(ListValue { elements })
    }

    pub fn function(parameters: Vec<String>, body: CodeObject) -> Value {
        Value::Function(FunctionValue { parameters, body })
    }

    pub fn null() -> Value {
        Value::Null
    }

    pub fn try_const_from_object(node: ObjectNode) -> Result<Value, ValueError> {
        let mut obj_props = indexmap::IndexMap::new();
        for obj_prop in node.properties {
            obj_props.insert(
                Value::from_expr(obj_prop.key)?,
                Rc::new(RefCell::new(Value::from_expr(obj_prop.value)?)),
            );
        }
        Ok(Value::object(obj_props))
    }

    pub fn try_const_from_list(node: ListNode) -> Result<Value, ValueError> {
        let list_elements: Result<Vec<_>, _> = node
            .elements
            .into_iter()
            .map(|list_item| Value::from_expr(list_item).map(|v| Rc::new(RefCell::new(v))))
            .collect();
        Ok(Value::list(list_elements?))
    }

    pub fn from_expr(expr: ExprNode) -> Result<Value, ValueError> {
        match expr {
            ExprNode::Int(int_node) => Ok(Value::int(int_node.value)),
            ExprNode::Float(float_node) => Ok(Value::float(float_node.value)),
            ExprNode::String(string_node) => Ok(Value::string(string_node.value)),
            ExprNode::Bool(bool_node) => Ok(Value::bool(bool_node.value)),
            _ => Err(ValueError::InvalidType),
        }
    }

    pub fn bin_add(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
            (Value::Int(a), Value::Float(b)) => Value::Float(OrderedFloat(*a as f64) + *b),
            (Value::Float(a), Value::Int(b)) => Value::Float(*a + OrderedFloat(*b as f64)),
            (Value::Float(a), Value::Float(b)) => Value::Float(*a + *b),
            (Value::String(a), Value::String(b)) => Value::String(format!("{}{}", a, b)),
            _ => panic!("Invalid binary operation"),
        }
    }

    pub fn bin_sub(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::int(a - b),
            (Value::Int(a), Value::Float(b)) => Value::float(*a as f64 - **b),
            (Value::Float(a), Value::Int(b)) => Value::float(*a - *b as f64),
            (Value::Float(a), Value::Float(b)) => Value::float(*a - *b),
            _ => panic!("Invalid binary operation"),
        }
    }

    pub fn bin_div(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::float(*a as f64 / *b as f64),
            (Value::Int(a), Value::Float(b)) => Value::float(*a as f64 / **b),
            (Value::Float(a), Value::Int(b)) => Value::float(*a / *b as f64),
            (Value::Float(a), Value::Float(b)) => Value::float(*a / *b),
            _ => panic!("Invalid binary operation"),
        }
    }

    pub fn bin_int_div(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::int(a / b),
            (Value::Int(a), Value::Float(b)) => Value::float((*a as f64 / **b).floor()),
            (Value::Float(a), Value::Int(b)) => Value::float((*a / *b as f64).floor()),
            (Value::Float(a), Value::Float(b)) => Value::float((*a / *b).floor()),
            _ => panic!("Invalid binary operation"),
        }
    }

    pub fn bin_mul(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::int(a * b),
            (Value::Int(a), Value::Float(b)) => Value::float(*a as f64 * **b),
            (Value::Float(a), Value::Int(b)) => Value::float(*a * *b as f64),
            (Value::Float(a), Value::Float(b)) => Value::float(*a * *b),
            _ => panic!("Invalid binary operation"),
        }
    }

    pub fn bin_exp(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => {
                if *b < 0 {
                    return Value::float((*a as f64).powf(-b as f64));
                }
                Value::int(a.pow(*b as u32))
            }
            (Value::Int(a), Value::Float(b)) => Value::float((*a as f64).powf(**b)),
            (Value::Float(a), Value::Int(b)) => Value::float(a.powf(*b as f64)),
            (Value::Float(a), Value::Float(b)) => Value::float(a.powf(**b)),
            _ => panic!("Invalid binary operation"),
        }
    }

    pub fn bin_mod(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::int(a % b),
            (Value::Int(a), Value::Float(b)) => Value::float(*a as f64 % **b),
            (Value::Float(a), Value::Int(b)) => Value::float(*a % *b as f64),
            (Value::Float(a), Value::Float(b)) => Value::float(*a % *b),
            _ => panic!("Invalid binary operation"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodeObject {
    pub operations: Vec<OpIndex>,
    pub constants: Vec<Rc<RefCell<Value>>>,
    pub variables: Vec<Rc<RefCell<Value>>>,
    pub constant_index_lookup: HashMap<usize, usize>, // constant ExprNode id -> constant index
    pub variable_index_lookup: HashMap<String, usize>, // variable name -> variable index
}

impl CodeObject {
    pub fn push_op(&mut self, op: OpIndex) {
        self.operations.push(op);
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
