use crate::compiler::ByteOp;
use crate::compiler::code_object::CodeObject;
use crate::runtime::assign::*;
use crate::runtime::call::*;
use crate::runtime::compare::*;
use crate::runtime::frame::RuntimeFrame;
use crate::runtime::logical::*;
use crate::runtime::make::*;
use crate::runtime::utils::{extract_class, extract_function, extract_string};
use crate::runtime::values::{ClassValue, FunctionValue, Value};
use std;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Runtime {
    mem_stack: Vec<Rc<RefCell<Value>>>,
    frames_cache: HashMap<usize, RuntimeFrame>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            mem_stack: Vec::new(),
            frames_cache: HashMap::new(),
        }
    }

    fn binary_subscribe(&mut self) {
        let constant = self.mem_stack.pop().unwrap();
        let container = self.mem_stack.pop().unwrap();
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
        self.mem_stack.push(result.clone());
    }

    fn get_inherited_attr(
        &mut self,
        class_value: ClassValue,
        attr_string: String,
    ) -> Option<Rc<RefCell<Value>>> {
        let code_object = class_value.body;
        if let Some(attr_index) = code_object.variable_index_lookup.get(&attr_string) {
            Some(self.get_code_object_frame(&code_object).variables[*attr_index].clone())
        } else if let Some(superclass) = class_value.parent {
            let superclass_value = extract_class(&superclass);
            self.get_inherited_attr(superclass_value, attr_string)
        } else {
            None
        }
    }

    fn access_attr(&mut self) {
        let attr = self.mem_stack.pop().unwrap();
        let container = self.mem_stack.pop().unwrap();
        let attr_string = extract_string(&attr);
        let attr_value = match &*container.borrow() {
            Value::Instance(instance_value) => {
                if let Some(attr_value) = instance_value.attributes.get(&attr_string) {
                    Some(attr_value.clone())
                } else {
                    let class_value = extract_class(&instance_value.class);
                    self.get_inherited_attr(class_value, attr_string)
                }
            }
            _ => panic!("Invalid type for attribute access"),
        };
        self.mem_stack
            .push(attr_value.expect("Attribute not found"))
    }

    fn pop_mem_stack_value_or_null(&mut self) -> Rc<RefCell<Value>> {
        self.mem_stack
            .pop()
            .unwrap_or_else(|| Rc::new(RefCell::new(Value::Null)))
    }

    fn get_code_object_frame(&mut self, code_object: &CodeObject) -> &RuntimeFrame {
        if self.frames_cache.contains_key(&code_object.id) {
            return self.frames_cache.get(&code_object.id).unwrap();
        }
        let mut frame = RuntimeFrame::from_size(code_object.variables.len());
        self.execute(&code_object, &mut frame);
        self.frames_cache.insert(code_object.id, frame);
        self.frames_cache.get(&code_object.id).unwrap()
    }

    fn make_instance(&mut self, value_cls: Rc<RefCell<Value>>, mut args: Vec<Rc<RefCell<Value>>>) {
        let class_value = extract_class(&value_cls);
        let class_code_object = class_value.body;
        let instance = Rc::new(RefCell::new(Value::instance(value_cls, HashMap::new())));
        let frame = self.get_code_object_frame(&class_code_object);
        if let Some(init_func_index) = class_code_object.variable_index_lookup.get("init") {
            if let Some(init_func) = frame.variables.get(*init_func_index) {
                let init_func_value = extract_function(init_func);
                args.push(instance.clone());
                // execute init if exists
                self.execute(
                    &init_func_value.body,
                    &mut get_function_runtime_frame(&init_func_value, args),
                );
            } else {
                panic!("Invalid class init function index");
            }
        };
        self.mem_stack.push(instance);
    }

    fn call(&mut self, arg_count: usize) {
        let callee = self.mem_stack.pop().unwrap();
        let args: Vec<Rc<RefCell<Value>>> = (0..arg_count)
            .map(|_| self.mem_stack.pop().unwrap())
            .collect();
        match &*callee.borrow() {
            Value::Function(func_value) => {
                self.execute(
                    &func_value.body,
                    &mut get_function_runtime_frame(func_value, args),
                );
                let return_value = self.pop_mem_stack_value_or_null();
                self.mem_stack.push(return_value);
            }
            Value::Class(_) => self.make_instance(callee.clone(), args),
            _ => panic!("Called uncallable value"),
        }
    }

    fn apply_bin_op<F>(&mut self, f: F)
    where
        F: Fn(&Value, &Value) -> Value,
    {
        let b = self.mem_stack.pop().unwrap();
        let a = self.mem_stack.pop().unwrap();
        self.mem_stack
            .push(Rc::new(RefCell::new(f(&*a.borrow(), &*b.borrow()))));
    }

    fn execute(&mut self, code_object: &CodeObject, frame: &mut RuntimeFrame) {
        let mut ip = 0;
        while let Some(byte_op) = code_object.operations.get(ip) {
            match byte_op.operation {
                ByteOp::LoadConstant => {
                    let constant_value = code_object.constants[byte_op.operand].clone();
                    self.mem_stack.push(constant_value);
                }
                ByteOp::LoadVariable => {
                    let var_value = frame.variables[byte_op.operand].clone();
                    self.mem_stack.push(var_value);
                }
                ByteOp::BinarySubscribe => self.binary_subscribe(),
                ByteOp::AccessAttribute => self.access_attr(),
                ByteOp::PreAssign => pre_assign(&mut self.mem_stack, frame, byte_op.operand),
                ByteOp::AssignSubscribe => assign_subscribe(&mut self.mem_stack),
                ByteOp::AssignAttribute => assign_attribute(&mut self.mem_stack),
                ByteOp::MakeMap => make_map(&mut self.mem_stack, byte_op.operand),
                ByteOp::MakeList => make_list(&mut self.mem_stack, byte_op.operand),
                ByteOp::MakeClass => make_class(&mut self.mem_stack, byte_op.operand == 1),
                ByteOp::ReturnValue => return,
                ByteOp::Call => self.call(byte_op.operand),
                ByteOp::Add => self.apply_bin_op(Value::bin_add),
                ByteOp::Sub => self.apply_bin_op(Value::bin_sub),
                ByteOp::Mul => self.apply_bin_op(Value::bin_mul),
                ByteOp::Div => self.apply_bin_op(Value::bin_div),
                ByteOp::IntDiv => self.apply_bin_op(Value::bin_int_div),
                ByteOp::Mod => self.apply_bin_op(Value::bin_mod),
                ByteOp::Exp => self.apply_bin_op(Value::bin_exp),
                ByteOp::Compare => compare(&mut self.mem_stack, byte_op.operand),
                ByteOp::LogicalAnd => logical_and(&mut self.mem_stack),
                ByteOp::LogicalOr => logical_or(&mut self.mem_stack),
                ByteOp::PopJumpIfFalse => {
                    let condition = self.mem_stack.pop().unwrap();
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
