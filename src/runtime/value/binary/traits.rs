use crate::runtime::value::{RuntimeException, Value};

pub trait Binary {
    fn add(&mut self, other: &Value) -> Result<Value, RuntimeException>;
    fn sub(&mut self, other: &Value) -> Result<Value, RuntimeException>;
    fn mul(&mut self, other: &Value) -> Result<Value, RuntimeException>;
    fn div(&mut self, other: &Value) -> Result<Value, RuntimeException>;
    fn int_div(&mut self, other: &Value) -> Result<Value, RuntimeException>;
    fn modulus(&mut self, other: &Value) -> Result<Value, RuntimeException>;
    fn pow(&mut self, other: &Value) -> Result<Value, RuntimeException>;
    fn equals(&mut self, other: &Value) -> Result<Value, RuntimeException>;
    fn greater(&mut self, other: &Value) -> Result<Value, RuntimeException>;
    fn greater_equals(&mut self, other: &Value) -> Result<Value, RuntimeException>;
    fn less(&mut self, other: &Value) -> Result<Value, RuntimeException>;
    fn less_equals(&mut self, other: &Value) -> Result<Value, RuntimeException>;
}
