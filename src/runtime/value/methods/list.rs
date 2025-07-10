use crate::runtime::exceptions::RuntimeError;
use crate::runtime::utils::extract_int_ref;
use crate::runtime::value::methods::traits::{MethodFn, MethodProvider};
use crate::runtime::value::methods::utils::arg_check;
use crate::runtime::value::utils::extract_list;
use crate::runtime::value::{ListValue, Value, ValueRef};
use std::cell::RefCell;
use std::rc::Rc;

impl MethodProvider for ListValue {
    fn get_method(name: &str) -> Option<MethodFn> {
        match name {
            "push" => Some(ListValue::method_push),
            "pop" => Some(ListValue::method_pop),
            "pop_at" => Some(ListValue::method_pop_at),
            "get" => Some(ListValue::method_get),
            "remove" => Some(ListValue::method_remove),
            "len" => Some(ListValue::method_len),
            "is_empty" => Some(ListValue::method_is_empty),
            _ => None,
        }
    }
}

impl ListValue {
    pub fn method_push(
        list_value: &ValueRef,
        args: &[&ValueRef],
    ) -> Result<Option<ValueRef>, RuntimeError> {
        arg_check(args.len(), 1, "List.push")?;
        match &mut *list_value.borrow_mut() {
            Value::List(list) => {
                list.elements.push(args[0].clone());
                Ok(None)
            }
            _ => unreachable!(),
        }
    }

    pub fn method_pop(
        list_value: &ValueRef,
        args: &[&ValueRef],
    ) -> Result<Option<ValueRef>, RuntimeError> {
        arg_check(args.len(), 0, "List.pop")?;
        match &mut *list_value.borrow_mut() {
            Value::List(list) => {
                list.elements.pop().ok_or(RuntimeError::OutOfBounds(
                    "Called .pop() on an empty list".to_string(),
                ))?;
                Ok(None)
            }
            _ => unreachable!(),
        }
    }

    pub fn method_pop_at(
        list_value: &ValueRef,
        args: &[&ValueRef],
    ) -> Result<Option<ValueRef>, RuntimeError> {
        arg_check(args.len(), 1, "List.pop_at")?;
        let index = extract_int_ref(&args[0]) as usize;
        match &mut *list_value.borrow_mut() {
            Value::List(list) => {
                if list.elements.len() <= index {
                    return Ok(Some(list.elements.remove(index)));
                }
                Err(RuntimeError::OutOfBounds(format!(
                    "List index {} is out of bounds",
                    index
                )))
            }
            _ => unreachable!(),
        }
    }

    pub fn method_get(
        list_value: &ValueRef,
        args: &[&ValueRef],
    ) -> Result<Option<ValueRef>, RuntimeError> {
        arg_check(args.len(), 1, "List.get")?;
        let index = extract_int_ref(&args[0]) as usize;
        match &mut *list_value.borrow_mut() {
            Value::List(list) => {
                let result = list
                    .elements
                    .get(index)
                    .ok_or(RuntimeError::OutOfBounds(format!(
                        "List index {} is out of bounds",
                        index
                    )))?;
                Ok(Some(result.clone()))
            }
            _ => unreachable!(),
        }
    }

    pub fn method_remove(
        list_value: &ValueRef,
        args: &[&ValueRef],
    ) -> Result<Option<ValueRef>, RuntimeError> {
        arg_check(args.len(), 1, "List.remove")?;
        let index = extract_int_ref(&args[0]) as usize;
        match &mut *list_value.borrow_mut() {
            Value::List(list) => {
                list.elements.remove(index);
                Ok(None)
            }
            _ => unreachable!(),
        }
    }

    pub fn method_len(
        list_value: &ValueRef,
        args: &[&ValueRef],
    ) -> Result<Option<ValueRef>, RuntimeError> {
        arg_check(args.len(), 0, "List.len")?;
        match &mut *list_value.borrow_mut() {
            Value::List(list) => Ok(Some(Rc::new(RefCell::new(Value::int(
                list.elements.len() as i64
            ))))),
            _ => unreachable!(),
        }
    }

    pub fn method_is_empty(
        list_value: &ValueRef,
        args: &[&ValueRef],
    ) -> Result<Option<ValueRef>, RuntimeError> {
        arg_check(args.len(), 0, "List.is_empty")?;
        match &mut *list_value.borrow_mut() {
            Value::List(list) => Ok(Some(Rc::new(RefCell::new(Value::bool(
                list.elements.is_empty(),
            ))))),
            _ => unreachable!(),
        }
    }
}
