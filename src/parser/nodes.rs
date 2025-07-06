use crate::lexer::token;
use crate::parser::traits::HasId;
use crate::utils::counter::Counter;
use ordered_float::OrderedFloat;

#[derive(Clone, Debug)]
pub enum ExprNode {
    Int(IntNode),
    Float(FloatNode),
    Bool(BoolNode),
    String(StringNode),
    Identity(IdentityNode),
    BinarySubscribe(BinarySubscribeNode),
    AccessAttribute(AccessAttributeNode),
    Reference(ReferenceNode),
    Logical(LogicalNode),
    Comparison(ComparisonNode),
    Binary(BinaryNode),
    Call(CallNode),
    Assign(AssignNode),
    Map(MapNode),
    List(ListNode),
    Function(FunctionNode),
    Class(ClassNode),
    Return(ReturnNode),
    While(WhileNode),
    If(IfNode),
    Null,
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
    Call,
    Assign,
    Object,
    List,
    Function,
    Class,
    Return,
    While,
    If,
    Null,
}

static NODE_ID_COUNTER: Counter = Counter::new();

impl ExprNode {
    pub fn kind(&self) -> ExprKind {
        match self {
            ExprNode::Int(_) => ExprKind::Int,
            ExprNode::Float(_) => ExprKind::Float,
            ExprNode::Bool(_) => ExprKind::Bool,
            ExprNode::String(_) => ExprKind::String,
            ExprNode::Identity(_) => ExprKind::Identity,
            ExprNode::Reference(_) => ExprKind::Reference,
            ExprNode::Binary(_) => ExprKind::Binary,
            ExprNode::Call(_) => ExprKind::Call,
            ExprNode::Assign(_) => ExprKind::Assign,
            ExprNode::Map(_) => ExprKind::Object,
            ExprNode::List(_) => ExprKind::List,
            ExprNode::Logical(_) => ExprKind::Logical,
            ExprNode::Comparison(_) => ExprKind::Comparison,
            ExprNode::BinarySubscribe(_) => ExprKind::AccessConstant,
            ExprNode::AccessAttribute(_) => ExprKind::AccessAttribute,
            ExprNode::Function(_) => ExprKind::Function,
            ExprNode::Class(_) => ExprKind::Class,
            ExprNode::Return(_) => ExprKind::Return,
            ExprNode::While(_) => ExprKind::While,
            ExprNode::If(_) => ExprKind::If,
            ExprNode::Null => ExprKind::Null,
        }
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            ExprNode::Bool(_) | ExprNode::Int(_) | ExprNode::Float(_) | ExprNode::String(_)
        )
    }

    pub fn int(value: i64) -> ExprNode {
        ExprNode::Int(IntNode {
            id: NODE_ID_COUNTER.next(),
            value,
        })
    }

    pub fn float<T: Into<OrderedFloat<f64>>>(value: T) -> ExprNode {
        ExprNode::Float(FloatNode {
            id: NODE_ID_COUNTER.next(),
            value: value.into(),
        })
    }

    pub fn bool(value: bool) -> ExprNode {
        ExprNode::Bool(BoolNode {
            id: NODE_ID_COUNTER.next(),
            value,
        })
    }

    pub fn string(value: String) -> ExprNode {
        ExprNode::String(StringNode {
            id: NODE_ID_COUNTER.next(),
            value,
        })
    }

    pub fn map(properties: Vec<MapProperty>) -> ExprNode {
        ExprNode::Map(MapNode {
            id: NODE_ID_COUNTER.next(),
            properties,
        })
    }

    pub fn list(elements: Vec<ExprNode>) -> ExprNode {
        ExprNode::List(ListNode {
            id: NODE_ID_COUNTER.next(),
            elements,
        })
    }

    pub fn access_constant(value: ExprNode) -> ExprNode {
        ExprNode::BinarySubscribe(BinarySubscribeNode {
            id: NODE_ID_COUNTER.next(),
            value: Box::new(value),
        })
    }

    pub fn access_attribute(value: ExprNode) -> ExprNode {
        ExprNode::AccessAttribute(AccessAttributeNode {
            id: NODE_ID_COUNTER.next(),
            value: Box::new(value),
        })
    }

    pub fn binary(op: token::TokenKind, left: ExprNode, right: ExprNode) -> ExprNode {
        ExprNode::Binary(BinaryNode {
            id: NODE_ID_COUNTER.next(),
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    pub fn comparison(op: token::TokenKind, left: ExprNode, right: ExprNode) -> ExprNode {
        ExprNode::Comparison(ComparisonNode {
            id: NODE_ID_COUNTER.next(),
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    pub fn logical(op: token::TokenKind, left: ExprNode, right: ExprNode) -> ExprNode {
        ExprNode::Logical(LogicalNode {
            id: NODE_ID_COUNTER.next(),
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    pub fn call(identity: IdentityNode, args: Vec<ExprNode>) -> ExprNode {
        ExprNode::Call(CallNode {
            id: NODE_ID_COUNTER.next(),
            identity,
            arguments: args,
        })
    }

    pub fn identity(address: Vec<ExprNode>) -> ExprNode {
        ExprNode::Identity(IdentityNode {
            id: NODE_ID_COUNTER.next(),
            address,
        })
    }

    pub fn reference(identity: IdentityNode) -> ExprNode {
        ExprNode::Reference(ReferenceNode {
            id: NODE_ID_COUNTER.next(),
            identity,
        })
    }

    pub fn assign(identity: IdentityNode, value: ExprNode, return_after: bool) -> ExprNode {
        ExprNode::Assign(AssignNode {
            id: NODE_ID_COUNTER.next(),
            identity,
            value: Box::new(value),
            return_after,
        })
    }

    pub fn function(arguments: Vec<String>, body: Vec<ExprNode>) -> ExprNode {
        ExprNode::Function(FunctionNode {
            id: NODE_ID_COUNTER.next(),
            arguments,
            body,
        })
    }

    pub fn class(parent: Option<ExprNode>, body: Vec<ExprNode>) -> ExprNode {
        ExprNode::Class(ClassNode {
            id: NODE_ID_COUNTER.next(),
            superclass: parent.map(|p| Box::new(p)),
            body,
        })
    }

    pub fn return_n(value: ExprNode) -> ExprNode {
        ExprNode::Return(ReturnNode {
            id: NODE_ID_COUNTER.next(),
            value: Box::new(value),
        })
    }

    pub fn while_n(condition: ExprNode, body: Vec<ExprNode>) -> ExprNode {
        ExprNode::While(WhileNode {
            id: NODE_ID_COUNTER.next(),
            condition: Box::new(condition),
            body,
        })
    }

    pub fn if_n(
        condition: ExprNode,
        then_body: Vec<ExprNode>,
        else_body: Vec<ExprNode>,
    ) -> ExprNode {
        ExprNode::If(IfNode {
            id: NODE_ID_COUNTER.next(),
            condition: Box::new(condition),
            then_body,
            else_body,
        })
    }

    pub fn null() -> ExprNode {
        ExprNode::Null
    }
}

impl HasId for ExprNode {
    fn id(&self) -> usize {
        match self {
            ExprNode::Int(node) => node.id,
            ExprNode::Float(node) => node.id,
            ExprNode::Bool(node) => node.id,
            ExprNode::String(node) => node.id,
            ExprNode::Identity(node) => node.id,
            ExprNode::Reference(node) => node.id,
            ExprNode::Binary(node) => node.id,
            ExprNode::Call(node) => node.id,
            ExprNode::Assign(node) => node.id,
            ExprNode::Map(node) => node.id,
            ExprNode::List(node) => node.id,
            ExprNode::Logical(node) => node.id,
            ExprNode::Comparison(node) => node.id,
            ExprNode::BinarySubscribe(node) => node.id,
            ExprNode::AccessAttribute(node) => node.id,
            ExprNode::Function(node) => node.id,
            ExprNode::Class(node) => node.id,
            ExprNode::Return(node) => node.id,
            ExprNode::While(node) => node.id,
            ExprNode::If(node) => node.id,
            ExprNode::Null => 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IntNode {
    pub id: usize,
    pub value: i64,
}

#[derive(Clone, Debug)]
pub struct FloatNode {
    pub id: usize,
    pub value: OrderedFloat<f64>,
}

#[derive(Clone, Debug)]
pub struct BoolNode {
    pub id: usize,
    pub value: bool,
}

#[derive(Clone, Debug)]
pub struct StringNode {
    pub id: usize,
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct BinaryNode {
    pub id: usize,
    pub operator: token::TokenKind,
    pub left: Box<ExprNode>,
    pub right: Box<ExprNode>,
}

#[derive(Clone, Debug)]
pub struct CallNode {
    pub id: usize,
    pub identity: IdentityNode,
    pub arguments: Vec<ExprNode>,
}

#[derive(Clone, Debug)]
pub struct AssignNode {
    pub id: usize,
    pub identity: IdentityNode,
    pub value: Box<ExprNode>,
    pub return_after: bool,
}

#[derive(Clone, Debug)]
pub struct MapNode {
    pub id: usize,
    pub properties: Vec<MapProperty>,
}

#[derive(Clone, Debug)]
pub struct MapProperty {
    pub key: ExprNode,
    pub value: ExprNode,
}

#[derive(Clone, Debug)]
pub struct ListNode {
    pub id: usize,
    pub elements: Vec<ExprNode>,
}

#[derive(Clone, Debug)]
pub struct LogicalNode {
    pub id: usize,
    pub operator: token::TokenKind,
    pub left: Box<ExprNode>,
    pub right: Box<ExprNode>,
}

#[derive(Clone, Debug)]
pub struct ComparisonNode {
    pub id: usize,
    pub operator: token::TokenKind,
    pub left: Box<ExprNode>,
    pub right: Box<ExprNode>,
}

#[derive(Clone, Debug)]
pub struct BinarySubscribeNode {
    pub id: usize,
    pub value: Box<ExprNode>,
}

#[derive(Clone, Debug)]
pub struct AccessAttributeNode {
    pub id: usize,
    pub value: Box<ExprNode>,
}

#[derive(Clone, Debug)]
pub struct IdentityNode {
    pub id: usize,
    pub address: Vec<ExprNode>,
}

impl IdentityNode {
    pub fn new(address: Vec<ExprNode>) -> Self {
        Self {
            id: NODE_ID_COUNTER.next(),
            address,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ReferenceNode {
    pub id: usize,
    pub identity: IdentityNode,
}

#[derive(Clone, Debug)]
pub struct FunctionNode {
    pub id: usize,
    pub arguments: Vec<String>,
    pub body: Vec<ExprNode>,
}

#[derive(Clone, Debug)]
pub struct ClassNode {
    pub id: usize,
    pub superclass: Option<Box<ExprNode>>,
    pub body: Vec<ExprNode>,
}

#[derive(Clone, Debug)]
pub struct ReturnNode {
    pub id: usize,
    pub value: Box<ExprNode>,
}

#[derive(Clone, Debug)]
pub struct WhileNode {
    pub id: usize,
    pub condition: Box<ExprNode>,
    pub body: Vec<ExprNode>,
}

#[derive(Clone, Debug)]
pub struct IfNode {
    pub id: usize,
    pub condition: Box<ExprNode>,
    pub then_body: Vec<ExprNode>,
    pub else_body: Vec<ExprNode>,
}
