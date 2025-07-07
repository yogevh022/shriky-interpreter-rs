use crate::runtime::values::Value;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct RuntimeFrame {
    pub variables: Vec<Rc<RefCell<Value>>>,
}

impl RuntimeFrame {
    pub fn from_size(size: usize) -> Self {
        Self {
            variables: (0..size)
                .map(|_| Rc::new(RefCell::new(Value::Null)))
                .collect(),
        }
    }
}
