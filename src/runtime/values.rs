use crate::compiler::code_object::CodeObject;
use crate::parser::ExprNode;
use crate::parser::nodes::{ListNode, MapNode};
use crate::utils::counter::Counter;
use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(OrderedFloat<f64>),
    String(String),
    Bool(bool),
    Map(MapValue),
    List(ListValue),
    Function(FunctionValue),
    Class(ClassValue),
    Instance(InstanceValue),
    Null,
}

static RUNTIME_VALUE_COUNTER: Counter = Counter::new();

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

    pub fn map(properties: indexmap::IndexMap<Value, Rc<RefCell<Value>>>) -> Value {
        Value::Map(MapValue { properties })
    }

    pub fn list(elements: Vec<Rc<RefCell<Value>>>) -> Value {
        Value::List(ListValue { elements })
    }

    pub fn function(parameters: Vec<String>, body: CodeObject) -> Value {
        Value::Function(FunctionValue {
            id: RUNTIME_VALUE_COUNTER.next(),
            parameters,
            body,
        })
    }

    pub fn class(parent: Option<Rc<RefCell<Value>>>, body: CodeObject) -> Value {
        Value::Class(ClassValue {
            id: RUNTIME_VALUE_COUNTER.next(),
            parent, // will always be ClassValue
            body,
        })
    }

    pub fn instance(
        class: Rc<RefCell<Value>>,
        attributes: HashMap<String, Rc<RefCell<Value>>>,
    ) -> Value {
        Value::Instance(InstanceValue {
            id: RUNTIME_VALUE_COUNTER.next(),
            class,
            attributes,
        })
    }

    pub fn null() -> Value {
        Value::Null
    }

    pub fn try_const_from_map(node: MapNode) -> Result<Value, ValueError> {
        let mut obj_props = indexmap::IndexMap::new();
        for obj_prop in node.properties {
            obj_props.insert(
                Value::from_expr(obj_prop.key)?,
                Rc::new(RefCell::new(Value::from_expr(obj_prop.value)?)),
            );
        }
        Ok(Value::map(obj_props))
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

    pub fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Int(a), Value::Float(b)) => *a as f64 == **b,
            (Value::Float(a), Value::Int(b)) => *a == *b as f64,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }

    pub fn greater_than(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a > b,
            (Value::Int(a), Value::Float(b)) => *a as f64 > **b,
            (Value::Float(a), Value::Float(b)) => a > b,
            (Value::String(a), Value::String(b)) => a.len() > b.len(),
            (Value::Bool(a), Value::Bool(b)) => a > b,
            _ => false,
        }
    }

    pub fn greater_than_equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a >= b,
            (Value::Int(a), Value::Float(b)) => *a as f64 >= **b,
            (Value::Float(a), Value::Float(b)) => a >= b,
            (Value::String(a), Value::String(b)) => a.len() >= b.len(),
            (Value::Bool(a), Value::Bool(b)) => a >= b,
            _ => false,
        }
    }

    pub fn less_than(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a < b,
            (Value::Int(a), Value::Float(b)) => (*a as f64) < **b,
            (Value::Float(a), Value::Float(b)) => a < b,
            (Value::String(a), Value::String(b)) => a.len() < b.len(),
            (Value::Bool(a), Value::Bool(b)) => a < b,
            _ => false,
        }
    }

    pub fn less_than_equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a <= b,
            (Value::Int(a), Value::Float(b)) => (*a as f64) <= **b,
            (Value::Float(a), Value::Float(b)) => a <= b,
            (Value::String(a), Value::String(b)) => a.len() <= b.len(),
            (Value::Bool(a), Value::Bool(b)) => a <= b,
            _ => false,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Null => false,
            Value::Map(o) => !o.properties.is_empty(),
            Value::List(l) => !l.elements.is_empty(),
            Value::Function(_) => true,
            Value::Class(_) => true,
            Value::Instance(_) => true,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct MapValue {
    pub properties: indexmap::IndexMap<Value, Rc<RefCell<Value>>>,
}

impl Hash for MapValue {
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
    pub id: usize,
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

#[derive(Eq, Clone, Debug)]
pub struct ClassValue {
    pub id: usize,
    pub parent: Option<Rc<RefCell<Value>>>, // Will always be ClassValue
    pub body: CodeObject,
}

impl Hash for ClassValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for ClassValue {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct InstanceValue {
    pub id: usize,
    pub class: Rc<RefCell<Value>>,
    pub attributes: HashMap<String, Rc<RefCell<Value>>>,
}

impl Hash for InstanceValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

pub enum ValueError {
    InvalidOperation,
    InvalidType,
}
