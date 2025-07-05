use crate::compiler::byte_operations::*;
use crate::compiler::code_object::{CodeObject, Value};
use crate::lexer::TokenKind;
use crate::parser::ExprNode;
use crate::parser::nodes::*;
use crate::parser::traits::HasId;
use std::arch::x86_64::_mm_cmpord_pd;
use std::cell::RefCell;
use std::rc::Rc;

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
        match identity_address_iter.next() {
            Some(ExprNode::String(string_base)) => self.load_name(code_object, string_base),
            Some(ExprNode::FuncCall(func_call_base)) => {
                self.function_call(code_object, func_call_base)
            }
            _ => panic!(
                "Unexpected identity base: {:?}",
                identity_address_iter.collect::<Vec<_>>()
            ),
        };
        for part in identity_address_iter {
            match part {
                ExprNode::BinarySubscribe(binary_subscribe_node) => {
                    self.binary_subscribe(code_object, binary_subscribe_node)
                }
                ExprNode::AccessAttribute(access_attribute_node) => {
                    self.access_attribute(code_object, access_attribute_node)
                }
                _ => panic!("Unexpected identity part: {:?}", part),
            }
        }
    }

    fn identity_popped_head(
        &mut self,
        code_object: &mut CodeObject,
        mut identity: IdentityNode,
    ) -> ExprNode {
        let head = identity.address.pop().unwrap();
        if !identity.address.is_empty() {
            self.identity(code_object, identity);
        }
        head
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
        code_object
            .constants
            .push(Rc::new(RefCell::new(runtime_constant)));
        new_literal_index
    }

    fn cache_variable(code_object: &mut CodeObject, name: String) -> usize {
        if let Some(var_index) = code_object.variable_index_lookup.get(&name) {
            *var_index
        } else {
            let new_variable_index = code_object.variables.len();
            code_object
                .variable_index_lookup
                .insert(name.clone(), new_variable_index);
            code_object
                .variables
                .push(Rc::new(RefCell::new(Value::Null)));
            new_variable_index
        }
    }

    fn constant(
        &mut self,
        code_object: &mut CodeObject,
        constant_id: usize,
        constant_value: Value,
    ) {
        let constant_index = Compiler::cache_constant(code_object, constant_id, constant_value);
        self.push_op(
            code_object,
            OpIndex::with_op(ByteOp::LoadConstant, constant_index),
        );
    }

    fn load_name(&mut self, code_object: &mut CodeObject, node: StringNode) {
        if let Some(var_index) = code_object.variable_index_lookup.get(&node.value) {
            self.push_op(
                code_object,
                OpIndex::with_op(ByteOp::LoadVariable, *var_index),
            );
            return;
        };
        let var_index = Compiler::cache_variable(code_object, node.value);
        self.push_op(
            code_object,
            OpIndex::with_op(ByteOp::LoadVariable, var_index),
        );
    }

    fn binary_subscribe(&mut self, code_object: &mut CodeObject, node: BinarySubscribeNode) {
        self.compile_expr(code_object, *node.value);
        self.push_op(code_object, OpIndex::without_op(ByteOp::BinarySubscribe));
    }

    fn access_attribute(
        &mut self,
        code_object: &mut CodeObject,
        access_attribute_node: AccessAttributeNode,
    ) {
        todo!()
    }

    fn assign(&mut self, code_object: &mut CodeObject, assign_node: AssignNode) {
        // this loads rest of identity to the stack vv
        let head = self.identity_popped_head(code_object, assign_node.identity);
        match head {
            ExprNode::BinarySubscribe(binary_subscribe_node) => {
                self.compile_expr(code_object, *binary_subscribe_node.value);
                self.compile_expr(code_object, *assign_node.value);
                self.push_op(code_object, OpIndex::without_op(ByteOp::AssignSubscribe));
            }
            ExprNode::AccessAttribute(access_attribute_node) => {
                let attribute_index = match *access_attribute_node.value {
                    ExprNode::String(string_node) => {
                        Compiler::cache_variable(code_object, string_node.value)
                    }
                    _ => panic!("Unexpected head of assign: {:?}", access_attribute_node),
                };
                self.compile_expr(code_object, *assign_node.value);
                self.push_op(
                    code_object,
                    OpIndex::with_op(ByteOp::AssignAttribute, attribute_index),
                );
            }
            ExprNode::String(string_node) => {
                let var_index = Compiler::cache_variable(code_object, string_node.value);
                self.compile_expr(code_object, *assign_node.value);
                self.push_op(code_object, OpIndex::with_op(ByteOp::PreAssign, var_index));
            }
            ExprNode::FuncCall(func_call_node) => {
                todo!()
            }
            _ => panic!("Unexpected head of assign: {:?}", head),
        }
    }

    fn make_runtime_object(&mut self, code_object: &mut CodeObject, object_node: ObjectNode) {
        let obj_size = object_node.properties.len() * 2;
        object_node.properties.into_iter().for_each(|property| {
            self.compile_expr(code_object, property.key);
            self.compile_expr(code_object, property.value);
        });
        self.push_op(code_object, OpIndex::with_op(ByteOp::MakeObject, obj_size));
    }

    fn make_runtime_list(&mut self, code_object: &mut CodeObject, list_node: ListNode) {
        let list_len = list_node.elements.len();
        list_node.elements.into_iter().for_each(|element| {
            self.compile_expr(code_object, element);
        });
        self.push_op(code_object, OpIndex::with_op(ByteOp::MakeList, list_len));
    }

    fn make_function(&mut self, code_object: &mut CodeObject, function_node: FunctionNode) {
        let func_id = function_node.id;
        let func_value = Value::function(function_node.arguments, self.compile(function_node.body));
        let func_const_index = Compiler::cache_constant(code_object, func_id, func_value);
        self.push_op(
            code_object,
            OpIndex::with_op(ByteOp::LoadConstant, func_const_index),
        );
    }

    fn return_value(&mut self, code_object: &mut CodeObject, return_node: ReturnNode) {
        self.compile_expr(code_object, *return_node.value);
        self.push_op(code_object, OpIndex::without_op(ByteOp::ReturnValue));
    }

    fn function_call(
        &mut self,
        code_object: &mut CodeObject,
        function_call_node: FunctionCallNode,
    ) {
        let arg_count = function_call_node.arguments.len();
        function_call_node
            .arguments
            .into_iter()
            .for_each(|arg| self.compile_expr(code_object, arg));
        self.identity(code_object, function_call_node.identity);
        self.push_op(code_object, OpIndex::with_op(ByteOp::Call, arg_count));
    }

    fn make_closure_body(&mut self, code_object: &mut CodeObject, body: Vec<ExprNode>) {
        for ast_node in body.into_iter() {
            self.compile_expr(code_object, ast_node);
        }
    }

    fn while_closure(&mut self, code_object: &mut CodeObject, while_node: WhileNode) {
        let loop_body_start_index = self.ip;
        self.compile_expr(code_object, *while_node.condition);
        let pop_jump_op_index = code_object.operations.len();
        self.push_op(code_object, OpIndex::without_op(ByteOp::PopJumpIfFalse));
        self.make_closure_body(code_object, while_node.body);
        self.push_op(
            code_object,
            OpIndex::with_op(ByteOp::Jump, loop_body_start_index),
        );
        code_object.operations[pop_jump_op_index].operand = self.ip;
    }

    fn if_closure(&mut self, code_object: &mut CodeObject, if_node: IfNode) {
        self.compile_expr(code_object, *if_node.condition);
        let pop_jump_false_op_index = code_object.operations.len();
        self.push_op(code_object, OpIndex::without_op(ByteOp::PopJumpIfFalse));
        self.make_closure_body(code_object, if_node.then_body);
        let pop_jump_true_op_index = code_object.operations.len();
        self.push_op(code_object, OpIndex::without_op(ByteOp::Jump));
        code_object.operations[pop_jump_false_op_index].operand = self.ip;
        self.make_closure_body(code_object, if_node.else_body);
        code_object.operations[pop_jump_true_op_index].operand = self.ip;
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
            ExprNode::Int(_) | ExprNode::Float(_) | ExprNode::Bool(_) | ExprNode::String(_) => {
                self.constant(code_object, expr.id(), Value::from_expr(expr).ok().unwrap())
            }
            ExprNode::Object(object) => {
                let constant_id = object.id;
                if let Ok(obj_const) = Value::try_const_from_object(object.clone()) {
                    self.constant(code_object, constant_id, obj_const);
                } else {
                    self.make_runtime_object(code_object, object);
                }
            }
            ExprNode::List(list) => {
                let constant_id = list.id;
                if let Ok(list_const) = Value::try_const_from_list(list.clone()) {
                    self.constant(code_object, constant_id, list_const);
                } else {
                    self.make_runtime_list(code_object, list);
                }
            }
            ExprNode::Function(function_node) => self.make_function(code_object, function_node),
            ExprNode::FuncCall(function_call_node) => {
                self.function_call(code_object, function_call_node)
            }
            ExprNode::Return(return_node) => self.return_value(code_object, return_node),
            ExprNode::If(if_node) => self.if_closure(code_object, if_node),
            ExprNode::While(while_node) => self.while_closure(code_object, while_node),
            ExprNode::Comparison(comparison_node) => self.comparison(code_object, comparison_node),
            ExprNode::Identity(identity_node) => self.identity(code_object, identity_node),
            ExprNode::Assign(assign_node) => self.assign(code_object, assign_node),
            ExprNode::Binary(binary_node) => self.binary(code_object, binary_node),
            ExprNode::Null(_) => self.push_op(code_object, OpIndex::without_op(ByteOp::LoadNull)),
            _ => panic!("Unexpected expr node: {:?}", expr),
        }
    }

    pub fn compile(&mut self, ast: Vec<ExprNode>) -> CodeObject {
        let mut code_object = CodeObject::default();
        for ast_node in ast.into_iter() {
            self.compile_expr(&mut code_object, ast_node);
        }
        code_object
    }
}
