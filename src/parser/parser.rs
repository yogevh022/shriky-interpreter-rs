use std::collections::{HashMap, HashSet};
use crate::lexer::{Lexer, TokenKind};
use crate::lexer::Token;
use crate::parser::nodes::{AssignNode, BinaryNode, ExprNode, FuncCallNode, IdentityNode};

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
    expr_handlers: HashMap<TokenKind, for<'b> fn(&'b mut Parser<'a>) -> ExprNode>,
    identity_handlers: HashMap<TokenKind, for<'b> fn(&'b mut Parser<'a>, &mut IdentityNode)>,
    assignment_token_kinds: HashSet<TokenKind>,
    augmented_assignment_to_arithmetic: HashMap<TokenKind, TokenKind>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        let current_token = lexer.next();
        let expr_handlers = HashMap::from([
            (TokenKind::Int, Parser::handle_int as fn(&mut Self) -> ExprNode),
            (TokenKind::Float, Parser::handle_float),
            (TokenKind::String, Parser::handle_string),
            (TokenKind::Identifier, Parser::handle_identity),
            (TokenKind::Minus, Parser::handle_minus),
            (TokenKind::LeftParen, Parser::handle_paren),
        ]);
        let identity_handlers = HashMap::from([
            (TokenKind::Dot, Parser::handle_identity_address as fn(&mut Self, &mut IdentityNode)),
            (TokenKind::LeftBracket, Parser::handle_identity_address)
        ]);
        let assignment_token_kinds = HashSet::from([
            TokenKind::Assign,
            TokenKind::PlusAssign,
            TokenKind::MinusAssign,
            TokenKind::AsteriskAssign,
            TokenKind::SlashAssign,
            TokenKind::ModuloAssign,
            TokenKind::ExponentAssign
        ]);
        let augmented_assignment_to_arithmetic = HashMap::from([
            (TokenKind::PlusAssign, TokenKind::Plus),
            (TokenKind::MinusAssign, TokenKind::Minus),
            (TokenKind::AsteriskAssign, TokenKind::Asterisk),
            (TokenKind::SlashAssign, TokenKind::Slash),
            (TokenKind::ModuloAssign, TokenKind::Modulo),
            (TokenKind::ExponentAssign, TokenKind::Exponent),
        ]);
        Self { 
            lexer,
            current_token, 
            expr_handlers, 
            identity_handlers,
            assignment_token_kinds,
            augmented_assignment_to_arithmetic
        }
    }
    
    fn eat(&mut self, expected_token_kind: TokenKind){
        if (self.current_token.kind == expected_token_kind) {
            self.current_token = self.lexer.next();
            return;
        }
        panic!("Expected token type {:?}, but got {:?}", expected_token_kind, self.current_token.kind)
    }
    
    fn handle_int(&mut self) -> ExprNode {
        let node = ExprNode::Int(self.current_token.value.parse::<i64>().unwrap());
        self.eat(TokenKind::Int);
        node
    }
    
    fn handle_float(&mut self) -> ExprNode {
        let node = ExprNode::Float(self.current_token.value.parse::<f64>().unwrap());
        self.eat(TokenKind::Float);
        node
    }
    
    fn handle_string(&mut self) -> ExprNode {
        let node = ExprNode::String(self.current_token.value.clone());
        self.eat(TokenKind::String);
        node
    }
    
    fn handle_minus(&mut self) -> ExprNode {
        self.eat(TokenKind::Minus);
        let binary = BinaryNode {
            operator: TokenKind::Asterisk,
            left: Box::new(self.expr()),
            right: Box::new(ExprNode::Int(-1))
        };
        self.eat(self.current_token.kind);
        ExprNode::Binary(binary)
    }
    
    fn handle_identity(&mut self) -> ExprNode {
        let mut identity = IdentityNode { address: vec![ExprNode::String(self.current_token.value.to_string())] };
        self.eat(self.current_token.kind);
        while let Some(handler) = self.identity_handlers.get(&self.current_token.kind){
            handler(self, &mut identity);
        }
        ExprNode::Identity(identity)
    }
    
    fn handle_identity_address(&mut self, identity: &mut IdentityNode) {
        while match self.current_token.kind {
            TokenKind::Dot => { // property access
                self.handle_identity_property(&mut identity.address);
                true
            },
            TokenKind::LeftBracket => { // index access (can be string)
                self.handle_identity_index(&mut identity.address);
                true
            },
            _ => false
        } {}
    }
    
    fn handle_identity_index(&mut self, address: &mut Vec<ExprNode>) {
        self.eat(TokenKind::LeftBracket);
        address.push(self.expr());
        self.eat(TokenKind::RightBracket);
    }
    
    fn handle_identity_property(&mut self, address: &mut Vec<ExprNode>) {
        self.eat(TokenKind::Dot);
        address.push(ExprNode::String(self.current_token.value.to_string()));
        self.eat(self.current_token.kind)
    }
    
    fn handle_func_call(&mut self, identity: &mut IdentityNode) {
        self.eat(TokenKind::LeftParen);
        let args = self.get_args();
        self.eat(TokenKind::RightParen);
        let func_call = FuncCallNode {
            identity: identity.clone(),
            args,
        };
        let new_identity = IdentityNode{ address: vec![ExprNode::FuncCall(func_call)] };
        *identity = new_identity
    }
    
    fn handle_assign(&mut self, node: ExprNode) -> ExprNode {
        let expr_node = node.clone();
        let ExprNode::Identity(identity) = node else {
            panic!("Invalid assignment type {:?}", self.current_token.kind)
        };
        let assignment_type = self.current_token.kind;
        self.eat(assignment_type);

        let mut value_node = self.expr();
        value_node =
            if assignment_type == TokenKind::Assign {
                value_node
            }
            else {
                match self.augmented_assignment_to_arithmetic.get(&assignment_type) {
                    Some(arithmetic) => {
                        let binary = BinaryNode { operator: *arithmetic, left: Box::new(expr_node), right: Box::new(value_node) };
                        ExprNode::Binary(binary)
                    },
                    _ => panic!("Invalid assignment type {:?}", assignment_type)
                }
            };
        let assign = AssignNode { identity, value: Box::new(value_node) };
        ExprNode::Assign(assign)
    }
    
    fn handle_paren(&mut self) -> ExprNode {
        self.eat(TokenKind::LeftParen);
        let expr = self.expr();
        self.eat(TokenKind::RightParen);
        expr
    }
    
    fn get_args(&mut self) -> Vec<ExprNode> {
        let mut args: Vec<ExprNode> = Vec::new();
        while self.current_token.kind == TokenKind::Comma {
            self.eat(TokenKind::Comma);
            args.push(self.expr());
        }
        args
    }
    
    fn factor(&mut self) -> ExprNode {
        if let Some(handler) = self.expr_handlers.get(&self.current_token.kind){
            return handler(self);
        }
        panic!("Unknown token {:?}", self.current_token.value);
    }
    
    fn exponent(&mut self) -> ExprNode {
        let mut node = self.factor();
        while self.current_token.kind == TokenKind::Exponent {
            let token_kind = self.current_token.kind;
            self.eat(token_kind);
            let binary = BinaryNode {
                operator: token_kind,
                left: Box::new(node),
                right: Box::new(self.factor())
            };
            node = ExprNode::Binary(binary);
        }
        node
    }
    
    fn term(&mut self) -> ExprNode {
        let mut node = self.exponent();
        while self.current_token.kind == TokenKind::Asterisk || self.current_token.kind == TokenKind::Slash {
            let token_kind = self.current_token.kind;
            self.eat(token_kind);
            let binary = BinaryNode {
                operator: token_kind,
                left: Box::new(node),
                right: Box::new(self.exponent())
            };
            node = ExprNode::Binary(binary);
        }
        node
    }
    
    fn expr(&mut self) -> ExprNode {
        let mut node = self.term();
        while self.current_token.kind == TokenKind::Plus || self.current_token.kind == TokenKind::Minus {
            let token_kind = self.current_token.kind;
            self.eat(token_kind);
            let binary = BinaryNode {
                operator: token_kind,
                left: Box::new(node),
                right: Box::new(self.term())
            };
            node = ExprNode::Binary(binary);
        }
        if self.assignment_token_kinds.contains(&self.current_token.kind){
            return self.handle_assign(node);
        }
        node
    }
    
    pub fn parse(&mut self) -> Vec<ExprNode> {
        let mut ast: Vec<ExprNode> = Vec::new();
        while self.current_token.kind != TokenKind::EOF {
            if self.current_token.kind == TokenKind::Semicolon {
                self.eat(TokenKind::Semicolon);
                continue;
            }
            ast.push(self.expr());
        }
        ast
    }
}