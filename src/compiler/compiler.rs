use crate::parser::ExprNode;
use crate::parser::nodes::{AssignNode, IdentityNode};
use crate::compiler::byte_operations::ByteOperation;

pub struct Compiler {
    ast: Vec<ExprNode>,
}

impl Compiler {
    pub fn new(ast: Vec<ExprNode>) -> Self {
        Self { ast }
    }

    fn compile_identity(&mut self, identity_node: IdentityNode) {
        let mut byte_ops: Vec<ByteOperation> = Vec::new();
        byte_ops.push(ByteOperation::LoadName())
        for part in identity_node.address {
            match part {
                ExprNode::String(string_part) => {ByteOperation::},
                ExprNode::Int(int_part) => {},
            }
        }
    }

    fn compile_assignment(&mut self, assign_node: AssignNode) {
        assign_node.identity
    }

    pub fn compile(&self) {

    }
}