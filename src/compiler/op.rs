use crate::compiler::byte_operations::{ByteComparisonOp, OpIndex};
use crate::compiler::code_object::CodeObject;
use crate::compiler::compiler::CompileContext;
use crate::compiler::load::{identity, identity_popped_head};
use crate::compiler::vm_static::cache_variable;
use crate::compiler::{ByteOp, Compiler};
use crate::lexer::TokenKind;
use crate::parser::ExprNode;
use crate::parser::nodes::{
    AssignNode, BinaryNode, CallNode, ComparisonNode, LogicalNode, ReturnNode,
};
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn binary(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    binary_node: BinaryNode,
    context: &CompileContext,
) {
    compiler.compile_expr(*binary_node.left, context);
    compiler.compile_expr(*binary_node.right, context);
    let mut_code_obj = &mut *code_object.borrow_mut();
    match binary_node.operator {
        TokenKind::Plus | TokenKind::Increment => {
            compiler.push_op(mut_code_obj, OpIndex::without_op(ByteOp::Add))
        }
        TokenKind::Minus | TokenKind::Decrement => {
            compiler.push_op(mut_code_obj, OpIndex::without_op(ByteOp::Sub))
        }
        TokenKind::Asterisk => compiler.push_op(mut_code_obj, OpIndex::without_op(ByteOp::Mul)),
        TokenKind::Slash => compiler.push_op(mut_code_obj, OpIndex::without_op(ByteOp::Div)),
        TokenKind::DoubleSlash => {
            compiler.push_op(mut_code_obj, OpIndex::without_op(ByteOp::IntDiv))
        }
        TokenKind::Exponent => compiler.push_op(mut_code_obj, OpIndex::without_op(ByteOp::Exp)),
        TokenKind::Modulo => compiler.push_op(mut_code_obj, OpIndex::without_op(ByteOp::Mod)),
        _ => unreachable!("Expected binary operator, got: {:?}", binary_node.operator),
    }
}

pub(crate) fn assign(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    mut assign_node: AssignNode,
    context: &CompileContext,
) {
    let mut ctx = context;
    let head = if assign_node.identity.address.len() > 1 {
        // this also loads the rest of identity to the stack vv
        identity_popped_head(
            compiler,
            code_object.clone(),
            assign_node.identity,
            &CompileContext::Assignment,
        )
    } else {
        // assigning to a root address (head is root)
        // now we check whether/where this needs to be cached
        ctx = if matches!(context, CompileContext::Class) {
            // class context takes priority
            &context
        } else {
            // otherwise set the context to assignment (for caching/scoping logic)
            &CompileContext::Assignment
        };
        assign_node.identity.address.pop().unwrap()
    };
    match head {
        ExprNode::BinarySubscribe(binary_subscribe_node) => {
            compiler.compile_expr(*binary_subscribe_node.value, ctx);
            compiler.compile_expr(*assign_node.value, ctx);
            compiler.push_op(
                &mut *code_object.borrow_mut(),
                OpIndex::without_op(ByteOp::AssignSubscribe),
            );
        }
        ExprNode::AccessAttribute(access_attribute_node) => {
            compiler.compile_expr(*access_attribute_node.value, ctx);
            compiler.compile_expr(*assign_node.value, ctx);
            compiler.push_op(
                &mut *code_object.borrow_mut(),
                OpIndex::without_op(ByteOp::AssignAttribute),
            );
        }
        ExprNode::String(string_node) => {
            let var_index = cache_variable(&mut *code_object.borrow_mut(), &string_node.value);
            compiler.compile_expr(*assign_node.value, ctx);
            compiler.push_op(
                &mut *code_object.borrow_mut(),
                OpIndex::with_op(ByteOp::PreAssign, var_index),
            );
        }
        ExprNode::Call(func_call_node) => {
            todo!()
        }
        _ => panic!("Unexpected head of assign: {:?}", head),
    }
}

pub(crate) fn return_value(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    return_node: ReturnNode,
    context: &CompileContext,
) {
    compiler.compile_expr(*return_node.value, context);
    compiler.push_op(
        &mut *code_object.borrow_mut(),
        OpIndex::without_op(ByteOp::ReturnValue),
    );
}

pub(crate) fn call(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    call_node: CallNode,
    context: &CompileContext,
) {
    let arg_count = call_node.arguments.len();
    call_node
        .arguments
        .into_iter()
        .for_each(|arg| compiler.compile_expr(arg, &CompileContext::Normal));
    identity(compiler, code_object.clone(), call_node.identity, context);
    compiler.push_op(
        &mut *code_object.borrow_mut(),
        OpIndex::with_op(ByteOp::Call, arg_count),
    );
}

pub(crate) fn comparison(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    comparison_node: ComparisonNode,
) {
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
    compiler.compile_expr(*comparison_node.left, &CompileContext::Normal);
    compiler.compile_expr(*comparison_node.right, &CompileContext::Normal);
    compiler.push_op(
        &mut *code_object.borrow_mut(),
        OpIndex::with_op(ByteOp::Compare, operand as usize),
    );
}

pub(crate) fn logical(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    logical_node: LogicalNode,
) {
    compiler.compile_expr(*logical_node.left, &CompileContext::Normal);
    compiler.compile_expr(*logical_node.right, &CompileContext::Normal);
    let op = match logical_node.operator {
        TokenKind::LogicalAND => ByteOp::LogicalAnd,
        TokenKind::LogicalOR => ByteOp::LogicalOr,
        _ => unreachable!("Unexpected logical operator: {:?}", logical_node.operator),
    };
    compiler.push_op(&mut *code_object.borrow_mut(), OpIndex::without_op(op));
}
