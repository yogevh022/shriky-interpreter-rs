use crate::runtime::value::exception;
use crate::runtime::value::methods::traits::{MethodFn, MethodProvider};
use crate::runtime::value::methods::utils::arg_check;
use crate::runtime::value::{MapValue, RuntimeException, Value, ValueRef};
use std::cell::RefCell;
use std::rc::Rc;

impl MethodProvider for MapValue {
    fn get_method(name: &str) -> Option<MethodFn> {
        match name {
            "insert" => Some(MapValue::method_insert),
            "get" => Some(MapValue::method_get),
            "remove" => Some(MapValue::method_remove),
            "len" => Some(MapValue::method_len),
            "is_empty" => Some(MapValue::method_is_empty),
            _ => None,
        }
    }
}

impl MapValue {
    pub fn method_insert(
        map_value: &ValueRef,
        args: &[&ValueRef],
    ) -> Result<Option<ValueRef>, RuntimeException> {
        arg_check(args.len(), 2, "Map.insert")?;
        match &mut *map_value.borrow_mut() {
            Value::Map(map) => {
                map.properties
                    .insert(args[0].borrow().clone(), args[1].clone());
            }
            _ => unreachable!(),
        }
        Ok(None)
    }

    pub fn method_get(
        map_value: &ValueRef,
        args: &[&ValueRef],
    ) -> Result<Option<ValueRef>, RuntimeException> {
        arg_check(args.len(), 1, "Map.get")?;
        match &mut *map_value.borrow_mut() {
            Value::Map(map) => {
                let key = &*args[0].borrow();
                let result = map.properties.get(key).ok_or(
                    exception::KEY_ERROR.runtime(format!("Entry with key {:?} not found", key)),
                )?;
                Ok(Some(result.clone()))
            }
            _ => unreachable!(),
        }
    }

    pub fn method_remove(
        map_value: &ValueRef,
        args: &[&ValueRef],
    ) -> Result<Option<ValueRef>, RuntimeException> {
        arg_check(args.len(), 1, "Map.remove")?;
        match &mut *map_value.borrow_mut() {
            Value::Map(map) => {
                let key = &*args[0].borrow();
                map.properties.shift_remove(key).ok_or(
                    exception::KEY_ERROR.runtime(format!("Entry with key {:?} not found", key)),
                )?;
                Ok(None)
            }
            _ => unreachable!(),
        }
    }

    pub fn method_len(
        map_value: &ValueRef,
        args: &[&ValueRef],
    ) -> Result<Option<ValueRef>, RuntimeException> {
        arg_check(args.len(), 0, "Map.len")?;
        match &mut *map_value.borrow_mut() {
            Value::Map(map) => Ok(Some(Rc::new(RefCell::new(Value::int(
                map.properties.len() as i64,
            ))))),
            _ => unreachable!(),
        }
    }

    pub fn method_is_empty(
        map_value: &ValueRef,
        args: &[&ValueRef],
    ) -> Result<Option<ValueRef>, RuntimeException> {
        arg_check(args.len(), 0, "Map.is_empty")?;
        match &mut *map_value.borrow_mut() {
            Value::Map(map) => Ok(Some(Rc::new(RefCell::new(Value::bool(
                map.properties.is_empty(),
            ))))),
            _ => unreachable!(),
        }
    }
}
