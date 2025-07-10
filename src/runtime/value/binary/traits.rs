use crate::runtime::exceptions::RuntimeError;
use crate::runtime::value::Value;

pub trait Binary {
    fn add(&mut self, other: &Value) -> Result<Value, RuntimeError>;
    fn sub(&mut self, other: &Value) -> Result<Value, RuntimeError>;
    fn mul(&mut self, other: &Value) -> Result<Value, RuntimeError>;
    fn div(&mut self, other: &Value) -> Result<Value, RuntimeError>;
    fn int_div(&mut self, other: &Value) -> Result<Value, RuntimeError>;
    fn modulus(&mut self, other: &Value) -> Result<Value, RuntimeError>;
    fn pow(&mut self, other: &Value) -> Result<Value, RuntimeError>;
    fn equals(&mut self, other: &Value) -> Result<Value, RuntimeError>;
    fn greater(&mut self, other: &Value) -> Result<Value, RuntimeError>;
    fn greater_equals(&mut self, other: &Value) -> Result<Value, RuntimeError>;
    fn less(&mut self, other: &Value) -> Result<Value, RuntimeError>;
    fn less_equals(&mut self, other: &Value) -> Result<Value, RuntimeError>;
}
