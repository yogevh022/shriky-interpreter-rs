use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ByteOperation {
    LoadName = 0x01,
    LoadGlobal = 0x02,
    LoadLiteral = 0x03,
    Add = 0x04,
    Return = 0x05,
    Call = 0x06,
    Pop = 0x07,
    StoreName = 0x08,
    AccessLiteral = 0x09,
    Assign = 0x0A,
}

#[derive(Clone, Copy)]
pub struct OperationIndex {
    operation: ByteOperation,
    index: usize,
}

impl OperationIndex {
    pub fn new(operation: ByteOperation, index: usize) -> Self {
        Self { operation, index }
    }
}

impl fmt::Debug for OperationIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}, {}]", self.operation, self.index)
    }
}