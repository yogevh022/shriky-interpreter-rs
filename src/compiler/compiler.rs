use crate::compiler::byte_operations::*;
use crate::lexer::TokenKind;
use crate::parser::ExprNode;
use crate::parser::nodes::*;
use crate::parser::traits::HasId;
use crate::utils::counter::Counter;
use std::collections::HashMap;

pub struct Compiler {
    counter: Counter,
    literals: Vec<ExprNode>,
    variables: Vec<String>,
    variable_index_lookup: HashMap<String, usize>, // variable name -> variable index
    literal_index_lookup: HashMap<usize, usize>,   // literal ExprNode id -> literal index
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            counter: Counter::new(),
            literals: Vec::new(),
            variables: Vec::new(),
            variable_index_lookup: HashMap::new(),
            literal_index_lookup: HashMap::new(),
        }
    }

    fn binary(&mut self, operations: &mut Vec<OperationIndex>, binary_node: BinaryNode) {
        operations.extend(self.compile_expr(*binary_node.left));
        operations.extend(self.compile_expr(*binary_node.right));
        match binary_node.operator {
            TokenKind::Plus => operations.push(OperationIndex::new(ByteOperation::Add, 0)),
            TokenKind::Minus => operations.push(OperationIndex::new(ByteOperation::Sub, 0)),
            TokenKind::Asterisk => operations.push(OperationIndex::new(ByteOperation::Mul, 0)),
            TokenKind::Slash => operations.push(OperationIndex::new(ByteOperation::Div, 0)),
            TokenKind::DoubleSlash => {
                operations.push(OperationIndex::new(ByteOperation::IntDiv, 0))
            }
            TokenKind::Exponent => operations.push(OperationIndex::new(ByteOperation::Exp, 0)),
            TokenKind::Modulo => operations.push(OperationIndex::new(ByteOperation::Mod, 0)),
            _ => unreachable!("Expected binary operator, got: {:?}", binary_node.operator),
        }
    }

    fn identity(&mut self, operations: &mut Vec<OperationIndex>, identity: IdentityNode) {
        let mut identity_address_iter = identity.address.into_iter();
        if let Some(ExprNode::String(identity_base)) = identity_address_iter.next() {
            self.load_name(operations, identity_base);
        } else {
            panic!(
                "Unexpected identity base: {:?}",
                identity_address_iter.collect::<Vec<_>>()
            );
        }
        for part in identity_address_iter {
            match part {
                ExprNode::AccessLiteral(access_literal_node) => {
                    self.access_literal(operations, access_literal_node)
                }
                ExprNode::AccessAttribute(access_attribute_node) => {
                    self.access_attribute(operations, access_attribute_node)
                }
                _ => panic!("Unexpected identity part: {:?}", part),
            }
        }
    }

    fn cache_literal(&mut self, literal: ExprNode) -> usize {
        let new_literal_index = self.literals.len();
        self.literal_index_lookup
            .insert(literal.id(), new_literal_index);
        self.literals.push(literal);
        new_literal_index
    }

    fn cache_variable(&mut self, name: String) -> usize {
        let new_variable_index = self.variables.len();
        self.variable_index_lookup
            .insert(name.clone(), new_variable_index);
        self.variables.push(name);
        new_variable_index
    }

    fn load_name(&mut self, operations: &mut Vec<OperationIndex>, node: StringNode) {
        if let Some(var_index) = self.variable_index_lookup.get(&node.value) {
            operations.push(OperationIndex::new(ByteOperation::LoadName, *var_index));
            return;
        };
        let var_index = self.cache_variable(node.value);
        operations.push(OperationIndex::new(ByteOperation::LoadName, var_index))
    }

    fn access_literal(&mut self, operations: &mut Vec<OperationIndex>, node: AccessConstantNode) {
        let literal_index = self.cache_literal(*node.value);
        operations.extend([
            OperationIndex::new(ByteOperation::LoadLiteral, literal_index),
            OperationIndex::new(ByteOperation::BinarySubscribe, 0),
        ]);
    }

    fn access_attribute(
        &mut self,
        operations: &mut Vec<OperationIndex>,
        access_attribute_node: AccessAttributeNode,
    ) {
        todo!()
    }

    fn assign(&mut self, operations: &mut Vec<OperationIndex>, assign_node: AssignNode) {
        self.identity(operations, assign_node.identity);
        operations.extend(self.compile_expr(*assign_node.value));
        operations.push(OperationIndex::new(ByteOperation::Assign, 0));
    }

    pub fn compile_expr(&mut self, expr: ExprNode) -> Vec<OperationIndex> {
        let mut operations: Vec<OperationIndex> = Vec::new();
        match expr {
            ExprNode::Identity(identity_node) => self.identity(&mut operations, identity_node),
            ExprNode::Assign(assign_node) => self.assign(&mut operations, assign_node),
            ExprNode::Int(_) | ExprNode::Float(_) | ExprNode::Bool(_) | ExprNode::String(_) => {
                let literal_index = self.cache_literal(expr);
                operations.push(OperationIndex::new(
                    ByteOperation::LoadLiteral,
                    literal_index,
                ));
            }
            ExprNode::Binary(binary_node) => self.binary(&mut operations, binary_node),
            _ => panic!("Unexpected expr node: {:?}", expr),
        }
        operations
    }

    pub fn compile(&mut self, ast: Vec<ExprNode>) {
        todo!()
    }
}
