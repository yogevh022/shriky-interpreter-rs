use crate::compiler::byte_operations::ByteOperation;
use crate::parser::ExprNode;
use crate::parser::nodes::{AssignNode, IdentityNode};

pub struct Compiler {
    ast: Vec<ExprNode>,
}

impl Compiler {
    pub fn new(ast: Vec<ExprNode>) -> Self {
        Self { ast }
    }

    pub fn compile(&self) {}
}
