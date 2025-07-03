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
    BinarySubscribe = 0x09,
    Assign = 0x0A,
    Sub = 0x0B,
    Div = 0x0C,
    Mul = 0x0D,
    Negate = 0x0E,
    Not = 0x0F,
    Greater = 0x10,
    Less = 0x11,
    GreaterEqual = 0x12,
    LessEqual = 0x13,
    Equal = 0x14,
    Exp = 0x15,
    Mod = 0x16,
    IntDiv = 0x17,
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

    pub fn hex(&self) -> String {
        format!("0x{:02X} {}", self.operation as u8, self.index)
    }
}

impl fmt::Debug for OperationIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}, {}]", self.operation, self.index)
    }
}
