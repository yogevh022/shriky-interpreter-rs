use crate::compiler::code_object::CodeObject;
use std::fmt::Debug;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct FunctionValue {
    pub id: usize,
    pub parameters: Vec<String>,
    pub body: CodeObject,
}

impl FunctionValue {
    pub fn new(parameters: Vec<String>, body: CodeObject) -> Self {
        Self {
            id: crate::runtime::value::base::RUNTIME_VALUE_ID.next(),
            parameters,
            body,
        }
    }
}

impl Debug for FunctionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}>{:?}",
            self.parameters.join(", "),
            self.body.operations
        )
    }
}
