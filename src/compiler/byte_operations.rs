use std::fmt;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ByteOp {
    LoadConstant,
    LoadLocal,
    LoadNonlocal,
    LoadScope,
    LoadNull,

    MakeList,
    MakeMap,
    MakeClass,

    // unary
    Negate,
    Not,

    // binary
    Add,
    Sub,
    Mul,
    Div,
    IntDiv,
    Mod,
    Exp,

    Compare,

    LogicalAnd,
    LogicalOr,

    BinarySubscribe,
    AccessAttribute,

    PreAssign,
    PostAssign,
    AssignSubscribe,
    AssignAttribute,

    Call,

    // return
    ReturnValue,

    PopJumpIfFalse,
    Jump,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ByteComparisonOp {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

impl From<u8> for ByteComparisonOp {
    fn from(value: u8) -> Self {
        // unsafe but less of a headache, just make sure the value is in range
        unsafe { std::mem::transmute(value) }
    }
}

#[derive(Clone, Copy)]
pub struct OpIndex {
    pub operation: ByteOp,
    pub operand: usize,
}

impl OpIndex {
    pub fn with_op(op: ByteOp, operand: usize) -> Self {
        Self {
            operation: op,
            operand,
        }
    }

    pub fn without_op(op: ByteOp) -> Self {
        Self {
            operation: op,
            operand: 0,
        }
    }

    pub fn hex(&self) -> String {
        format!("0x{:02X} {}", self.operation as u8, self.operand)
    }
}

impl fmt::Debug for OpIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}, {}]", self.operation, self.operand)
    }
}
