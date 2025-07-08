use crate::compiler::byte_operations::OpIndex;
use crate::compiler::code_object::CodeObject;
use crate::compiler::compiler::CompileContext;
use crate::compiler::op::call;
use crate::compiler::vm_static::{cache_constant, cache_variable};
use crate::compiler::{ByteOp, Compiler};
use crate::parser::ExprNode;
use crate::parser::nodes::{AccessAttributeNode, BinarySubscribeNode, IdentityNode, StringNode};
use crate::runtime::values::Value;

fn binary_subscribe(
    compiler: &mut Compiler,
    code_object: &mut CodeObject,
    node: BinarySubscribeNode,
) {
    compiler.compile_expr(code_object, *node.value, &CompileContext::Normal);
    compiler.push_op(code_object, OpIndex::without_op(ByteOp::BinarySubscribe));
}

fn access_attribute(
    compiler: &mut Compiler,
    code_object: &mut CodeObject,
    node: AccessAttributeNode,
) {
    compiler.compile_expr(code_object, *node.value, &CompileContext::Normal);
    compiler.push_op(code_object, OpIndex::without_op(ByteOp::AccessAttribute));
}

pub(crate) fn identity(
    compiler: &mut Compiler,
    code_object: &mut CodeObject,
    identity: IdentityNode,
    context: &CompileContext,
) {
    let mut identity_address_iter = identity.address.into_iter();
    match identity_address_iter.next() {
        Some(ExprNode::String(string_base)) => load_name(compiler, code_object, string_base),
        Some(ExprNode::Call(func_call_base)) => {
            call(compiler, code_object, func_call_base, context)
        }
        _ => panic!(
            "Unexpected identity base: {:?}",
            identity_address_iter.collect::<Vec<_>>()
        ),
    };
    for part in identity_address_iter {
        match part {
            ExprNode::BinarySubscribe(binary_subscribe_node) => {
                binary_subscribe(compiler, code_object, binary_subscribe_node)
            }
            ExprNode::AccessAttribute(access_attribute_node) => {
                access_attribute(compiler, code_object, access_attribute_node)
            }
            _ => panic!("Unexpected identity part: {:?}", part),
        }
    }
}

pub(crate) fn identity_popped_head(
    compiler: &mut Compiler,
    code_object: &mut CodeObject,
    mut identity_node: IdentityNode,
    context: &CompileContext,
) -> ExprNode {
    let head = identity_node.address.pop().unwrap();
    if !identity_node.address.is_empty() {
        identity(compiler, code_object, identity_node, context);
    }
    head
}

pub(crate) fn load_constant(
    compiler: &mut Compiler,
    code_object: &mut CodeObject,
    constant_id: usize,
    constant_value: Value,
) {
    let constant_index = cache_constant(code_object, constant_id, constant_value);
    compiler.push_op(
        code_object,
        OpIndex::with_op(ByteOp::LoadConstant, constant_index),
    );
}

pub(crate) fn load_name(compiler: &mut Compiler, code_object: &mut CodeObject, node: StringNode) {
    if let Some(var_index) = code_object.variable_index_lookup.get(&node.value) {
        compiler.push_op(
            code_object,
            OpIndex::with_op(ByteOp::LoadVariable, *var_index),
        );
        return;
    };
    let var_index = cache_variable(code_object, &node.value);
    compiler.push_op(
        code_object,
        OpIndex::with_op(ByteOp::LoadVariable, var_index),
    );
}
