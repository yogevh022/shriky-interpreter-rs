use crate::compiler::code_object::CodeObject;
use crate::runtime::value::ValueRef;
use std::hash::Hash;

#[derive(Eq, Clone, Debug)]
pub struct ClassValue {
    pub id: usize,
    pub parent: Option<ValueRef>, // Will always be ClassValue
    pub body: CodeObject,
}

impl Hash for ClassValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for ClassValue {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
