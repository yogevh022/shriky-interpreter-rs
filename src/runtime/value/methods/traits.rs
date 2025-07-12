use crate::runtime::value::{RuntimeException, ValueRef};

pub type MethodFn = fn(&ValueRef, &[&ValueRef]) -> Result<Option<ValueRef>, RuntimeException>;

pub trait MethodProvider {
    fn get_method(name: &str) -> Option<MethodFn>
    where
        Self: Sized;
}
