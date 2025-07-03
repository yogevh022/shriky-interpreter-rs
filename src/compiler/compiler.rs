use crate::compiler::byte_operations::*;
use crate::compiler::code_object::{CodeObject, Value};
use crate::lexer::TokenKind;
use crate::parser::ExprNode;
use crate::parser::nodes::*;
use crate::parser::traits::HasId;
use std::collections::HashMap;

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }

    fn binary(code_object: &mut CodeObject, binary_node: BinaryNode) {
        Compiler::compile_expr(code_object, *binary_node.left);
        Compiler::compile_expr(code_object, *binary_node.right);
        match binary_node.operator {
            TokenKind::Plus => code_object
                .operations
                .push(OperationIndex::without_operand(ByteOperation::Add)),
            TokenKind::Minus => code_object
                .operations
                .push(OperationIndex::without_operand(ByteOperation::Sub)),
            TokenKind::Asterisk => code_object
                .operations
                .push(OperationIndex::without_operand(ByteOperation::Mul)),
            TokenKind::Slash => code_object
                .operations
                .push(OperationIndex::without_operand(ByteOperation::Div)),
            TokenKind::DoubleSlash => code_object
                .operations
                .push(OperationIndex::without_operand(ByteOperation::IntDiv)),
            TokenKind::Exponent => code_object
                .operations
                .push(OperationIndex::without_operand(ByteOperation::Exp)),
            TokenKind::Modulo => code_object
                .operations
                .push(OperationIndex::without_operand(ByteOperation::Mod)),
            _ => unreachable!("Expected binary operator, got: {:?}", binary_node.operator),
        }
    }

    fn identity(code_object: &mut CodeObject, identity: IdentityNode) {
        let mut identity_address_iter = identity.address.into_iter();
        if let Some(ExprNode::String(identity_base)) = identity_address_iter.next() {
            Compiler::load_name(code_object, identity_base);
        } else {
            panic!(
                "Unexpected identity base: {:?}",
                identity_address_iter.collect::<Vec<_>>()
            );
        }
        for part in identity_address_iter {
            match part {
                ExprNode::AccessLiteral(access_literal_node) => {
                    Compiler::access_constant(code_object, access_literal_node)
                }
                ExprNode::AccessAttribute(access_attribute_node) => {
                    Compiler::access_attribute(code_object, access_attribute_node)
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

    fn load_name(code_object: &mut CodeObject, node: StringNode) {
        if let Some(var_index) = code_object.variable_index_lookup.get(&node.value) {
            code_object.operations.push(OperationIndex::with_operand(
                ByteOperation::LoadName,
                *var_index,
            ));
            return;
        };
        let var_index = Compiler::cache_variable(code_object, node.value);
        code_object.operations.push(OperationIndex::with_operand(
            ByteOperation::LoadName,
            var_index,
        ))
    }

    fn access_constant(code_object: &mut CodeObject, node: AccessConstantNode) {
        let literal_index =
            Compiler::cache_constant(code_object, node.value.id(), Value::from_expr(*node.value));
        code_object.operations.extend([
            OperationIndex::with_operand(ByteOperation::LoadConstant, literal_index),
            OperationIndex::without_operand(ByteOperation::BinarySubscribe),
        ]);
    }

    fn access_attribute(code_object: &mut CodeObject, access_attribute_node: AccessAttributeNode) {
        todo!()
    }

    fn assign(code_object: &mut CodeObject, assign_node: AssignNode) {
        Compiler::identity(code_object, assign_node.identity);
        Compiler::compile_expr(code_object, *assign_node.value);
        code_object.operations.push(OperationIndex::with_operand(
            ByteOperation::Assign,
            assign_node.return_after as usize,
        ));
    }

    fn make_function(code_object: &mut CodeObject, function_node: FunctionNode) {
        let func_id = function_node.id;
        let func_obj = Compiler::compile(function_node.body);
        let func_const_index =
            Compiler::cache_constant(code_object, func_id, Value::Function(func_obj));
        code_object.operations.push(OperationIndex::with_operand(
            ByteOperation::LoadConstant,
            func_const_index,
        ));
    }

    fn return_value(code_object: &mut CodeObject, return_node: ReturnNode) {
        Compiler::compile_expr(code_object, *return_node.value);
        code_object
            .operations
            .push(OperationIndex::without_operand(ByteOperation::ReturnValue))
    }

    fn while_loop(code_object: &mut CodeObject, while_node: WhileNode) {
        todo!()
        // let loop_condition = Compiler::compile_expr(code_object, *while_node.condition);
        // let loop_body = Compiler::compile(while_node.body);
    }

    pub fn compile_expr(code_object: &mut CodeObject, expr: ExprNode) {
        match expr {
            ExprNode::Int(_)
            | ExprNode::Float(_)
            | ExprNode::Bool(_)
            | ExprNode::String(_)
            | ExprNode::Object(_)
            | ExprNode::List(_) => {
                let constant_index =
                    Compiler::cache_constant(code_object, expr.id(), Value::from_expr(expr));
                code_object.operations.push(OperationIndex::with_operand(
                    ByteOperation::LoadConstant,
                    constant_index,
                ));
            }
            ExprNode::Function(function_node) => {
                Compiler::make_function(code_object, function_node)
            }
            ExprNode::Return(return_node) => Compiler::return_value(code_object, return_node),
            ExprNode::Identity(identity_node) => Compiler::identity(code_object, identity_node),
            ExprNode::Assign(assign_node) => Compiler::assign(code_object, assign_node),
            ExprNode::Binary(binary_node) => Compiler::binary(code_object, binary_node),
            _ => panic!("Unexpected expr node: {:?}", expr),
        }
    }

    pub fn compile(ast: Vec<ExprNode>) -> CodeObject {
        let mut code_object = CodeObject::default();
        for ast_node in ast.iter() {
            Compiler::compile_expr(&mut code_object, ast_node.clone());
        }
        code_object
    }
}
