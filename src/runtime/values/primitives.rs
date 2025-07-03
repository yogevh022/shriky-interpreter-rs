use super::traits::*;
use crate::runtime::environment::Counter;
use ordered_float::OrderedFloat;
use std::fmt;

#[derive(Eq, Clone, Debug)]
pub struct IntValue {
    pub id: usize,
    pub value: i64,
}

impl From<i64> for IntValue {
    fn from(value: i64) -> Self {
        Self {
            value,
            id: Counter.next(),
        }
    }
}

impl fmt::Display for IntValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
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
            id: Counter.next(),
        }
    }
}

impl HasId for IntValue {
    fn id(&self) -> usize {
        self.id
    }
}

#[derive(Eq, Clone, Debug)]
pub struct FloatValue {
    pub id: usize,
    pub value: OrderedFloat<f64>,
}

impl From<f64> for FloatValue {
    fn from(value: f64) -> Self {
        Self {
            value: OrderedFloat(value),
            id: Counter.next(),
        }
    }
}

impl fmt::Display for FloatValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<OrderedFloat<f64>> for FloatValue {
    fn from(value: OrderedFloat<f64>) -> Self {
        Self {
            value,
            id: Counter.next(),
        }
    }
}

impl PartialEq for FloatValue {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl HasId for FloatValue {
    fn id(&self) -> usize {
        self.id
    }
}

#[derive(Eq, Clone, Debug)]
pub struct BoolValue {
    pub id: usize,
    pub value: bool,
}

impl From<bool> for BoolValue {
    fn from(value: bool) -> Self {
        Self {
            value,
            id: Counter.next(),
        }
    }
}

impl fmt::Display for BoolValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialEq for BoolValue {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl HasId for BoolValue {
    fn id(&self) -> usize {
        self.id
    }
}

#[derive(Eq, Clone, Debug)]
pub struct StringValue {
    pub id: usize,
    pub value: String,
}

impl From<String> for StringValue {
    fn from(s: String) -> Self {
        Self {
            value: s,
            id: Counter.next(),
        }
    }
}

impl From<&str> for StringValue {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
            id: Counter.next(),
        }
    }
}

impl fmt::Display for StringValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialEq for StringValue {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl HasId for StringValue {
    fn id(&self) -> usize {
        self.id
    }
}
