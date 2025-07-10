use crate::runtime::exceptions::RuntimeError;
use crate::runtime::value::Value;
use crate::runtime::value::binary::traits::Binary;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct StringValue(pub String);

impl Binary for StringValue {
    fn add(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        match other {
            Value::String(other) => Ok(Value::string(format!("{}{}", self.0, other.0))),
            _ => Err(RuntimeError::InvalidOperation(format!(
                "Invalid binary operation: {:?} + {:?}",
                self, other
            ))),
        }
    }

    fn sub(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        Err(RuntimeError::InvalidOperation(format!(
            "Invalid binary operation: {:?} - {:?}",
            self, other
        )))
    }

    fn mul(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        Err(RuntimeError::InvalidOperation(format!(
            "Invalid binary operation: {:?} * {:?}",
            self, other
        )))
    }

    fn div(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        Err(RuntimeError::InvalidOperation(format!(
            "Invalid binary operation: {:?} / {:?}",
            self, other
        )))
    }

    fn int_div(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        Err(RuntimeError::InvalidOperation(format!(
            "Invalid binary operation: {:?} // {:?}",
            self, other
        )))
    }

    fn modulus(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        Err(RuntimeError::InvalidOperation(format!(
            "Invalid binary operation: {:?} % {:?}",
            self, other
        )))
    }

    fn pow(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        Err(RuntimeError::InvalidOperation(format!(
            "Invalid binary operation: {:?} ** {:?}",
            self, other
        )))
    }

    fn equals(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        match other {
            Value::String(other) => Ok(Value::bool(self.0 == other.0)),
            _ => Ok(Value::bool(false)),
        }
    }

    fn greater(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        Err(RuntimeError::InvalidOperation(format!(
            "Invalid binary operation: {:?} > {:?}",
            self, other
        )))
    }

    fn greater_equals(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        Err(RuntimeError::InvalidOperation(format!(
            "Invalid binary operation: {:?} >= {:?}",
            self, other
        )))
    }

    fn less(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        Err(RuntimeError::InvalidOperation(format!(
            "Invalid binary operation: {:?} < {:?}",
            self, other
        )))
    }

    fn less_equals(&mut self, other: &Value) -> Result<Value, RuntimeError> {
        Err(RuntimeError::InvalidOperation(format!(
            "Invalid binary operation: {:?} <= {:?}",
            self, other
        )))
    }
}
