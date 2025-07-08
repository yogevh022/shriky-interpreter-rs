use crate::compiler::byte_operations::{ByteComparisonOp, OpIndex};
use crate::compiler::{ByteOp, Compiler};
use crate::compiler::code_object::CodeObject;
use crate::compiler::compiler::CompileContext;
use crate::compiler::load::{identity, identity_popped_head};
use crate::compiler::vm_static::cache_variable;
use crate::lexer::TokenKind;
use crate::parser::ExprNode;
use crate::parser::nodes::{AssignNode, BinaryNode, CallNode, ComparisonNode, LogicalNode, ReturnNode};

pub(crate) fn binary(
    compiler: &mut Compiler,
    code_object: &mut CodeObject,
    binary_node: BinaryNode,
    context: &CompileContext,
) {
    compiler.compile_expr(code_object, *binary_node.left, context);
    compiler.compile_expr(code_object, *binary_node.right, context);
    match binary_node.operator {
        TokenKind::Plus | TokenKind::Increment => {
            compiler.push_op(code_object, OpIndex::without_op(ByteOp::Add))
        }
        TokenKind::Minus | TokenKind::Decrement => {
            compiler.push_op(code_object, OpIndex::without_op(ByteOp::Sub))
        }
        TokenKind::Asterisk => compiler.push_op(code_object, OpIndex::without_op(ByteOp::Mul)),
        TokenKind::Slash => compiler.push_op(code_object, OpIndex::without_op(ByteOp::Div)),
        TokenKind::DoubleSlash => {
            compiler.push_op(code_object, OpIndex::without_op(ByteOp::IntDiv))
        }
        TokenKind::Exponent => compiler.push_op(code_object, OpIndex::without_op(ByteOp::Exp)),
        TokenKind::Modulo => compiler.push_op(code_object, OpIndex::without_op(ByteOp::Mod)),
        _ => unreachable!("Expected binary operator, got: {:?}", binary_node.operator),
    }
}

pub(crate) fn assign(
    compiler: &mut Compiler,
    code_object: &mut CodeObject,
    assign_node: AssignNode,
    context: &CompileContext,
) {
    // this loads rest of identity to the stack vv
    let head = identity_popped_head(compiler, code_object, assign_node.identity, context);
    match head {
        ExprNode::BinarySubscribe(binary_subscribe_node) => {
            compiler.compile_expr(code_object, *binary_subscribe_node.value, context);
            compiler.compile_expr(code_object, *assign_node.value, context);
            compiler.push_op(code_object, OpIndex::without_op(ByteOp::AssignSubscribe));
        }
        ExprNode::AccessAttribute(access_attribute_node) => {
            compiler.compile_expr(code_object, *access_attribute_node.value, context);
            compiler.compile_expr(code_object, *assign_node.value, context);
            compiler.push_op(code_object, OpIndex::without_op(ByteOp::AssignAttribute));
        }
        ExprNode::String(string_node) => {
            let var_index = cache_variable(code_object, &string_node.value);
            compiler.compile_expr(code_object, *assign_node.value, context);
            compiler.push_op(code_object, OpIndex::with_op(ByteOp::PreAssign, var_index));
        }
        ExprNode::Call(func_call_node) => {
            todo!()
        }
        _ => panic!("Unexpected head of assign: {:?}", head),
    }
}

pub(crate) fn return_value(
    compiler: &mut Compiler,
    code_object: &mut CodeObject,
    return_node: ReturnNode,
    context: &CompileContext,
) {
    compiler.compile_expr(code_object, *return_node.value, context);
    compiler.push_op(code_object, OpIndex::without_op(ByteOp::ReturnValue));
}

pub(crate) fn call(
    compiler: &mut Compiler,
    code_object: &mut CodeObject,
    call_node: CallNode,
    context: &CompileContext,
) {
    let arg_count = call_node.arguments.len();
    call_node
        .arguments
        .into_iter()
        .for_each(|arg| compiler.compile_expr(code_object, arg, &CompileContext::Normal));
    identity(compiler, code_object, call_node.identity, context);
    compiler.push_op(code_object, OpIndex::with_op(ByteOp::Call, arg_count));
}



pub(crate) fn comparison(compiler: &mut Compiler, code_object: &mut CodeObject, comparison_node: ComparisonNode) {
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
    compiler.compile_expr(code_object, *comparison_node.left, &CompileContext::Normal);
    compiler.compile_expr(code_object, *comparison_node.right, &CompileContext::Normal);
    compiler.push_op(
        code_object,
        OpIndex::with_op(ByteOp::Compare, operand as usize),
    );
}

pub(crate) fn logical(compiler: &mut Compiler, code_object: &mut CodeObject, logical_node: LogicalNode) {
    compiler.compile_expr(code_object, *logical_node.left, &CompileContext::Normal);
    compiler.compile_expr(code_object, *logical_node.right, &CompileContext::Normal);
    let op = match logical_node.operator {
        TokenKind::LogicalAND => ByteOp::LogicalAnd,
        TokenKind::LogicalOR => ByteOp::LogicalOr,
        _ => unreachable!("Unexpected logical operator: {:?}", logical_node.operator),
    };
    compiler.push_op(code_object, OpIndex::without_op(op));
}