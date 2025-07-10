use crate::runtime::value::{FunctionValue, ValueRef};
use std::hash::Hash;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct MethodValue {
    pub id: usize,
    pub function: FunctionValue,
    pub caller: Option<ValueRef>,
}

impl Hash for MethodValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}
