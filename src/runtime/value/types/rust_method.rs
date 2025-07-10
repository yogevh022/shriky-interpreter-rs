use crate::runtime::value::ValueRef;
use crate::runtime::value::methods::MethodFn;
use std::hash::Hash;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct RustMethodValue {
    pub id: usize,
    pub function: MethodFn,
    pub caller: Option<ValueRef>,
}

impl Hash for RustMethodValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}
