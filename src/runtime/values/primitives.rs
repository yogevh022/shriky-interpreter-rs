use ordered_float::OrderedFloat;
use super::traits::*;
use crate::runtime::environment::Counter;

#[derive(Eq, Clone, Debug)]
pub struct IntValue {
    pub id: u64,
    pub value: i64,
}

impl PartialEq for IntValue {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl Default for IntValue {
    fn default() -> Self {
        IntValue {
            value: 0,
            id: Counter.next()
        }
    }
}

impl HasId for IntValue {
    fn id(&self) -> u64 { self.id }
}

#[derive(Eq, Clone, Debug)]
pub struct FloatValue {
    pub id: u64,
    pub value: OrderedFloat<f64>,
}

impl PartialEq for FloatValue {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl HasId for FloatValue {
    fn id(&self) -> u64 { self.id }
}

#[derive(Eq, Clone, Debug)]
pub struct BoolValue {
    pub id: u64,
    pub value: bool,
}

impl PartialEq for BoolValue {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl HasId for BoolValue {
    fn id(&self) -> u64 { self.id }
}

#[derive(Eq, Clone, Debug)]
pub struct StringValue {
    pub id: u64,
    pub value: String,
}

impl StringValue {
    pub fn from(s: &str) -> StringValue {
        StringValue {
            id: Counter.next(),
            value: s.to_string(),
        }
    }
}

impl PartialEq for StringValue {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl HasId for StringValue {
    fn id(&self) -> u64 { self.id }
}