use ordered_float::OrderedFloat;
use crate::interpreter::Interpreter;
use crate::lexer::TokenKind;
use crate::parser::ExprNode;
use crate::parser::nodes::BinaryNode;
use crate::runtime::values::RuntimeValue;

impl<'a> Interpreter<'a> {
    
    // fn binary_plus_int(&mut self, left: i64, right: ExprNode) -> ExprNode {
    //     match right {
    //         ExprNode::Int(v) => ExprNode::Int(left + v),
    //         ExprNode::Float(v) => ExprNode::Float(OrderedFloat(left as f64) + v),
    //         ExprNode::Binary(v) => {},
    //         ExprNode::FuncCall(v) => {},
    //         ExprNode::Identity(v) => {},
    //         _ => panic!("Cannot add int + {:?}", right),
    //     }
    // }
    // fn binary_plus(&mut self, left: Box<ExprNode>, right: Box<ExprNode>) {
    //     match left {
    //         ExprNode::Int(v) => self.binary_plus_int(v, *right),
    //         ExprNode::Float(v) => {},
    //         ExprNode::String(v) => {},
    //         ExprNode::FuncCall(v) => {},
    //         ExprNode::Identity(v) => {},
    //         ExprNode::Binary(v) => {},
    //         _ => panic!("Cannot add {:?} + {:?}", left, right),
    //     }
    // }
    // 
    // fn binary_op(&mut self, op: BinaryNode) -> ExprNode {
    //     match op.operator {
    //         TokenKind::Plus => {},
    //         TokenKind::Minus => {},
    //         TokenKind::Asterisk => {},
    //         TokenKind::Slash => {},
    //         TokenKind::DoubleSlash => {},
    //         TokenKind::Exponent => {},
    //         TokenKind::Modulo => {},
    //         _ => unreachable!()
    //     }
    // }
}