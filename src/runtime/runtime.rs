use crate::compiler::ByteOp;
use crate::compiler::code_object::CodeObject;
use crate::runtime::assign::*;
use crate::runtime::call::*;
use crate::runtime::compare::*;
use crate::runtime::frame::RuntimeFrame;
use crate::runtime::logical::*;
use crate::runtime::make::*;
use crate::runtime::values::{FunctionValue, Value};
use std;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Runtime {
    memory_stack: Vec<Rc<RefCell<Value>>>,
    scope_stack: Vec<RuntimeFrame>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            memory_stack: Vec::new(),
            scope_stack: Vec::new(),
        }
    }

    fn binary_subscribe(&mut self) {
        let constant = self.memory_stack.pop().unwrap();
        let container = self.memory_stack.pop().unwrap();
        let constant_ref = constant.borrow();
        let container_ref = container.borrow();
        let result = match &*container_ref {
            Value::Map(obj) => obj.properties.get(&*constant_ref).unwrap(),
            Value::List(list) => {
                if let Value::Int(index) = constant_ref.clone() {
                    list.elements.get(index as usize).unwrap()
                } else {
                    panic!("Can only subscribe to lists with integers")
                }
            }
            _ => panic!("Invalid type for binary subscribe"),
        };
        self.memory_stack.push(result.clone());
    }

    fn call_function_result(
        &mut self,
        function_value: &FunctionValue,
        args: Vec<Rc<RefCell<Value>>>,
    ) {
        let mut frame = get_function_runtime_frame(function_value, args);
        self.execute(&function_value.body, &mut frame);
        let return_value = self
            .memory_stack
            .pop()
            .unwrap_or_else(|| Rc::new(RefCell::new(Value::Null)));
        self.memory_stack.push(return_value);
    }

    fn call(&mut self, arg_count: usize) {
        let callee = self.memory_stack.pop().unwrap();
        let args: Vec<Rc<RefCell<Value>>> = (0..arg_count)
            .map(|_| self.memory_stack.pop().unwrap())
            .collect();
        match &*callee.borrow() {
            Value::Function(func_value) => self.call_function_result(func_value, args),
            Value::Class(_) => todo!(),
            _ => panic!("Called uncallable value"),
        }
    }

    fn apply_bin_op<F>(&mut self, f: F)
    where
        F: Fn(&Value, &Value) -> Value,
    {
        let b = self.memory_stack.pop().unwrap();
        let a = self.memory_stack.pop().unwrap();
        self.memory_stack
            .push(Rc::new(RefCell::new(f(&*a.borrow(), &*b.borrow()))));
    }

    fn execute(&mut self, code_object: &CodeObject, frame: &mut RuntimeFrame) {
        let mut ip = 0;
        while let Some(byte_op) = code_object.operations.get(ip) {
            match byte_op.operation {
                ByteOp::LoadConstant => {
                    let constant_value = code_object.constants[byte_op.operand].clone();
                    self.memory_stack.push(constant_value);
                }
                ByteOp::LoadVariable => {
                    let var_value = frame.variables[byte_op.operand].clone();
                    self.memory_stack.push(var_value);
                }
                ByteOp::BinarySubscribe => self.binary_subscribe(),
                ByteOp::PreAssign => pre_assign(&mut self.memory_stack, frame, byte_op.operand),
                ByteOp::AssignSubscribe => assign_subscribe(&mut self.memory_stack),
                ByteOp::AssignAttribute => assign_attribute(&mut self.memory_stack),
                ByteOp::MakeMap => make_map(&mut self.memory_stack, byte_op.operand),
                ByteOp::MakeList => make_list(&mut self.memory_stack, byte_op.operand),
                ByteOp::MakeClass => make_class(&mut self.memory_stack, byte_op.operand == 1),
                ByteOp::ReturnValue => return,
                ByteOp::Call => self.call(byte_op.operand),
                ByteOp::Add => self.apply_bin_op(Value::bin_add),
                ByteOp::Sub => self.apply_bin_op(Value::bin_sub),
                ByteOp::Mul => self.apply_bin_op(Value::bin_mul),
                ByteOp::Div => self.apply_bin_op(Value::bin_div),
                ByteOp::IntDiv => self.apply_bin_op(Value::bin_int_div),
                ByteOp::Mod => self.apply_bin_op(Value::bin_mod),
                ByteOp::Exp => self.apply_bin_op(Value::bin_exp),
                ByteOp::Compare => compare(&mut self.memory_stack, byte_op.operand),
                ByteOp::LogicalAnd => logical_and(&mut self.memory_stack),
                ByteOp::LogicalOr => logical_or(&mut self.memory_stack),
                ByteOp::PopJumpIfFalse => {
                    let condition = self.memory_stack.pop().unwrap();
                    if !(&*condition.borrow()).is_truthy() {
                        ip = byte_op.operand;
                        continue;
                    }
                }
                ByteOp::Jump => {
                    ip = byte_op.operand;
                    continue;
                }
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
        self.memory_stack
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
