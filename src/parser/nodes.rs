use crate::lexer::token;
use ordered_float::OrderedFloat;

#[derive(Clone, Debug)]
pub enum ExprNode {
    Int(i64),
    Float(OrderedFloat<f64>),
    Bool(bool),
    String(String),
    Identity(IdentityNode),
    AccessConstant(AccessConstantNode),
    AccessAttribute(AccessAttributeNode),
    Reference(ReferenceNode),
    Logical(LogicalNode),
    Comparison(ComparisonNode),
    Binary(BinaryNode),
    FuncCall(FuncCallNode),
    Assign(AssignNode),
    Object(ObjectNode),
    List(ListNode),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum LiteralNode {
    Int(i64),
    Float(OrderedFloat<f64>),
    Bool(bool),
    String(String),
}


#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ExprKind {
    Int,
    Float,
    Bool,
    String,
    Identity,
    AccessConstant,
    AccessAttribute,
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

    pub fn access_constant(index: ExprNode, value: ExprNode) -> ExprNode {
        ExprNode::AccessConstant(AccessConstantNode {
            index: Box::new(index),
            value: Box::new(value),
        })
    }

    pub fn access_attribute(index: ExprNode, value: ExprNode) -> ExprNode {
        ExprNode::AccessAttribute(AccessAttributeNode {
            index: Box::new(index),
            value: Box::new(value),
        })
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

    pub fn func_call(identity: ExprNode, args: Vec<ExprNode>) -> ExprNode {
        ExprNode::FuncCall(FuncCallNode {
            identity: Box::new(identity),
            args,
        })
    }

    pub fn identity(value: ExprNode) -> ExprNode {
        ExprNode::Identity(IdentityNode {
            value: Box::new(value),
        })
    }

    pub fn reference(identity: IdentityNode) -> ExprNode {
        ExprNode::Reference(ReferenceNode::new(identity))
    }

    pub fn assign(identity: IdentityNode, value: ExprNode, return_after: bool) -> ExprNode {
        ExprNode::Assign(AssignNode {
            identity,
            value: Box::new(value),
            return_after,
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
            ExprNode::AccessConstant(_) => ExprKind::AccessConstant,
            ExprNode::AccessAttribute(_) => ExprKind::AccessAttribute,
        }
    }
    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            ExprNode::Bool(_) | ExprNode::Int(_) | ExprNode::Float(_) | ExprNode::String(_)
        )
    }
    
    pub fn to_literal(&self) -> LiteralNode {
        match self {
            ExprNode::Bool(value) => LiteralNode::Bool(*value),
            ExprNode::Int(value) => LiteralNode::Int(*value),
            ExprNode::Float(value) => LiteralNode::Float(*value),
            ExprNode::String(value) => LiteralNode::String(value.to_owned()),
            _ => panic!("Cannot convert to literal"),
        }
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
    pub identity: Box<ExprNode>,
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

#[derive(Clone, Debug)]
pub struct AccessConstantNode {
    pub index: Box<ExprNode>,
    pub value: Box<ExprNode>,
}

#[derive(Clone, Debug)]
pub struct AccessAttributeNode {
    pub index: Box<ExprNode>,
    pub value: Box<ExprNode>,
}

#[derive(Clone, Debug)]
pub struct IdentityNode {
    pub value: Box<ExprNode>,
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
