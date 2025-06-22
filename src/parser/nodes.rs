use crate::lexer::token;
use ordered_float::OrderedFloat;

#[derive(Clone, Debug)]
pub enum ExprNode {
    Int(i64),
    Float(OrderedFloat<f64>),
    Bool(bool),
    String(String),
    Identity(IdentityNode),
    Reference(ReferenceNode),
    Binary(BinaryNode),
    FuncCall(FuncCallNode),
    Assign(AssignNode),
    Object(ObjectNode),
    List(ListNode),
}

impl ExprNode {
    pub fn is_primitive(&self) -> bool {
        matches!(self,
            ExprNode::Bool(_) |
            ExprNode::Int(_) |
            ExprNode::Float(_) |
            ExprNode::String(_))
    }
}

#[derive(Clone, Debug)]
pub struct IdentityNode {
    pub address: Vec<ExprNode>
}

#[derive(Clone, Debug)]
pub struct ReferenceNode {
    pub identity: Box<ExprNode>,
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
    pub args: Vec<ExprNode>,
}

#[derive(Clone, Debug)]
pub struct AssignNode {
    pub identity: IdentityNode,
    pub value: Box<ExprNode>,
    pub return_after: bool
}

#[derive(Clone, Debug)]
pub struct ObjectNode {
    pub properties: Vec<ObjectProperty>,
}

#[derive(Clone, Debug)]
pub struct ObjectProperty {
    pub key: ExprNode,
    pub value: ExprNode,
}

#[derive(Clone, Debug)]
pub struct ListNode {
    pub elements: Vec<ExprNode>,
}