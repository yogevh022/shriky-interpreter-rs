use crate::compiler::byte_operations::OpIndex;
use crate::compiler::code_object::CodeObject;
use crate::compiler::compiler::CompileContext;
use crate::compiler::{ByteOp, Compiler};
use crate::parser::ExprNode;
use crate::parser::nodes::{IfNode, WhileNode};
use std::cell::RefCell;
use std::rc::Rc;

fn make_closure_body(compiler: &mut Compiler, body: Vec<ExprNode>) {
    for ast_node in body.into_iter() {
        compiler.compile_expr(ast_node, &CompileContext::Normal);
    }
}

pub(crate) fn while_closure(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    while_node: WhileNode,
) {
    compiler.compile_expr(*while_node.condition, &CompileContext::Normal);
    let loop_body_start_index = compiler.ip;
    let pop_jump_op_index = {
        let mut_code_obj = &mut *code_object.borrow_mut();
        let idx = mut_code_obj.operations.len();
        compiler.push_op(mut_code_obj, OpIndex::without_op(ByteOp::PopJumpIfFalse));
        idx
    };
    make_closure_body(compiler, while_node.body);

    let mut_code_obj = &mut *code_object.borrow_mut();
    compiler.push_op(
        mut_code_obj,
        OpIndex::with_op(ByteOp::Jump, loop_body_start_index),
    );
    mut_code_obj.operations[pop_jump_op_index].operand = compiler.ip;
}

pub(crate) fn if_closure(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    if_node: IfNode,
) {
    compiler.compile_expr(*if_node.condition, &CompileContext::Normal);

    let pop_jump_false_op_index = {
        let mut_code_obj = &mut *code_object.borrow_mut();
        let idx = mut_code_obj.operations.len();
        compiler.push_op(mut_code_obj, OpIndex::without_op(ByteOp::PopJumpIfFalse));
        idx
    };

    make_closure_body(compiler, if_node.then_body);

    let pop_jump_true_op_index = {
        let mut_code_obj = &mut *code_object.borrow_mut();
        let idx = mut_code_obj.operations.len();
        compiler.push_op(mut_code_obj, OpIndex::without_op(ByteOp::Jump));
        mut_code_obj.operations[pop_jump_false_op_index].operand = compiler.ip;
        idx
    };

    make_closure_body(compiler, if_node.else_body);

    code_object.borrow_mut().operations[pop_jump_true_op_index].operand = compiler.ip;
}
