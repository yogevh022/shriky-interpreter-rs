use crate::compiler::byte_operations::ByteComparisonOp;
use crate::runtime::values::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub fn compare(memory_stack: &mut Vec<Rc<RefCell<Value>>>, comparison_operand: usize) {
    let comparison = ByteComparisonOp::from(comparison_operand as u8);
    let b = memory_stack.pop().unwrap();
    let a = memory_stack.pop().unwrap();
    let result = match comparison {
        ByteComparisonOp::Equal => a.borrow().equals(&*b.borrow()),
        ByteComparisonOp::Greater => a.borrow().greater_than(&*b.borrow()),
        ByteComparisonOp::GreaterEqual => a.borrow().greater_than_equals(&*b.borrow()),
        ByteComparisonOp::Less => a.borrow().less_than(&*b.borrow()),
        ByteComparisonOp::LessEqual => a.borrow().less_than_equals(&*b.borrow()),
        _ => panic!("Unimplemented comparison op: {:?}", comparison),
    };
    memory_stack.push(Rc::new(RefCell::new(Value::Bool(result))));
}
