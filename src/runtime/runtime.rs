use crate::compiler::ByteOp;
use crate::compiler::byte_operations::ByteComparisonOp;
use crate::compiler::code_object::CodeObject;
use crate::runtime::frame::RuntimeFrame;
use crate::runtime::values::{ClassValue, FunctionValue, Value};
use std;
use std::cell::{Ref, RefCell};
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

    fn pre_assign(&mut self, frame: &RuntimeFrame, variable_index: usize) {
        let value = self.memory_stack.pop().unwrap();
        let var = frame.variables[variable_index].clone();
        let cloned = value.borrow().clone();
        *var.borrow_mut() = cloned;
    }

    fn assign_subscribe(&mut self) {
        let value = self.memory_stack.pop().unwrap();
        let key = self.memory_stack.pop().unwrap();
        let container = self.memory_stack.pop().unwrap();
        match &mut *container.borrow_mut() {
            Value::Map(obj) => {
                obj.properties.insert(key.borrow().clone(), value.clone());
            }
            Value::List(list) => {
                if let Value::Int(index) = key.borrow().clone() {
                    return list.elements.insert(index as usize, value.clone());
                }
                panic!("Can only subscribe to lists with integers")
            }
            _ => panic!("Invalid type for binary subscribe"),
        }
    }

    fn call_function(&mut self, function_value: &FunctionValue, args: Vec<Rc<RefCell<Value>>>) {
        let func_code_obj = function_value.clone().body;
        let mut func_runtime_frame = RuntimeFrame::from_size(func_code_obj.variables.len());
        function_value
            .parameters
            .iter()
            .zip(args.iter().rev())
            .for_each(|(p, v)| {
                func_runtime_frame.variables[func_code_obj.variable_index_lookup[p]] = v.clone();
            });
        self.run(&func_code_obj);
        let return_value = self
            .memory_stack
            .pop()
            .unwrap_or_else(|| Rc::new(RefCell::new(Value::Null)));
        self.memory_stack.push(return_value);
    }

    fn call_class(&mut self, class_value: &ClassValue, args: Vec<Rc<RefCell<Value>>>) {
        let class_code_object = class_value.body.clone();
        self.scope_stack // object initialization frame
            .push(RuntimeFrame::from_size(class_code_object.variables.len()));
        self.execute(&class_code_object); // load class
        let constructor_index = class_code_object.variable_index_lookup.get("con").unwrap();
        let constructor_func = self
            .scope_stack
            .last()
            .unwrap()
            .variables
            .get(*constructor_index)
            .unwrap()
            .clone();
        match &*constructor_func.borrow() {
            Value::Function(func_value) => self.call_function(func_value, args),
            _ => panic!("Invalid class constructor"),
        }
        self.scope_stack.pop(); // pop object initialization frame
    }

    fn call(&mut self, arg_count: usize) {
        let callee = self.memory_stack.pop().unwrap();
        let args: Vec<Rc<RefCell<Value>>> = (0..arg_count)
            .map(|_| self.memory_stack.pop().unwrap())
            .collect();
        match &*callee.borrow() {
            Value::Function(func_value) => self.call_function(func_value, args),
            Value::Class(class_value) => self.call_class(class_value, args),
            _ => panic!("Called uncallable value"),
        }
    }

    fn make_map(&mut self, property_count: usize) {
        let properties_kv: Vec<Rc<RefCell<Value>>> = self
            .memory_stack
            .drain(self.memory_stack.len() - property_count..)
            .collect();

        let mut properties = indexmap::IndexMap::new();
        for kv in properties_kv.chunks(2) {
            match kv {
                [k, v] => {
                    properties.insert(k.borrow().clone(), v.clone());
                }
                _ => unreachable!("Map key without a value"),
            }
        }
        self.memory_stack
            .push(Rc::new(RefCell::new(Value::map(properties))));
    }

    fn make_list(&mut self, list_size: usize) {
        let list_items = self
            .memory_stack
            .drain(self.memory_stack.len() - list_size..)
            .rev()
            .collect();
        self.memory_stack
            .push(Rc::new(RefCell::new(Value::list(list_items))));
    }

    fn make_class(&mut self, is_inheriting: bool) {
        let maybe_class_value = self.memory_stack.pop().unwrap();
        let superclass_ref = if is_inheriting {
            let superclass = self.memory_stack.pop().unwrap();
            Some(superclass.clone())
        } else {
            None
        };
        let class_code_obj = match &*maybe_class_value.borrow() {
            Value::Class(class_value) => class_value.body.clone(),
            _ => panic!("Invalid class code object"),
        };
        self.memory_stack.push(Rc::new(RefCell::new(Value::class(
            superclass_ref,
            class_code_obj,
        ))));
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

    fn compare(&mut self, comparison_operand: usize) {
        let comparison = ByteComparisonOp::from(comparison_operand as u8);
        let b = self.memory_stack.pop().unwrap();
        let a = self.memory_stack.pop().unwrap();
        let result = match comparison {
            ByteComparisonOp::Equal => a.borrow().equals(&*b.borrow()),
            ByteComparisonOp::Greater => a.borrow().greater_than(&*b.borrow()),
            ByteComparisonOp::GreaterEqual => a.borrow().greater_than_equals(&*b.borrow()),
            ByteComparisonOp::Less => a.borrow().less_than(&*b.borrow()),
            ByteComparisonOp::LessEqual => a.borrow().less_than_equals(&*b.borrow()),
            _ => panic!("Unimplemented comparison op: {:?}", comparison),
        };
        self.memory_stack
            .push(Rc::new(RefCell::new(Value::Bool(result))));
    }

    fn logical_and(&mut self) {
        let b = self.memory_stack.pop().unwrap();
        let a = self.memory_stack.pop().unwrap();
        let result = a.borrow().is_truthy() && b.borrow().is_truthy();
        self.memory_stack
            .push(Rc::new(RefCell::new(Value::Bool(result))));
    }

    fn logical_or(&mut self) {
        let b = self.memory_stack.pop().unwrap();
        let a = self.memory_stack.pop().unwrap();
        let result = a.borrow().is_truthy() || b.borrow().is_truthy();
        self.memory_stack
            .push(Rc::new(RefCell::new(Value::Bool(result))));
    }

    fn execute(&mut self, code_object: &CodeObject) {
        let mut frame = self.scope_stack.pop().unwrap();
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
                ByteOp::PreAssign => self.pre_assign(&mut frame, byte_op.operand),
                ByteOp::AssignSubscribe => self.assign_subscribe(),
                ByteOp::MakeMap => self.make_map(byte_op.operand),
                ByteOp::MakeList => self.make_list(byte_op.operand),
                ByteOp::MakeClass => self.make_class(byte_op.operand == 1),
                ByteOp::ReturnValue => return,
                ByteOp::Call => self.call(byte_op.operand),
                ByteOp::Add => self.apply_bin_op(Value::bin_add),
                ByteOp::Sub => self.apply_bin_op(Value::bin_sub),
                ByteOp::Mul => self.apply_bin_op(Value::bin_mul),
                ByteOp::Div => self.apply_bin_op(Value::bin_div),
                ByteOp::IntDiv => self.apply_bin_op(Value::bin_int_div),
                ByteOp::Mod => self.apply_bin_op(Value::bin_mod),
                ByteOp::Exp => self.apply_bin_op(Value::bin_exp),
                ByteOp::Compare => self.compare(byte_op.operand),
                ByteOp::LogicalAnd => self.logical_and(),
                ByteOp::LogicalOr => self.logical_or(),
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
        self.print_current_stack_status(code_object, frame);
    }

    pub fn run(&mut self, code_object: &CodeObject) {
        self.scope_stack
            .push(RuntimeFrame::from_size(code_object.variables.len()));
        self.execute(code_object);

        self.scope_stack.pop();
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
