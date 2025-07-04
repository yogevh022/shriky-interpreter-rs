use crate::compiler::byte_operations::*;
use crate::compiler::code_object;
use crate::compiler::code_object::{CodeObject, Value};
use crate::lexer::TokenKind;
use crate::parser::ExprNode;
use crate::parser::nodes::*;
use crate::parser::traits::HasId;
use std::collections::HashMap;
use std::fs::OpenOptions;

pub struct Compiler {
    ip: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Self { ip: 0 }
    }

    fn push_op(&mut self, code_object: &mut CodeObject, op: OpIndex) {
        code_object.operations.push(op);
        self.ip += 1;
    }

    fn push_ops(&mut self, code_object: &mut CodeObject, ops: Vec<OpIndex>) {
        self.ip += ops.len();
        code_object.operations.extend(ops);
    }

    fn binary(&mut self, code_object: &mut CodeObject, binary_node: BinaryNode) {
        self.compile_expr(code_object, *binary_node.left);
        self.compile_expr(code_object, *binary_node.right);
        match binary_node.operator {
            TokenKind::Plus => self.push_op(code_object, OpIndex::without_op(ByteOp::Add)),
            TokenKind::Minus => self.push_op(code_object, OpIndex::without_op(ByteOp::Sub)),
            TokenKind::Asterisk => self.push_op(code_object, OpIndex::without_op(ByteOp::Mul)),
            TokenKind::Slash => self.push_op(code_object, OpIndex::without_op(ByteOp::Div)),
            TokenKind::DoubleSlash => {
                self.push_op(code_object, OpIndex::without_op(ByteOp::IntDiv))
            }
            TokenKind::Exponent => self.push_op(code_object, OpIndex::without_op(ByteOp::Exp)),
            TokenKind::Modulo => self.push_op(code_object, OpIndex::without_op(ByteOp::Mod)),
            _ => unreachable!("Expected binary operator, got: {:?}", binary_node.operator),
        }
    }

    fn identity(&mut self, code_object: &mut CodeObject, identity: IdentityNode) {
        let mut identity_address_iter = identity.address.into_iter();
        if let Some(ExprNode::String(identity_base)) = identity_address_iter.next() {
            self.load_name(code_object, identity_base);
        } else {
            panic!(
                "Unexpected identity base: {:?}",
                identity_address_iter.collect::<Vec<_>>()
            );
        }
        for part in identity_address_iter {
            match part {
                ExprNode::AccessLiteral(access_literal_node) => {
                    self.access_constant(code_object, access_literal_node)
                }
                ExprNode::AccessAttribute(access_attribute_node) => {
                    self.access_attribute(code_object, access_attribute_node)
                }
                _ => panic!("Unexpected identity part: {:?}", part),
            }
        }
    }

    fn cache_constant(
        code_object: &mut CodeObject,
        constant_id: usize,
        runtime_constant: Value,
    ) -> usize {
        let new_literal_index = code_object.constants.len();
        code_object
            .constant_index_lookup
            .insert(constant_id, new_literal_index);
        code_object.constants.push(runtime_constant);
        new_literal_index
    }

    fn cache_variable(code_object: &mut CodeObject, name: String) -> usize {
        let new_variable_index = code_object.variables.len();
        code_object
            .variable_index_lookup
            .insert(name.clone(), new_variable_index);
        code_object.variables.push(name);
        new_variable_index
    }

    fn load_name(&mut self, code_object: &mut CodeObject, node: StringNode) {
        if let Some(var_index) = code_object.variable_index_lookup.get(&node.value) {
            self.push_op(code_object, OpIndex::with_op(ByteOp::LoadName, *var_index));
            return;
        };
        let var_index = Compiler::cache_variable(code_object, node.value);
        self.push_op(code_object, OpIndex::with_op(ByteOp::LoadName, var_index));
    }

    fn access_constant(&mut self, code_object: &mut CodeObject, node: AccessConstantNode) {
        let literal_index =
            Compiler::cache_constant(code_object, node.value.id(), Value::from_expr(*node.value));
        self.push_ops(
            code_object,
            vec![
                OpIndex::with_op(ByteOp::LoadConstant, literal_index),
                OpIndex::without_op(ByteOp::BinarySubscribe),
            ],
        );
    }

    fn access_attribute(
        &mut self,
        code_object: &mut CodeObject,
        access_attribute_node: AccessAttributeNode,
    ) {
        todo!()
    }

    fn assign(&mut self, code_object: &mut CodeObject, assign_node: AssignNode) {
        self.identity(code_object, assign_node.identity);
        self.compile_expr(code_object, *assign_node.value);
        self.push_op(
            code_object,
            OpIndex::with_op(ByteOp::Assign, assign_node.return_after as usize),
        );
    }

    fn make_function(&mut self, code_object: &mut CodeObject, function_node: FunctionNode) {
        let func_id = function_node.id;
        let func_obj = self.compile(function_node.body);
        let func_const_index =
            Compiler::cache_constant(code_object, func_id, Value::Function(func_obj));
        self.push_op(
            code_object,
            OpIndex::with_op(ByteOp::LoadConstant, func_const_index),
        );
    }

    fn return_value(&mut self, code_object: &mut CodeObject, return_node: ReturnNode) {
        self.compile_expr(code_object, *return_node.value);
        self.push_op(code_object, OpIndex::without_op(ByteOp::ReturnValue));
    }

    fn make_loop(&mut self, code_object: &mut CodeObject, body: Vec<ExprNode>) {
        for ast_node in body.into_iter() {
            self.compile_expr(code_object, ast_node);
        }
    }

    fn while_loop(&mut self, code_object: &mut CodeObject, while_node: WhileNode) {
        self.push_op(code_object, OpIndex::without_op(ByteOp::StartLoop));
        let loop_start_index = code_object.operations.len();
        self.compile_expr(code_object, *while_node.condition);
        self.push_op(code_object, OpIndex::without_op(ByteOp::PopJumpIfFalse));
        let pop_jump_op_index = code_object.operations.len() - 1;
        self.make_loop(code_object, while_node.body);
        self.push_op(
            code_object,
            OpIndex::with_op(ByteOp::Jump, loop_start_index),
        );
        code_object.operations[pop_jump_op_index].operand = self.ip;
    }

    fn comparison(&mut self, code_object: &mut CodeObject, comparison_node: ComparisonNode) {
        let operand = match comparison_node.operator {
            TokenKind::Equals => ByteComparisonOp::Equal,
            TokenKind::NotEquals => ByteComparisonOp::NotEqual,
            TokenKind::LessThan => ByteComparisonOp::Less,
            TokenKind::LessThanEquals => ByteComparisonOp::LessEqual,
            TokenKind::GreaterThan => ByteComparisonOp::Greater,
            TokenKind::GreaterThanEquals => ByteComparisonOp::GreaterEqual,
            _ => unreachable!(
                "Unexpected comparison operator: {:?}",
                comparison_node.operator
            ),
        };
        self.compile_expr(code_object, *comparison_node.left);
        self.compile_expr(code_object, *comparison_node.right);
        self.push_op(
            code_object,
            OpIndex::with_op(ByteOp::Compare, operand as usize),
        );
    }

    pub fn compile_expr(&mut self, code_object: &mut CodeObject, expr: ExprNode) {
        match expr {
            ExprNode::Int(_)
            | ExprNode::Float(_)
            | ExprNode::Bool(_)
            | ExprNode::String(_)
            | ExprNode::Object(_)
            | ExprNode::List(_) => {
                let constant_index =
                    Compiler::cache_constant(code_object, expr.id(), Value::from_expr(expr));
                self.push_op(
                    code_object,
                    OpIndex::with_op(ByteOp::LoadConstant, constant_index),
                );
            }
            ExprNode::Function(function_node) => self.make_function(code_object, function_node),
            ExprNode::Return(return_node) => self.return_value(code_object, return_node),
            ExprNode::While(while_node) => self.while_loop(code_object, while_node),
            ExprNode::Comparison(comparison_node) => self.comparison(code_object, comparison_node),
            ExprNode::Identity(identity_node) => self.identity(code_object, identity_node),
            ExprNode::Assign(assign_node) => self.assign(code_object, assign_node),
            ExprNode::Binary(binary_node) => self.binary(code_object, binary_node),
            ExprNode::Null(_) => self.push_op(code_object, OpIndex::without_op(ByteOp::LoadNull)),
            _ => panic!("Unexpected expr node: {:?}", expr),
        }
    }

    pub fn compile(&mut self, ast: Vec<ExprNode>) -> CodeObject {
        let mut code_object = CodeObject::from_index(self.ip);
        for ast_node in ast.into_iter() {
            self.compile_expr(&mut code_object, ast_node);
        }
        code_object
    }
}
