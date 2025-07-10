use crate::runtime::value::{ListValue, MapValue, Value};

pub fn extract_map(map_value: &mut Value) -> &mut MapValue {
    match map_value {
        Value::Map(map) => map,
        _ => unreachable!("Expected map value"),
    }
}

pub fn extract_list(list_value: &mut Value) -> &mut ListValue {
    match list_value {
        Value::List(list) => list,
        _ => unreachable!("Expected list value"),
    }
}
