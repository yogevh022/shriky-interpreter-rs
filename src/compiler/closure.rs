use crate::compiler::byte_operations::OpIndex;
use crate::compiler::code_object::CodeObject;
use crate::compiler::compiler::CompileContext;
use crate::compiler::{ByteOp, Compiler};
use crate::parser::ExprNode;
use crate::parser::nodes::{IfNode, WhileNode};

fn make_closure_body(compiler: &mut Compiler, code_object: &mut CodeObject, body: Vec<ExprNode>) {
    for ast_node in body.into_iter() {
        compiler.compile_expr(code_object, ast_node, &CompileContext::Normal);
    }
}

pub(crate) fn while_closure(
    compiler: &mut Compiler,
    code_object: &mut CodeObject,
    while_node: WhileNode,
) {
    let loop_body_start_index = compiler.ip;
    compiler.compile_expr(code_object, *while_node.condition, &CompileContext::Normal);
    let pop_jump_op_index = code_object.operations.len();
    compiler.push_op(code_object, OpIndex::without_op(ByteOp::PopJumpIfFalse));
    make_closure_body(compiler, code_object, while_node.body);
    compiler.push_op(
        code_object,
        OpIndex::with_op(ByteOp::Jump, loop_body_start_index),
    );
    code_object.operations[pop_jump_op_index].operand = compiler.ip;
}

pub(crate) fn if_closure(compiler: &mut Compiler, code_object: &mut CodeObject, if_node: IfNode) {
    compiler.compile_expr(code_object, *if_node.condition, &CompileContext::Normal);
    let pop_jump_false_op_index = code_object.operations.len();
    compiler.push_op(code_object, OpIndex::without_op(ByteOp::PopJumpIfFalse));
    make_closure_body(compiler, code_object, if_node.then_body);
    let pop_jump_true_op_index = code_object.operations.len();
    compiler.push_op(code_object, OpIndex::without_op(ByteOp::Jump));
    code_object.operations[pop_jump_false_op_index].operand = compiler.ip;
    make_closure_body(compiler, code_object, if_node.else_body);
    code_object.operations[pop_jump_true_op_index].operand = compiler.ip;
}
