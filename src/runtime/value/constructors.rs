use std::borrow::Cow;
use crate::compiler::code_object::CodeObject;
use crate::parser::ExprNode;
use crate::parser::nodes::{ListNode, MapNode};
use crate::runtime::value::base::{RUNTIME_VALUE_ID, ValueError};
use crate::runtime::value::methods::MethodFn;
use crate::runtime::value::types::bool::BoolValue;
use crate::runtime::value::types::float::FloatValue;
use crate::runtime::value::types::int::IntValue;
use crate::runtime::value::types::rust_method::RustMethodValue;
use crate::runtime::value::types::string::StringValue;
use crate::runtime::value::*;
use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Value {
    pub fn int(value: i64) -> Value {
        Value::Int(IntValue(value))
    }

    pub fn float<T: Into<OrderedFloat<f64>>>(value: T) -> Value {
        Value::Float(FloatValue(value.into()))
    }

    pub fn string(value: String) -> Value {
        Value::String(StringValue(value))
    }

    pub fn bool(value: bool) -> Value {
        Value::Bool(BoolValue(value))
    }

    pub fn map(properties: HashMap<Value, ValueRef>) -> Value {
        Value::Map(MapValue { properties })
    }

    pub fn list(elements: Vec<ValueRef>) -> Value {
        Value::List(ListValue { elements })
    }

    pub fn function(parameters: Vec<String>, body: CodeObject) -> Value {
        Value::Function(FunctionValue {
            id: RUNTIME_VALUE_ID.next(),
            parameters,
            body,
        })
    }

    pub fn method(function: FunctionValue, caller: Option<ValueRef>) -> Value {
        Value::Method(MethodValue {
            id: RUNTIME_VALUE_ID.next(),
            function,
            caller,
        })
    }

    pub fn class(parent: Option<ValueRef>, body: CodeObject) -> Value {
        Value::Class(ClassValue {
            id: RUNTIME_VALUE_ID.next(),
            parent, // will always be ClassValue
            body,
        })
    }

    pub fn instance(class: ValueRef, attributes: HashMap<String, ValueRef>) -> Value {
        Value::Instance(InstanceValue {
            id: RUNTIME_VALUE_ID.next(),
            class,
            attributes,
        })
    }

    pub fn try_const_from_map(node: MapNode) -> Result<Value, ValueError> {
        let mut obj_props = HashMap::new();
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

    pub fn rust_method(function: MethodFn, caller: Option<ValueRef>) -> Value {
        Value::RustMethod(RustMethodValue {
            id: RUNTIME_VALUE_ID.next(),
            function,
            caller,
        })
    }
    
    pub fn exception(kind: String) -> Value {
        Value::Exception(ExceptionValue(Cow::Owned(kind)))
    }
}
