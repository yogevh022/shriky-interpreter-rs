use crate::compiler::byte_operations::OpIndex;
use crate::compiler::code_object::CodeObject;
use crate::compiler::compiler::CompileContext;
use crate::compiler::op::call;
use crate::compiler::vm_static::{cache_constant, cache_variable};
use crate::compiler::{ByteOp, Compiler};
use crate::parser::ExprNode;
use crate::parser::nodes::{AccessAttributeNode, BinarySubscribeNode, IdentityNode, StringNode};
use crate::runtime::value::Value;
use std::cell::RefCell;
use std::rc::Rc;

fn binary_subscribe(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    node: BinarySubscribeNode,
) {
    compiler.compile_expr(*node.value, &CompileContext::Normal);
    compiler.push_op(
        &mut *code_object.borrow_mut(),
        OpIndex::without_op(ByteOp::BinarySubscribe),
    );
}

fn access_attribute(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    node: AccessAttributeNode,
) {
    compiler.compile_expr(*node.value, &CompileContext::Normal);
    compiler.push_op(
        &mut *code_object.borrow_mut(),
        OpIndex::without_op(ByteOp::AccessAttribute),
    );
}

pub(crate) fn identity(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    identity: IdentityNode,
    context: &CompileContext,
) {
    let mut identity_address_iter = identity.address.into_iter();
    match identity_address_iter.next() {
        Some(ExprNode::String(string_base)) => match context {
            CompileContext::Assignment => {
                load_or_cache_local(compiler, code_object.clone(), string_base)
            }
            _ => load_local_or_nonlocal(compiler, code_object.clone(), string_base),
        },
        Some(ExprNode::Call(func_call_base)) => {
            call(compiler, code_object.clone(), func_call_base, context)
        }
        _ => panic!(
            "Unexpected identity base: {:?}",
            identity_address_iter.collect::<Vec<_>>()
        ),
    };
    for part in identity_address_iter {
        match part {
            ExprNode::BinarySubscribe(binary_subscribe_node) => {
                binary_subscribe(compiler, code_object.clone(), binary_subscribe_node)
            }
            ExprNode::AccessAttribute(access_attribute_node) => {
                access_attribute(compiler, code_object.clone(), access_attribute_node)
            }
            _ => panic!("Unexpected identity part: {:?}", part),
        }
    }
}

pub(crate) fn identity_popped_head(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    mut identity_node: IdentityNode,
    context: &CompileContext,
) -> ExprNode {
    // address has > 1 items
    let head = identity_node.address.pop().unwrap();
    identity(compiler, code_object, identity_node, context);
    head
}

pub(crate) fn load_constant(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    constant_id: usize,
    constant_value: Value,
) {
    let mut_code_obj = &mut *code_object.borrow_mut();
    let constant_index = cache_constant(mut_code_obj, constant_id, constant_value);
    compiler.push_op(
        mut_code_obj,
        OpIndex::with_op(ByteOp::LoadConstant, constant_index),
    );
}

fn load_cached_local(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    node: &StringNode,
) -> Result<(), ()> {
    let var_index = *code_object
        .borrow()
        .variable_index_lookup
        .get(&node.value)
        .ok_or(())?;
    compiler.push_op(
        &mut *code_object.borrow_mut(),
        OpIndex::with_op(ByteOp::LoadLocal, var_index),
    );
    Ok(())
}

pub(crate) fn load_or_cache_local(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    node: StringNode,
) {
    load_cached_local(compiler, code_object.clone(), &node).unwrap_or_else(|_| {
        let mut_code_obj = &mut *code_object.borrow_mut();
        let var_index = cache_variable(mut_code_obj, &node.value);
        compiler.push_op(mut_code_obj, OpIndex::with_op(ByteOp::LoadLocal, var_index));
    });
}

pub(crate) fn load_local_or_nonlocal(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    node: StringNode,
) {
    if !load_cached_local(compiler, code_object.clone(), &node).is_ok() {
        let (scope, var_index) = compiler
            .scope_stack
            .iter()
            .rev()
            .find_map(|scope| {
                scope
                    .borrow()
                    .variable_index_lookup
                    .get(&node.value)
                    .map(|var_index| (scope.clone(), *var_index))
            })
            .unwrap_or_else(|| panic!("Variable {:?} not found in scope stack", node.value));
        let scope_id = scope.borrow().id;
        let mut_code_obj = &mut *code_object.borrow_mut();
        compiler.push_op(mut_code_obj, OpIndex::with_op(ByteOp::LoadScope, scope_id));
        compiler.push_op(
            mut_code_obj,
            OpIndex::with_op(ByteOp::LoadNonlocal, var_index),
        );
    }
}
