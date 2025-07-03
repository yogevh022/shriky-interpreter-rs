use crate::compiler::byte_operations::*;
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

    fn identity(&mut self, identity: ExprNode) -> Vec<OperationIndex> {
        match identity {
            ExprNode::AccessLiteral(access_literal_node) => {
                self.access_literal(access_literal_node)
            },
            ExprNode::AccessAttribute(access_attribute_node) => {
                self.access_attribute(access_attribute_node)
            },
            ExprNode::String(string_node) => {
                self.load_name(string_node)
            }
            _ => panic!("Unexpected identity node: {:?}", identity),
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
    
    fn load_literal(&self, literal_index: usize) -> Vec<OperationIndex> {
        vec![OperationIndex::new(ByteOperation::LoadLiteral, literal_index)]
    }
    
    fn load_name(&mut self, node: StringNode) -> Vec<OperationIndex> {
        if let Some(var_index) = self.variable_index_lookup.get(&node.value) {
            return vec![OperationIndex::new(ByteOperation::LoadName, *var_index)];
        };
        let var_index = self.cache_variable(node.value);
        vec![OperationIndex::new(ByteOperation::LoadName, var_index)]
    }

    fn access_literal(&mut self, node: AccessLiteralNode) -> Vec<OperationIndex> {
        let literal_index = self.cache_literal(*node.value);
        let mut operations = self.load_literal(literal_index);
        operations.extend(self.compile_expr(*node.index));
        operations.push(OperationIndex::new(ByteOperation::AccessLiteral, 0));
        operations
    }

    fn access_attribute(
        &mut self,
        access_attribute_node: AccessAttributeNode,
    ) -> Vec<OperationIndex> {
        todo!()
    }

    fn assign(&mut self, assign_node: AssignNode) -> Vec<OperationIndex> {
        let mut operations = self.compile_expr(*assign_node.value);
        operations.extend(self.identity(*assign_node.identity.value));
        operations.push(OperationIndex::new(ByteOperation::Assign, 0));
        operations
    }

    pub fn compile_expr(&mut self, expr: ExprNode) -> Vec<OperationIndex> {
        match expr {
            ExprNode::Identity(identity_node) => self.identity(*identity_node.value),
            ExprNode::Assign(assign_node) => self.assign(assign_node),
            ExprNode::Int(_) |
            ExprNode::Float(_) |
            ExprNode::Bool(_) |
            ExprNode::String(_) => { 
                let literal_index = self.cache_literal(expr);
                self.load_literal(literal_index)
            },
            _ => panic!("Unexpected expr node: {:?}", expr),
        }
    }

    pub fn compile(&mut self, ast: Vec<ExprNode>) {
        todo!()
    }
}
