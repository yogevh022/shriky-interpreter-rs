use crate::compiler::ByteOp;
use crate::compiler::code_object::CodeObject;
use crate::runtime::access::*;
use crate::runtime::assign::*;
use crate::runtime::call::*;
use crate::runtime::compare::*;
use crate::runtime::exceptions::RuntimeError;
use crate::runtime::frame::RuntimeFrame;
use crate::runtime::logical::*;
use crate::runtime::make::*;
use crate::runtime::value::traits::Binary;
use crate::runtime::value::{Value, ValueRef};
use crate::runtime::vm::*;
use std;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Runtime {
    pub(crate) mem_stack: Vec<ValueRef>,
    pub(crate) frames_stack: Vec<RuntimeFrame>,
    pub(crate) frames_cache: HashMap<usize, RuntimeFrame>,
    pub(crate) frames_stack_id_lookup: HashMap<usize, Vec<usize>>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            mem_stack: Vec::new(),
            frames_stack: Vec::new(),
            frames_cache: HashMap::new(),
            frames_stack_id_lookup: HashMap::new(),
        }
    }

    pub(crate) fn push_to_frame_stack(&mut self, frame: RuntimeFrame) {
        if self
            .frames_stack
            .last()
            .map(|last_frame| last_frame.code_object_id != frame.code_object_id)
            .unwrap_or(true)
        {
            self.frames_stack_id_lookup
                .entry(frame.code_object_id)
                .or_default()
                .push(self.frames_stack.len());
        }
        self.frames_stack.push(frame);
    }

    pub(crate) fn pop_from_frame_stack(&mut self) -> RuntimeFrame {
        let popped_frame = self.frames_stack.pop().unwrap();
        if self.frames_stack.len()
            <= *self
                .frames_stack_id_lookup
                .get(&popped_frame.code_object_id)
                .unwrap()
                .last()
                .unwrap()
        {
            self.frames_stack_id_lookup
                .get_mut(&popped_frame.code_object_id)
                .unwrap()
                .pop();
        }
        popped_frame
    }

    pub(crate) fn pop_mem_stack_value_or_null(&mut self) -> ValueRef {
        self.mem_stack
            .pop()
            .unwrap_or_else(|| Rc::new(RefCell::new(Value::Null)))
    }

    pub(crate) fn get_code_object_frame(
        &mut self,
        code_object: &CodeObject,
    ) -> Result<&RuntimeFrame, RuntimeError> {
        if self.frames_cache.contains_key(&code_object.id) {
            return Ok(self.frames_cache.get(&code_object.id).unwrap());
        }
        self.push_to_frame_stack(RuntimeFrame::from_co(code_object));
        self.execute(&code_object)?;
        let frame = self.pop_from_frame_stack();
        self.frames_cache.insert(code_object.id, frame);
        Ok(self.frames_cache.get(&code_object.id).unwrap())
    }

    pub(crate) fn execute(&mut self, code_object: &CodeObject) -> Result<(), RuntimeError> {
        let mut ip = 0;
        while let Some(byte_op) = code_object.operations.get(ip) {
            let operation_result = match byte_op.operation {
                ByteOp::LoadConstant => load_constant(self, code_object, byte_op.operand),
                ByteOp::LoadLocal => load_local(self, byte_op.operand),
                ByteOp::LoadScope => load_scope(self, byte_op.operand),
                ByteOp::LoadNonlocal => load_nonlocal(self, byte_op.operand),
                ByteOp::BinarySubscribe => binary_subscribe(self),
                ByteOp::AccessAttribute => access_attr(self),
                ByteOp::PreAssign => pre_assign(self, byte_op.operand),
                ByteOp::AssignSubscribe => assign_subscribe(self),
                ByteOp::AssignAttribute => assign_attribute(self),
                ByteOp::MakeMap => make_map(self, byte_op.operand),
                ByteOp::MakeList => make_list(self, byte_op.operand),
                ByteOp::MakeClass => make_class(self, byte_op.operand == 1),
                ByteOp::Call => call(self, byte_op.operand),
                ByteOp::Add => apply_bin_op(self, Value::add),
                ByteOp::Sub => apply_bin_op(self, Value::sub),
                ByteOp::Mul => apply_bin_op(self, Value::mul),
                ByteOp::Div => apply_bin_op(self, Value::div),
                ByteOp::IntDiv => apply_bin_op(self, Value::int_div),
                ByteOp::Mod => apply_bin_op(self, Value::modulus),
                ByteOp::Exp => apply_bin_op(self, Value::pow),
                ByteOp::Compare => compare(self, byte_op.operand),
                ByteOp::LogicalAnd => logical_and(self),
                ByteOp::LogicalOr => logical_or(self),
                ByteOp::PopJumpIfFalse => {
                    if !pop_check_truthy(self) {
                        ip = byte_op.operand;
                        continue;
                    }
                    Ok(())
                }
                ByteOp::Jump => {
                    ip = byte_op.operand;
                    continue;
                }
                ByteOp::ReturnValue => return Ok(()),
                _ => panic!("Unimplemented {:?}", byte_op.operation),
            };
            ip += 1;
            match operation_result {
                Err(err) => return Err(err),
                _ => {}
            }
        }
        Ok(())
    }

    pub fn run(&mut self, code_object: &CodeObject) {
        // self.print_ast(code_object);
        let frame = RuntimeFrame::from_co(code_object);
        self.push_to_frame_stack(frame);
        let status = self.execute(code_object); // todo catch this
        if let Err(err) = status {
            println!("{:?}", err);
        } else {
            let result = self.pop_from_frame_stack();
            self.print_current_stack_status(code_object, result);
        }
    }

    pub fn print_ast(&self, co: &CodeObject) {
        for q in co.operations.iter() {
            println!("{:?}", q);
        }
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
