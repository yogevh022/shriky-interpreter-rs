use crate::compiler::ByteOp;
use crate::compiler::code_object::*;
use std;
use std::arch::x86_64::_popcnt32;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Runtime {
    memory_stack: Vec<Rc<RefCell<Value>>>,
    scope_stack: Vec<Rc<RefCell<CodeObject>>>,
    ip: usize,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            memory_stack: Vec::new(),
            scope_stack: Vec::new(),
            ip: 0,
        }
    }

    fn binary_subscribe(&mut self) {
        let constant = self.memory_stack.pop().unwrap();
        let container = self.memory_stack.pop().unwrap();
        let constant_ref = constant.borrow();
        let container_ref = container.borrow();
        let result = match &*container_ref {
            Value::Object(obj) => obj.properties.get(&*constant_ref).unwrap(),
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

    fn pre_assign(&mut self, scope: &CodeObject, operand: usize) {
        let value = self.memory_stack.pop().unwrap();
        let var = scope.variables[operand].clone();
        *var.borrow_mut() = value.borrow().clone();
    }
    
    fn assign_subscribe(&mut self) {
        let value = self.memory_stack.pop().unwrap();
        let key = self.memory_stack.pop().unwrap();
        let container = self.memory_stack.pop().unwrap();
        match &mut *container.borrow_mut() {
            Value::Object(obj) => {
                obj.properties.insert(key.borrow().clone(), value.clone());
            },
            Value::List(list) => {
                if let Value::Int(index) = key.borrow().clone() {
                    return list.elements.insert(index as usize, value.clone());
                }
                panic!("Can only subscribe to lists with integers")
            },
            _ => panic!("Invalid type for binary subscribe"),
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

    fn execute(&mut self, scope: Rc<RefCell<CodeObject>>) {
        for byte_op in scope.borrow().operations.iter() {
            match byte_op.operation {
                ByteOp::LoadConstant => {
                    let constant_value = scope.borrow().constants[byte_op.operand].clone();
                    self.memory_stack.push(constant_value);
                }
                ByteOp::LoadVariable => {
                    let var_name = scope.borrow().variables[byte_op.operand].clone();
                    self.memory_stack.push(var_name);
                }
                ByteOp::BinarySubscribe => self.binary_subscribe(),
                ByteOp::PreAssign => self.pre_assign(&scope.borrow(), byte_op.operand),
                ByteOp::AssignSubscribe => self.assign_subscribe(),
                ByteOp::Add => self.apply_bin_op(Value::bin_add),
                ByteOp::Sub => self.apply_bin_op(Value::bin_sub),
                ByteOp::Mul => self.apply_bin_op(Value::bin_mul),
                ByteOp::Div => self.apply_bin_op(Value::bin_div),
                ByteOp::IntDiv => self.apply_bin_op(Value::bin_int_div),
                ByteOp::Mod => self.apply_bin_op(Value::bin_mod),
                ByteOp::Exp => self.apply_bin_op(Value::bin_exp),
                _ => panic!("Unimplemented {:?}", byte_op.operation),
            }
        }
    }

    pub fn run(&mut self, code_object: CodeObject) {
        self.scope_stack.push(Rc::new(RefCell::new(code_object)));
        let current_scope = self.scope_stack.last().unwrap().clone();
        self.execute(current_scope);

        println!("mem: {:?}", self.memory_stack);
        println!(
            "vars: {:?}",
            self.scope_stack.last().unwrap().borrow().variables
        );
        println!(
            "consts: {:?}",
            self.scope_stack.last().unwrap().borrow().constants
        );
    }

    pub fn print_code_object(&self, code_obj: CodeObject) {
        println!("bytecode:");
        for (i, val) in code_obj.operations.iter().enumerate() {
            println!("{}: {:?}", i, val);
        }
        println!(
            "hex: {:?}",
            code_obj
                .operations
                .iter()
                .map(|item| item.hex())
                .collect::<Vec<String>>()
                .join(" ")
        );
        println!("variables: {:?}", code_obj.variables);
        println!("constants: {:?}", code_obj.constants)
    }
}
