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
    Logical(LogicalNode),
    Comparison(ComparisonNode),
    Binary(BinaryNode),
    FuncCall(FuncCallNode),
    Assign(AssignNode),
    Object(ObjectNode),
    List(ListNode),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ExprKind {
    Int,
    Float,
    Bool,
    String,
    Identity,
    Reference,
    Logical,
    Comparison,
    Binary,
    FuncCall,
    Assign,
    Object,
    List,
}

impl ExprNode {
    pub fn int(value: i64) -> ExprNode {
        ExprNode::Int(value)
    }

    pub fn float<T: Into<OrderedFloat<f64>>>(value: T) -> ExprNode {
        ExprNode::Float(value.into())
    }

    pub fn bool(value: bool) -> ExprNode {
        ExprNode::Bool(value)
    }

    pub fn string(value: String) -> ExprNode {
        ExprNode::String(value)
    }

    pub fn object(properties: Vec<ObjectProperty>) -> ExprNode {
        ExprNode::Object(ObjectNode { properties })
    }

    pub fn list(elements: Vec<ExprNode>) -> ExprNode {
        ExprNode::List(ListNode { elements })
    }

    pub fn binary(op: token::TokenKind, left: ExprNode, right: ExprNode) -> ExprNode {
        ExprNode::Binary(BinaryNode {
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    pub fn comparison(op: token::TokenKind, left: ExprNode, right: ExprNode) -> ExprNode {
        ExprNode::Comparison(ComparisonNode {
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    pub fn logical(op: token::TokenKind, left: ExprNode, right: ExprNode) -> ExprNode {
        ExprNode::Logical(LogicalNode {
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    pub fn kind(&self) -> ExprKind {
        match self {
            ExprNode::Int(_) => ExprKind::Int,
            ExprNode::Float(_) => ExprKind::Float,
            ExprNode::Bool(_) => ExprKind::Bool,
            ExprNode::String(_) => ExprKind::String,
            ExprNode::Identity(_) => ExprKind::Identity,
            ExprNode::Reference(_) => ExprKind::Reference,
            ExprNode::Binary(_) => ExprKind::Binary,
            ExprNode::FuncCall(_) => ExprKind::FuncCall,
            ExprNode::Assign(_) => ExprKind::Assign,
            ExprNode::Object(_) => ExprKind::Object,
            ExprNode::List(_) => ExprKind::List,
            ExprNode::Logical(_) => ExprKind::Logical,
            ExprNode::Comparison(_) => ExprKind::Comparison,
        }
    }
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            ExprNode::Bool(_) | ExprNode::Int(_) | ExprNode::Float(_) | ExprNode::String(_)
        )
    }
}

#[derive(Clone, Debug)]
pub struct IdentityNode {
    pub address: Vec<ExprNode>,
}

#[derive(Clone, Debug)]
pub struct ReferenceNode {
    pub identity: IdentityNode,
}

impl ReferenceNode {
    pub fn new(identity: IdentityNode) -> Self {
        Self { identity }
    }
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
    pub return_after: bool,
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

#[derive(Clone, Debug)]
pub struct LogicalNode {
    pub operator: token::TokenKind,
    pub left: Box<ExprNode>,
    pub right: Box<ExprNode>,
}

#[derive(Clone, Debug)]
pub struct ComparisonNode {
    pub operator: token::TokenKind,
    pub left: Box<ExprNode>,
    pub right: Box<ExprNode>,
}
