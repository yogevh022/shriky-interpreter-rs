use crate::compiler::ByteOp;
use crate::compiler::code_object::CodeObject;
use crate::runtime::frame::RuntimeFrame;
use crate::runtime::values::Value;
use crate::runtime::assign::*;
use crate::runtime::call::*;
use crate::runtime::compare::*;
use crate::runtime::logical::*;
use crate::runtime::make::*;
use crate::runtime::access::*;
use crate::runtime::vm::*;
use std;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Runtime {
    pub(crate) mem_stack: Vec<Rc<RefCell<Value>>>,
    pub(crate) frames_cache: HashMap<usize, RuntimeFrame>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            mem_stack: Vec::new(),
            frames_cache: HashMap::new(),
        }
    }

    pub(crate) fn pop_mem_stack_value_or_null(&mut self) -> Rc<RefCell<Value>> {
        self.mem_stack
            .pop()
            .unwrap_or_else(|| Rc::new(RefCell::new(Value::Null)))
    }

    pub(crate) fn get_code_object_frame(&mut self, code_object: &CodeObject) -> &RuntimeFrame {
        if self.frames_cache.contains_key(&code_object.id) {
            return self.frames_cache.get(&code_object.id).unwrap();
        }
        let mut frame = RuntimeFrame::from_size(code_object.variables.len());
        self.execute(&code_object, &mut frame);
        self.frames_cache.insert(code_object.id, frame);
        self.frames_cache.get(&code_object.id).unwrap()
    }
    
    pub(crate) fn execute(&mut self, code_object: &CodeObject, frame: &mut RuntimeFrame) {
        let mut ip = 0;
        while let Some(byte_op) = code_object.operations.get(ip) {
            match byte_op.operation {
                ByteOp::LoadConstant => load_constant(self, code_object, byte_op.operand),
                ByteOp::LoadVariable => load_variable(self, frame, byte_op.operand),
                ByteOp::BinarySubscribe => binary_subscribe(self),
                ByteOp::AccessAttribute => access_attr(self),
                ByteOp::PreAssign => pre_assign(self, frame, byte_op.operand),
                ByteOp::AssignSubscribe => assign_subscribe(self),
                ByteOp::AssignAttribute => assign_attribute(self),
                ByteOp::MakeMap => make_map(self, byte_op.operand),
                ByteOp::MakeList => make_list(self, byte_op.operand),
                ByteOp::MakeClass => make_class(self, byte_op.operand == 1),
                ByteOp::Call => call(self, byte_op.operand),
                ByteOp::Add => apply_bin_op(self, Value::bin_add),
                ByteOp::Sub => apply_bin_op(self, Value::bin_sub),
                ByteOp::Mul => apply_bin_op(self, Value::bin_mul),
                ByteOp::Div => apply_bin_op(self, Value::bin_div),
                ByteOp::IntDiv => apply_bin_op(self, Value::bin_int_div),
                ByteOp::Mod => apply_bin_op(self, Value::bin_mod),
                ByteOp::Exp => apply_bin_op(self, Value::bin_exp),
                ByteOp::Compare => compare(self, byte_op.operand),
                ByteOp::LogicalAnd => logical_and(self),
                ByteOp::LogicalOr => logical_or(self),
                ByteOp::PopJumpIfFalse => if !pop_check_truthy(self) {
                    ip = byte_op.operand;
                    continue;
                }
                ByteOp::Jump => {
                    ip = byte_op.operand;
                    continue;
                }
                ByteOp::ReturnValue => return,
                _ => panic!("Unimplemented {:?}", byte_op.operation),
            }
            ip += 1;
        }
    }

    pub fn run(&mut self, code_object: &CodeObject) {
        let mut frame = RuntimeFrame::from_size(code_object.variables.len());
        self.execute(code_object, &mut frame);
        self.print_current_stack_status(code_object, frame);
    }

    pub fn print_current_stack_status(
        &self,
        code_object: &CodeObject,
        runtime_frame: RuntimeFrame,
    ) {
        println!("stack:");
        self.mem_stack
            .iter()
            .for_each(|item| println!("mem {:?}", item.borrow().clone()));
        println!("variables:");
        runtime_frame
            .variables
            .iter()
            .for_each(|item| println!("var {:?}", item.borrow().clone()));
        println!("bytecode:");
        for (i, val) in code_object.operations.iter().enumerate() {
            println!("{}: {:?}", i, val);
        }
        println!(
            "hex: {:?}",
            code_object
                .operations
                .iter()
                .map(|item| item.hex())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
