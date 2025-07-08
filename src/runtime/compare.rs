use crate::compiler::byte_operations::ByteComparisonOp;
use crate::runtime::Runtime;
use crate::runtime::exceptions::RuntimeError;
use crate::runtime::values::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub fn compare(runtime: &mut Runtime, comparison_operand: usize) -> Result<(), RuntimeError> {
    let comparison = ByteComparisonOp::from(comparison_operand as u8);
    let b = runtime.mem_stack.pop().unwrap();
    let a = runtime.mem_stack.pop().unwrap();
    let result = match comparison {
        ByteComparisonOp::Equal => a.borrow().equals(&*b.borrow()),
        ByteComparisonOp::Greater => a.borrow().greater_than(&*b.borrow()),
        ByteComparisonOp::GreaterEqual => a.borrow().greater_than_equals(&*b.borrow()),
        ByteComparisonOp::Less => a.borrow().less_than(&*b.borrow()),
        ByteComparisonOp::LessEqual => a.borrow().less_than_equals(&*b.borrow()),
        _ => panic!("Unimplemented comparison op: {:?}", comparison),
    };
    runtime
        .mem_stack
        .push(Rc::new(RefCell::new(Value::Bool(result))));
    Ok(())
}
