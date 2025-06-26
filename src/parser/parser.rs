use crate::lexer::Token;
use crate::lexer::{Lexer, TokenKind};
use crate::parser::nodes::*;
use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet};
use crate::parser::nodes::ExprKind::Logical;

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
            (
                TokenKind::Int,
                Parser::handle_int as fn(&mut Self) -> ExprNode,
            ),
            (TokenKind::Float, Parser::handle_float),
            (TokenKind::String, Parser::handle_string),
            (TokenKind::True, Parser::handle_boolean),
            (TokenKind::False, Parser::handle_boolean),
            (TokenKind::Identifier, Parser::handle_identity),
            (TokenKind::Minus, Parser::handle_minus),
            (TokenKind::Ampersand, Parser::handle_ampersand),
            (TokenKind::LeftParen, Parser::handle_paren),
            (TokenKind::LeftBracket, Parser::handle_array),
            (TokenKind::LeftCurly, Parser::handle_object),
            (TokenKind::Increment, Parser::handle_unary),
            (TokenKind::Decrement, Parser::handle_unary),
        ]);
        let identity_handlers = HashMap::from([
            (
                TokenKind::Dot,
                Parser::handle_identity_address as fn(&mut Self, &mut IdentityNode),
            ),
            (TokenKind::LeftBracket, Parser::handle_identity_address),
            (TokenKind::LeftParen, Parser::handle_func_call),
        ]);
        let assignment_token_kinds = HashSet::from([
            TokenKind::Assign,
            TokenKind::PlusAssign,
            TokenKind::MinusAssign,
            TokenKind::AsteriskAssign,
            TokenKind::SlashAssign,
            TokenKind::ModuloAssign,
            TokenKind::ExponentAssign,
        ]);
        let augmented_assignment_to_arithmetic = HashMap::from([
            (TokenKind::PlusAssign, TokenKind::Plus),
            (TokenKind::MinusAssign, TokenKind::Minus),
            (TokenKind::AsteriskAssign, TokenKind::Asterisk),
            (TokenKind::SlashAssign, TokenKind::Slash),
            (TokenKind::ModuloAssign, TokenKind::Modulo),
            (TokenKind::ExponentAssign, TokenKind::Exponent),
            (TokenKind::Increment, TokenKind::Plus),
            (TokenKind::Decrement, TokenKind::Minus),
        ]);
        Self {
            lexer,
            current_token,
            expr_handlers,
            identity_handlers,
            assignment_token_kinds,
            augmented_assignment_to_arithmetic,
        }
    }

    fn eat(&mut self, expected_token_kind: TokenKind) {
        if (self.current_token.kind == expected_token_kind) {
            self.current_token = self.lexer.next();
            return;
        }
        panic!(
            "Expected token type {:?}, but got {:?}",
            expected_token_kind, self.current_token.kind
        )
    }

    fn handle_int(&mut self) -> ExprNode {
        let node = ExprNode::Int(self.current_token.value.parse::<i64>().unwrap());
        self.eat(TokenKind::Int);
        node
    }

    fn handle_float(&mut self) -> ExprNode {
        let node = ExprNode::Float(OrderedFloat(
            self.current_token.value.parse::<f64>().unwrap(),
        ));
        self.eat(TokenKind::Float);
        node
    }

    fn handle_string(&mut self) -> ExprNode {
        let node = ExprNode::String(self.current_token.value.clone());
        self.eat(TokenKind::String);
        node
    }

    fn handle_boolean(&mut self) -> ExprNode {
        match self.current_token.kind {
            TokenKind::True => {
                self.eat(TokenKind::True);
                ExprNode::Bool(true)
            }
            TokenKind::False => {
                self.eat(TokenKind::False);
                ExprNode::Bool(false)
            }
            _ => panic!(
                "Expected a boolean literal, got {:?}",
                self.current_token.kind
            ),
        }
    }

    fn handle_minus(&mut self) -> ExprNode {
        self.eat(TokenKind::Minus);
        let binary = BinaryNode {
            operator: TokenKind::Asterisk,
            left: Box::new(self.expr()),
            right: Box::new(ExprNode::Int(-1)),
        };
        self.eat(self.current_token.kind);
        ExprNode::Binary(binary)
    }

    fn handle_ampersand(&mut self) -> ExprNode {
        self.eat(TokenKind::Ampersand);
        let identity_node = match self.current_token.kind {
            TokenKind::Identifier => self.expr(),
            _ => panic!("Cannot reference non identifier"),
        };
        match identity_node {
            ExprNode::Identity(identity) => ExprNode::Reference(ReferenceNode::new(identity)),
            _ => unreachable!(),
        }
    }

    fn handle_identity(&mut self) -> ExprNode {
        let mut identity = IdentityNode {
            address: vec![ExprNode::String(self.current_token.value.to_string())],
        };
        self.eat(self.current_token.kind);
        while let Some(handler) = self.identity_handlers.get(&self.current_token.kind) {
            handler(self, &mut identity);
        }
        if matches!(
            self.current_token.kind,
            TokenKind::Increment | TokenKind::Decrement
        ) {
            let token_kind = self.current_token.kind;
            self.eat(token_kind);
            return ExprNode::Assign(AssignNode {
                identity: identity.clone(),
                value: Box::new(ExprNode::Binary(BinaryNode {
                    operator: self
                        .augmented_assignment_to_arithmetic
                        .get(&token_kind)
                        .unwrap()
                        .clone(),
                    left: Box::new(ExprNode::Identity(identity)),
                    right: Box::new(ExprNode::Int(1)),
                })),
                return_after: true,
            });
        }
        ExprNode::Identity(identity)
    }

    fn handle_identity_address(&mut self, identity: &mut IdentityNode) {
        while match self.current_token.kind {
            TokenKind::Dot => {
                // property access
                self.handle_identity_property(&mut identity.address);
                true
            }
            TokenKind::LeftBracket => {
                // index access (can be string)
                self.handle_identity_index(&mut identity.address);
                true
            }
            _ => false,
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
        let args = self.get_args(TokenKind::RightParen);
        self.eat(TokenKind::RightParen);
        identity.address = vec![ExprNode::FuncCall(FuncCallNode {
            identity: identity.clone(),
            args,
        })];
    }

    fn handle_object(&mut self) -> ExprNode {
        let mut object = ObjectNode {
            properties: Vec::new(),
        };
        self.eat(TokenKind::LeftCurly);
        while self.current_token.kind != TokenKind::RightCurly {
            let key = self.expr();
            self.eat(TokenKind::Colon);
            object.properties.push(ObjectProperty {
                key,
                value: self.expr(),
            });
            if self.current_token.kind == TokenKind::Comma {
                self.eat(TokenKind::Comma);
            }
        }
        self.eat(TokenKind::RightCurly);
        ExprNode::Object(object)
    }

    fn handle_array(&mut self) -> ExprNode {
        self.eat(TokenKind::LeftBracket);
        let args_list = ListNode {
            elements: self.get_args(TokenKind::RightBracket),
        };
        self.eat(TokenKind::RightBracket);
        ExprNode::List(args_list)
    }

    fn handle_unary(&mut self) -> ExprNode {
        let token_kind = self.current_token.kind;
        self.eat(token_kind);
        let node = self.expr();
        if let ExprNode::Identity(identity_node) = node {
            return ExprNode::Assign(AssignNode {
                identity: identity_node.clone(),
                value: Box::new(ExprNode::Binary(BinaryNode {
                    operator: token_kind,
                    left: Box::new(ExprNode::Identity(identity_node)),
                    right: Box::new(ExprNode::Int(1)),
                })),
                return_after: true,
            });
        }
        panic!("Unary operator can only be applied to an identifier")
    }

    fn handle_assign(&mut self, node: ExprNode) -> ExprNode {
        let expr_node = node.clone();
        let ExprNode::Identity(identity) = node else {
            panic!("Invalid assignment type {:?}", self.current_token.kind)
        };
        let assignment_type = self.current_token.kind;
        self.eat(assignment_type);

        let mut value_node = self.expr();
        value_node = if assignment_type == TokenKind::Assign {
            value_node
        } else {
            match self
                .augmented_assignment_to_arithmetic
                .get(&assignment_type)
            {
                Some(arithmetic) => {
                    let binary = BinaryNode {
                        operator: *arithmetic,
                        left: Box::new(expr_node),
                        right: Box::new(value_node),
                    };
                    ExprNode::Binary(binary)
                }
                _ => panic!("Invalid assignment type {:?}", assignment_type),
            }
        };
        let assign = AssignNode {
            identity,
            value: Box::new(value_node),
            return_after: true,
        };
        ExprNode::Assign(assign)
    }

    fn handle_paren(&mut self) -> ExprNode {
        self.eat(TokenKind::LeftParen);
        let expr = self.expr();
        self.eat(TokenKind::RightParen);
        expr
    }

    fn get_args(&mut self, closing: TokenKind) -> Vec<ExprNode> {
        let mut args: Vec<ExprNode> = Vec::new();
        if self.current_token.kind != closing {
            args.push(self.expr());
            while self.current_token.kind == TokenKind::Comma {
                self.eat(TokenKind::Comma);
                args.push(self.expr());
            }
        }
        args
    }

    fn factor(&mut self) -> ExprNode {
        if let Some(handler) = self.expr_handlers.get(&self.current_token.kind) {
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
                right: Box::new(self.factor()),
            };
            node = ExprNode::Binary(binary);
        }
        node
    }

    fn term(&mut self) -> ExprNode {
        let mut node = self.exponent();
        while self.current_token.kind == TokenKind::Asterisk
            || self.current_token.kind == TokenKind::Slash
            || self.current_token.kind == TokenKind::Modulo
        {
            let token_kind = self.current_token.kind;
            self.eat(token_kind);
            let binary = BinaryNode {
                operator: token_kind,
                left: Box::new(node),
                right: Box::new(self.exponent()),
            };
            node = ExprNode::Binary(binary);
        }
        node
    }

    fn add_sub(&mut self) -> ExprNode {
        let mut node = self.term();
        while self.current_token.kind == TokenKind::Plus
            || self.current_token.kind == TokenKind::Minus
        {
            let token_kind = self.current_token.kind;
            self.eat(token_kind);
            let binary = BinaryNode {
                operator: token_kind,
                left: Box::new(node),
                right: Box::new(self.term()),
            };
            node = ExprNode::Binary(binary);
        }
        node
    }

    fn comparison(&mut self) -> ExprNode {
        let mut node = self.add_sub();
        while matches!(self.current_token.kind, TokenKind::GreaterThan | TokenKind::GreaterThanEquals | TokenKind::LessThan | TokenKind::LessThanEquals) {
            let token_kind = self.current_token.kind;
            self.eat(token_kind);
            let comparison = ComparisonNode {
                operator: token_kind,
                left: Box::new(node),
                right: Box::new(self.add_sub()),
            };
            node = ExprNode::Comparison(comparison);
        }
        node
    }

    fn equality(&mut self) -> ExprNode {
        let mut node = self.comparison();
        while matches!(self.current_token.kind, TokenKind::Equals | TokenKind::NotEquals) {
            let token_kind = self.current_token.kind;
            self.eat(token_kind);
            let equality = ComparisonNode {
                operator: token_kind,
                left: Box::new(node),
                right: Box::new(self.comparison()),
            };
            node = ExprNode::Comparison(equality);
        }
        node
    }
    
    fn logical(&mut self) -> ExprNode {
        let mut node = self.equality();
        while matches!(self.current_token.kind, TokenKind::LogicalAND | TokenKind::LogicalOR) {
            let token_kind = self.current_token.kind;
            self.eat(token_kind);
            let logical = LogicalNode {
                operator: token_kind,
                left: Box::new(node),
                right: Box::new(self.equality()),
            };
            node = ExprNode::Logical(logical);
        }
        node
    }
    
    fn assign(&mut self) -> ExprNode {
        let mut node = self.logical();
        if self
            .assignment_token_kinds
            .contains(&self.current_token.kind)
        {
            return self.handle_assign(node);
        }
        node
    }

    fn expr(&mut self) -> ExprNode {
        self.assign()
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
