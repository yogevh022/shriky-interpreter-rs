use crate::compiler::byte_operations::*;
use crate::compiler::closure::*;
use crate::compiler::code_object::CodeObject;
use crate::compiler::load::*;
use crate::compiler::make::*;
use crate::compiler::op::*;
use crate::parser::ExprNode;
use crate::parser::traits::HasId;
use crate::runtime::value::Value;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum CompileContext {
    Function,
    Class,
    Assignment,
    Normal,
}

pub struct Compiler {
    pub(crate) ip: usize,
    pub(crate) scope_stack: Vec<Rc<RefCell<CodeObject>>>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            ip: 0,
            scope_stack: Vec::new(),
        }
    }

    pub(crate) fn push_op(&mut self, code_object: &mut CodeObject, op: OpIndex) {
        code_object.operations.push(op);
        self.ip += 1;
    }

    pub(crate) fn compile_expr(&mut self, expr: ExprNode, context: &CompileContext) {
        let code_object_ref = self.scope_stack.last().unwrap().clone();
        match expr {
            ExprNode::Int(_) | ExprNode::Float(_) | ExprNode::Bool(_) | ExprNode::String(_) => {
                load_constant(
                    self,
                    code_object_ref,
                    expr.id(),
                    Value::from_expr(expr).ok().unwrap(),
                )
            }
            ExprNode::Map(map) => make_map(self, code_object_ref, map),
            ExprNode::List(list) => make_list(self, code_object_ref, list),
            ExprNode::Function(function_node) => {
                if matches!(context, CompileContext::Class)
                    && function_node
                        .arguments
                        .first()
                        .is_some_and(|arg| arg == "self")
                {
                    // class methods without self arg are static (regular functions)
                    make_method(self, code_object_ref, function_node);
                } else {
                    make_function(self, code_object_ref, function_node);
                }
            }
            ExprNode::Class(class_node) => make_class(self, code_object_ref, class_node, context),
            ExprNode::Identity(identity_node) => {
                identity(self, code_object_ref, identity_node, context)
            }
            ExprNode::Call(call_node) => call(self, code_object_ref, call_node, context),
            ExprNode::Assign(assign_node) => assign(self, code_object_ref, assign_node, context),
            ExprNode::Binary(binary_node) => binary(self, code_object_ref, binary_node, context),
            ExprNode::If(if_node) => if_closure(self, code_object_ref, if_node),
            ExprNode::While(while_node) => while_closure(self, code_object_ref, while_node),
            ExprNode::Comparison(comparison_node) => {
                comparison(self, code_object_ref, comparison_node)
            }
            ExprNode::Logical(logical_node) => logical(self, code_object_ref, logical_node),
            ExprNode::Return(return_node) => {
                return_value(self, code_object_ref, return_node, context)
            }
            ExprNode::Null => self.push_op(
                &mut *code_object_ref.borrow_mut(),
                OpIndex::without_op(ByteOp::LoadNull),
            ),
            _ => panic!("Unexpected expr node: {:?}", expr),
        }
    }

    pub fn compile_into(
        &mut self,
        ast: Vec<ExprNode>,
        context: &CompileContext,
        code_object: Rc<RefCell<CodeObject>>,
    ) {
        self.scope_stack.push(code_object);
        for ast_node in ast.into_iter() {
            self.compile_expr(ast_node, context);
        }
        self.scope_stack.pop();
    }

    pub fn compile(&mut self, ast: Vec<ExprNode>, context: &CompileContext) -> CodeObject {
        let code_object = Rc::new(RefCell::new(CodeObject::default()));
        self.compile_into(ast, context, code_object.clone());
        Rc::try_unwrap(code_object).unwrap().into_inner()
    }
}
