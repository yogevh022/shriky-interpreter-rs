use crate::lexer::token;

#[derive(Clone, Debug)]
pub struct IdentityNode {
    pub address: Vec<ExprNode>
}

#[derive(Clone, Debug)]
pub struct BinaryNode {
    pub operator: token::TokenKind,
    pub left: Box<ExprNode>,
    pub right: Box<ExprNode>,
}

#[derive(Clone, Debug)]
pub struct FuncCallNode {
    pub identity: IdentityNode,
    pub args: Vec<ExprNode>
}

#[derive(Clone, Debug)]
pub struct AssignNode {
    pub identity: IdentityNode,
    pub value: Box<ExprNode>,
}

#[derive(Clone, Debug)]
pub enum ExprNode {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Identity(IdentityNode),
    Binary(BinaryNode),
    FuncCall(FuncCallNode),
    Assign(AssignNode)
}