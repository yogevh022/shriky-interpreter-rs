use crate::compiler::byte_operations::*;
use crate::compiler::code_object::CodeObject;
use crate::lexer::TokenKind;
use crate::parser::ExprNode;
use crate::parser::nodes::*;
use crate::parser::traits::HasId;
use crate::runtime::values::{FunctionValue, Value};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum CompileContext {
    Function,
    Class,
    Normal,
}

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

    fn binary(
        &mut self,
        code_object: &mut CodeObject,
        binary_node: BinaryNode,
        context: &CompileContext,
    ) {
        self.compile_expr(code_object, *binary_node.left, context);
        self.compile_expr(code_object, *binary_node.right, context);
        match binary_node.operator {
            TokenKind::Plus | TokenKind::Increment => {
                self.push_op(code_object, OpIndex::without_op(ByteOp::Add))
            }
            TokenKind::Minus | TokenKind::Decrement => {
                self.push_op(code_object, OpIndex::without_op(ByteOp::Sub))
            }
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

    fn identity(
        &mut self,
        code_object: &mut CodeObject,
        identity: IdentityNode,
        context: &CompileContext,
    ) {
        let mut identity_address_iter = identity.address.into_iter();
        match identity_address_iter.next() {
            Some(ExprNode::String(string_base)) => self.load_name(code_object, string_base),
            Some(ExprNode::Call(func_call_base)) => self.call(code_object, func_call_base, context),
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
        context: &CompileContext,
    ) -> ExprNode {
        let head = identity.address.pop().unwrap();
        if !identity.address.is_empty() {
            self.identity(code_object, identity, context);
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

    fn cache_variable(code_object: &mut CodeObject, name: &String) -> usize {
        if let Some(var_index) = code_object.variable_index_lookup.get(name) {
            *var_index
        } else {
            let new_variable_index = code_object.variables.len();
            code_object
                .variable_index_lookup
                .insert(name.clone(), new_variable_index);
            code_object.variables.push(name.clone());
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
        let var_index = Compiler::cache_variable(code_object, &node.value);
        self.push_op(
            code_object,
            OpIndex::with_op(ByteOp::LoadVariable, var_index),
        );
    }

    fn binary_subscribe(&mut self, code_object: &mut CodeObject, node: BinarySubscribeNode) {
        self.compile_expr(code_object, *node.value, &CompileContext::Normal);
        self.push_op(code_object, OpIndex::without_op(ByteOp::BinarySubscribe));
    }

    fn access_attribute(&mut self, code_object: &mut CodeObject, node: AccessAttributeNode) {
        self.compile_expr(code_object, *node.value, &CompileContext::Normal);
        self.push_op(code_object, OpIndex::without_op(ByteOp::AccessAttribute));
    }

    fn assign(
        &mut self,
        code_object: &mut CodeObject,
        assign_node: AssignNode,
        context: &CompileContext,
    ) {
        // this loads rest of identity to the stack vv
        let head = self.identity_popped_head(code_object, assign_node.identity, context);
        match head {
            ExprNode::BinarySubscribe(binary_subscribe_node) => {
                self.compile_expr(code_object, *binary_subscribe_node.value, context);
                self.compile_expr(code_object, *assign_node.value, context);
                self.push_op(code_object, OpIndex::without_op(ByteOp::AssignSubscribe));
            }
            ExprNode::AccessAttribute(access_attribute_node) => {
                self.compile_expr(code_object, *access_attribute_node.value, context);
                self.compile_expr(code_object, *assign_node.value, context);
                self.push_op(code_object, OpIndex::without_op(ByteOp::AssignAttribute));
            }
            ExprNode::String(string_node) => {
                let var_index = Compiler::cache_variable(code_object, &string_node.value);
                self.compile_expr(code_object, *assign_node.value, context);
                self.push_op(code_object, OpIndex::with_op(ByteOp::PreAssign, var_index));
            }
            ExprNode::Call(func_call_node) => {
                todo!()
            }
            _ => panic!("Unexpected head of assign: {:?}", head),
        }
    }

    fn make_runtime_map(&mut self, code_object: &mut CodeObject, object_node: MapNode) {
        let obj_size = object_node.properties.len() * 2;
        object_node.properties.into_iter().for_each(|property| {
            self.compile_expr(code_object, property.key, &CompileContext::Normal);
            self.compile_expr(code_object, property.value, &CompileContext::Normal);
        });
        self.push_op(code_object, OpIndex::with_op(ByteOp::MakeMap, obj_size));
    }

    fn make_runtime_list(&mut self, code_object: &mut CodeObject, list_node: ListNode) {
        let list_len = list_node.elements.len();
        list_node.elements.into_iter().for_each(|element| {
            self.compile_expr(code_object, element, &CompileContext::Normal);
        });
        self.push_op(code_object, OpIndex::with_op(ByteOp::MakeList, list_len));
    }

    fn get_function(&mut self, function_node: FunctionNode) -> FunctionValue {
        let mut func_code_obj = self.compile(function_node.body, &CompileContext::Function);
        function_node.arguments.iter().for_each(|arg| {
            Compiler::cache_variable(&mut func_code_obj, arg); // cache params
        });
        FunctionValue::new(function_node.arguments, func_code_obj)
    }

    fn make_function(&mut self, code_object: &mut CodeObject, function_node: FunctionNode) {
        let func_id = function_node.id;
        let func_value = self.get_function(function_node);
        let func_const_index =
            Compiler::cache_constant(code_object, func_id, Value::Function(func_value));
        self.push_op(
            code_object,
            OpIndex::with_op(ByteOp::LoadConstant, func_const_index),
        );
    }

    fn make_method(&mut self, code_object: &mut CodeObject, function_node: FunctionNode) {
        let func_value = self.get_function(function_node);
        let method_id = func_value.id;
        let method_value = Value::method(func_value, None); // caller known only at runtime
        let method_const_index = Compiler::cache_constant(code_object, method_id, method_value);
        self.push_op(
            code_object,
            OpIndex::with_op(ByteOp::LoadConstant, method_const_index),
        )
    }

    fn make_class(
        &mut self,
        code_object: &mut CodeObject,
        class_node: ClassNode,
        context: &CompileContext,
    ) {
        let class_id = class_node.id;
        let class_inheritance_operand = if let Some(boxed_superclass) = class_node.superclass {
            if let ExprNode::Identity(superclass) = *boxed_superclass {
                self.identity(code_object, superclass, context);
                1usize
            } else {
                panic!("Unexpected superclass: {:?}", *boxed_superclass);
            }
        } else {
            0usize
        };
        let class_value = Value::class(None, self.compile(class_node.body, &CompileContext::Class)); // inherits at runtime
        let class_const_index = Compiler::cache_constant(code_object, class_id, class_value);
        self.push_op(
            code_object,
            OpIndex::with_op(ByteOp::LoadConstant, class_const_index),
        );
        self.push_op(
            code_object,
            OpIndex::with_op(ByteOp::MakeClass, class_inheritance_operand),
        );
    }

    fn return_value(
        &mut self,
        code_object: &mut CodeObject,
        return_node: ReturnNode,
        context: &CompileContext,
    ) {
        self.compile_expr(code_object, *return_node.value, context);
        self.push_op(code_object, OpIndex::without_op(ByteOp::ReturnValue));
    }

    fn call(
        &mut self,
        code_object: &mut CodeObject,
        call_node: CallNode,
        context: &CompileContext,
    ) {
        let arg_count = call_node.arguments.len();
        call_node
            .arguments
            .into_iter()
            .for_each(|arg| self.compile_expr(code_object, arg, &CompileContext::Normal));
        self.identity(code_object, call_node.identity, context);
        self.push_op(code_object, OpIndex::with_op(ByteOp::Call, arg_count));
    }

    fn make_closure_body(&mut self, code_object: &mut CodeObject, body: Vec<ExprNode>) {
        for ast_node in body.into_iter() {
            self.compile_expr(code_object, ast_node, &CompileContext::Normal);
        }
    }

    fn while_closure(&mut self, code_object: &mut CodeObject, while_node: WhileNode) {
        let loop_body_start_index = self.ip;
        self.compile_expr(code_object, *while_node.condition, &CompileContext::Normal);
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
        self.compile_expr(code_object, *if_node.condition, &CompileContext::Normal);
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
        self.compile_expr(code_object, *comparison_node.left, &CompileContext::Normal);
        self.compile_expr(code_object, *comparison_node.right, &CompileContext::Normal);
        self.push_op(
            code_object,
            OpIndex::with_op(ByteOp::Compare, operand as usize),
        );
    }

    fn logical(&mut self, code_object: &mut CodeObject, logical_node: LogicalNode) {
        self.compile_expr(code_object, *logical_node.left, &CompileContext::Normal);
        self.compile_expr(code_object, *logical_node.right, &CompileContext::Normal);
        let op = match logical_node.operator {
            TokenKind::LogicalAND => ByteOp::LogicalAnd,
            TokenKind::LogicalOR => ByteOp::LogicalOr,
            _ => unreachable!("Unexpected logical operator: {:?}", logical_node.operator),
        };
        self.push_op(code_object, OpIndex::without_op(op));
    }

    pub fn compile_expr(
        &mut self,
        code_object: &mut CodeObject,
        expr: ExprNode,
        context: &CompileContext,
    ) {
        match expr {
            ExprNode::Int(_) | ExprNode::Float(_) | ExprNode::Bool(_) | ExprNode::String(_) => {
                self.constant(code_object, expr.id(), Value::from_expr(expr).ok().unwrap())
            }
            ExprNode::Map(map) => {
                let constant_id = map.id;
                if let Ok(map_const) = Value::try_const_from_map(map.clone()) {
                    self.constant(code_object, constant_id, map_const);
                } else {
                    self.make_runtime_map(code_object, map);
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
            ExprNode::Function(function_node) => match context {
                CompileContext::Class => self.make_method(code_object, function_node),
                _ => self.make_function(code_object, function_node),
            },
            ExprNode::Call(call_node) => self.call(code_object, call_node, context),
            ExprNode::Class(class_node) => self.make_class(code_object, class_node, context),
            ExprNode::Return(return_node) => self.return_value(code_object, return_node, context),
            ExprNode::If(if_node) => self.if_closure(code_object, if_node),
            ExprNode::While(while_node) => self.while_closure(code_object, while_node),
            ExprNode::Comparison(comparison_node) => self.comparison(code_object, comparison_node),
            ExprNode::Logical(logical_node) => self.logical(code_object, logical_node),
            ExprNode::Identity(identity_node) => self.identity(code_object, identity_node, context),
            ExprNode::Assign(assign_node) => self.assign(code_object, assign_node, context),
            ExprNode::Binary(binary_node) => self.binary(code_object, binary_node, context),
            ExprNode::Null => self.push_op(code_object, OpIndex::without_op(ByteOp::LoadNull)),
            _ => panic!("Unexpected expr node: {:?}", expr),
        }
    }

    pub fn compile(&mut self, ast: Vec<ExprNode>, context: &CompileContext) -> CodeObject {
        let mut code_object = CodeObject::default();
        for ast_node in ast.into_iter() {
            self.compile_expr(&mut code_object, ast_node, context);
        }
        code_object
    }
}
