use crate::runtime::value::binary::traits::Binary;
use crate::runtime::value::exception;
use crate::runtime::value::{RuntimeException, Value};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct IntValue(pub i64);

impl Binary for IntValue {
    fn add(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match other {
            Value::Int(other) => Ok(Value::int(self.0 + other.0)),
            Value::Float(other) => Ok(Value::float(self.0 as f64 + other.0.0)),
            _ => Err(exception::INVALID_OPERATION
                .runtime(format!("Invalid binary operation {:?} + {:?}", self, other))),
        }
    }

    fn sub(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match other {
            Value::Int(other) => Ok(Value::int(self.0 - other.0)),
            Value::Float(other) => Ok(Value::float(self.0 as f64 - other.0.0)),
            _ => Err(exception::INVALID_OPERATION
                .runtime(format!("Invalid binary operation {:?} - {:?}", self, other))),
        }
    }

    fn mul(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match other {
            Value::Int(other) => Ok(Value::int(self.0 * other.0)),
            Value::Float(other) => Ok(Value::float(self.0 as f64 * other.0.0)),
            _ => Err(exception::INVALID_OPERATION.runtime(format!(
                "Invalid binary operation {:?} {} {:?}",
                self, "*", other
            ))),
        }
    }

    fn div(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match other {
            Value::Int(other) => {
                if other.0 == 0 {
                    return Err(exception::INVALID_OPERATION
                        .runtime(format!("Divide by zero {:?} / {:?}", self, other)));
                }
                Ok(Value::float(self.0 as f64 / other.0 as f64))
            },
            Value::Float(other) => {
                if other.0 == 0.0 {
                    return Err(exception::INVALID_OPERATION
                        .runtime(format!("Divide by zero {:?} / {:?}", self, other)));
                }
                Ok(Value::float(self.0 as f64 / other.0.0))
            },
            _ => Err(exception::INVALID_OPERATION
                .runtime(format!("Invalid binary operation {:?} / {:?}", self, other))),
        }
    }

    fn int_div(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match other {
            Value::Int(other) => Ok(Value::int(self.0 / other.0)),
            Value::Float(other) => Ok(Value::float((self.0 as f64 / other.0.0).floor())),
            _ => Err(exception::INVALID_OPERATION.runtime(format!(
                "Invalid binary operation {:?} {} {:?}",
                self, "//", other
            ))),
        }
    }

    fn modulus(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match other {
            Value::Int(other) => Ok(Value::int(self.0 % other.0)),
            Value::Float(other) => Ok(Value::float(self.0 as f64 % other.0.0)),
            _ => Err(exception::INVALID_OPERATION.runtime(format!(
                "Invalid binary operation {:?} {} {:?}",
                self, "%", other
            ))),
        }
    }

    fn pow(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match other {
            Value::Int(other) => {
                if other.0 < 0 {
                    return Ok(Value::float((self.0 as f64).powi(other.0 as i32)));
                }
                Ok(Value::int(self.0.pow(other.0 as u32)))
            }
            Value::Float(other) => Ok(Value::float((self.0 as f64).powf(other.0.0))),
            _ => Err(exception::INVALID_OPERATION.runtime(format!(
                "Invalid binary operation {:?} {} {:?}",
                self, "**", other
            ))),
        }
    }

    fn equals(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match other {
            Value::Int(other) => Ok(Value::bool(self.0 == other.0)),
            Value::Float(other) => Ok(Value::bool(self.0 as f64 == other.0.0)),
            _ => Ok(Value::bool(false)),
        }
    }

    fn greater(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match other {
            Value::Int(other) => Ok(Value::bool(self.0 > other.0)),
            Value::Float(other) => Ok(Value::bool(self.0 as f64 > other.0.0)),
            _ => Ok(Value::bool(false)),
        }
    }

    fn greater_equals(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match other {
            Value::Int(other) => Ok(Value::bool(self.0 >= other.0)),
            Value::Float(other) => Ok(Value::bool(self.0 as f64 >= other.0.0)),
            _ => Ok(Value::bool(false)),
        }
    }

    fn less(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match other {
            Value::Int(other) => Ok(Value::bool(self.0 < other.0)),
            Value::Float(other) => Ok(Value::bool((self.0 as f64) < other.0.0)),
            _ => Ok(Value::bool(false)),
        }
    }

    fn less_equals(&mut self, other: &Value) -> Result<Value, RuntimeException> {
        match other {
            Value::Int(other) => Ok(Value::bool(self.0 <= other.0)),
            Value::Float(other) => Ok(Value::bool(self.0 as f64 <= other.0.0)),
            _ => Ok(Value::bool(false)),
        }
    }
}
