use std::fmt;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ByteOp {
    LoadConstant,
    LoadName,
    LoadGlobal,
    LoadNull,

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

    BinarySubscribe,
    Assign,

    Call,

    // return
    ReturnValue,

    StartLoop,
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
