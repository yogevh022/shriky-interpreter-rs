use std::collections::HashMap;
use crate::compiler::byte_operations::ByteOperation;
use crate::parser::ExprNode;
use crate::parser::nodes::{AccessConstantNode, AssignNode, IdentityNode, LiteralNode};
use crate::utils::counter::Counter;

pub struct Compiler {
    ast: Vec<ExprNode>,
    counter: Counter,
    constants: HashMap<LiteralNode, usize>,
}

impl Compiler {
    pub fn new(ast: Vec<ExprNode>) -> Self {
        Self {
            ast,
            counter: Counter::new(),
        }
    }

    fn identity(&self, identity_node: &IdentityNode) {}

    fn access_constant(&self, access_constant_node: &AccessConstantNode) {
        let mut stack: Vec<(ByteOperation, usize)> = Vec::new();
        if let Some(constant_index) = self.constants.get(&access_constant_node.value.to_literal()) {
            
        }
        access_constant_node.
    }
    
    fn assign(&self, assign_node: &AssignNode) {
        let mut stack: Vec<ByteOperation> = Vec::new();
        // assign_node.
    }

    pub fn compile(&self) {
        for node in self.ast.iter() {
            match node {
                ExprNode::Assign(assign_node) => self.assign(assign_node),
                _ => panic!(),
            }
        }
    }
}
