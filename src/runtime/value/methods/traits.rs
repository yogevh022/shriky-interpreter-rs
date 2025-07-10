use crate::runtime::exceptions::RuntimeError;
use crate::runtime::value::ValueRef;

pub type MethodFn = fn(&ValueRef, &[&ValueRef]) -> Result<Option<ValueRef>, RuntimeError>;

pub trait MethodProvider {
    fn get_method(name: &str) -> Option<MethodFn>
    where
        Self: Sized;
}
