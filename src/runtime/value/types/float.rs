use crate::runtime::exceptions::RuntimeError;
use crate::runtime::value::Value;
use crate::runtime::value::binary::traits::Binary;
use crate::runtime::value::types::int::IntValue;
use ordered_float::{OrderedFloat, Pow};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct FloatValue(pub OrderedFloat<f64>);

impl Binary for FloatValue {
    fn add(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        match other {
            Value::Int(other) => Ok(Value::float(self.0 + other.0 as f64)),
            Value::Float(other) => Ok(Value::float(self.0 + other.0)),
            _ => Err(RuntimeError::InvalidOperation(format!(
                "Invalid binary operation {:?} + {:?}",
                self, other
            ))),
        }
    }

    fn sub(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        match other {
            Value::Int(other) => Ok(Value::float(self.0 - other.0 as f64)),
            Value::Float(other) => Ok(Value::float(self.0 - other.0)),
            _ => Err(RuntimeError::InvalidOperation(format!(
                "Invalid binary operation {:?} - {:?}",
                self, other
            ))),
        }
    }

    fn mul(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        match other {
            Value::Int(other) => Ok(Value::float(self.0 * other.0 as f64)),
            Value::Float(other) => Ok(Value::float(self.0 * other.0)),
            _ => Err(RuntimeError::InvalidOperation(format!(
                "Invalid binary operation {:?} {} {:?}",
                self, "*", other
            ))),
        }
    }

    fn div(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        match other {
            Value::Int(other) => Ok(Value::float(self.0 / other.0 as f64)),
            Value::Float(other) => Ok(Value::float(self.0 / other.0)),
            _ => Err(RuntimeError::InvalidOperation(format!(
                "Invalid binary operation {:?} / {:?}",
                self, other
            ))),
        }
    }

    fn int_div(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        match other {
            Value::Int(other) => Ok(Value::float(self.0 / other.0 as f64)),
            Value::Float(other) => Ok(Value::float((self.0 / other.0).floor())),
            _ => Err(RuntimeError::InvalidOperation(format!(
                "Invalid binary operation {:?} {} {:?}",
                self, "//", other
            ))),
        }
    }

    fn modulus(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        match other {
            Value::Int(other) => Ok(Value::float(self.0 % other.0 as f64)),
            Value::Float(other) => Ok(Value::float(self.0 % other.0)),
            _ => Err(RuntimeError::InvalidOperation(format!(
                "Invalid binary operation {:?} {} {:?}",
                self, "%", other
            ))),
        }
    }

    fn pow(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        match other {
            Value::Int(other) => Ok(Value::float(self.0.powi(other.0 as i32))),
            Value::Float(other) => Ok(Value::float(self.0.powf(other.0.0))),
            _ => Err(RuntimeError::InvalidOperation(format!(
                "Invalid binary operation {:?} {} {:?}",
                self, "**", other
            ))),
        }
    }

    fn equals(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        match other {
            Value::Int(other) => Ok(Value::bool(self.0.0 == other.0 as f64)),
            Value::Float(other) => Ok(Value::bool(self.0 == other.0)),
            _ => Ok(Value::bool(false)),
        }
    }

    fn greater(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        match other {
            Value::Int(other) => Ok(Value::bool(self.0.0 > other.0 as f64)),
            Value::Float(other) => Ok(Value::bool(self.0 > other.0)),
            _ => Ok(Value::bool(false)),
        }
    }

    fn greater_equals(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        match other {
            Value::Int(other) => Ok(Value::bool(self.0.0 >= other.0 as f64)),
            Value::Float(other) => Ok(Value::bool(self.0 >= other.0)),
            _ => Ok(Value::bool(false)),
        }
    }

    fn less(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        match other {
            Value::Int(other) => Ok(Value::bool(self.0.0 < other.0 as f64)),
            Value::Float(other) => Ok(Value::bool(self.0 < other.0)),
            _ => Ok(Value::bool(false)),
        }
    }

    fn less_equals(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        match other {
            Value::Int(other) => Ok(Value::bool(self.0.0 <= other.0 as f64)),
            Value::Float(other) => Ok(Value::bool(self.0 <= other.0)),
            _ => Ok(Value::bool(false)),
        }
    }
}
