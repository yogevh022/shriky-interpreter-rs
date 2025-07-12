use crate::compiler::byte_operations::ByteComparisonOp;
use crate::runtime::Runtime;
use crate::runtime::value::RuntimeException;
use crate::runtime::value::traits::Binary;
use std::cell::RefCell;
use std::rc::Rc;

pub fn compare(runtime: &mut Runtime, comparison_operand: usize) -> Result<(), RuntimeException> {
    let comparison = ByteComparisonOp::from(comparison_operand as u8);
    let b = runtime.mem_stack.pop().unwrap();
    let a = runtime.mem_stack.pop().unwrap();
    let result = match comparison {
        ByteComparisonOp::Equal => a.borrow_mut().equals(&*b.borrow()),
        ByteComparisonOp::Greater => a.borrow_mut().greater(&*b.borrow()),
        ByteComparisonOp::GreaterEqual => a.borrow_mut().greater_equals(&*b.borrow()),
        ByteComparisonOp::Less => a.borrow_mut().less(&*b.borrow()),
        ByteComparisonOp::LessEqual => a.borrow_mut().less_equals(&*b.borrow()),
        _ => panic!("Unimplemented comparison op: {:?}", comparison),
    };
    runtime.mem_stack.push(Rc::new(RefCell::new(result?)));
    Ok(())
}
