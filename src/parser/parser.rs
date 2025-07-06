use crate::lexer::Token;
use crate::lexer::{Lexer, TokenKind};
use crate::parser::nodes::ExprKind::Logical;
use crate::parser::nodes::*;
use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet};
use std::mem;

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
    expr_handlers: HashMap<TokenKind, for<'b> fn(&'b mut Parser<'a>) -> ExprNode>,
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
            (TokenKind::Function, Parser::handle_function),
            (TokenKind::Class, Parser::handle_class),
            (TokenKind::Constructor, Parser::handle_constructor),
            (TokenKind::If, Parser::handle_if),
            (TokenKind::While, Parser::handle_while),
            (TokenKind::Return, Parser::handle_return),
            (TokenKind::LeftParen, Parser::handle_paren),
            (TokenKind::LeftBracket, Parser::handle_list),
            (TokenKind::LeftCurly, Parser::handle_map),
            (TokenKind::Increment, Parser::handle_increment_decrement_pre),
            (TokenKind::Decrement, Parser::handle_increment_decrement_pre),
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
        let node = ExprNode::int(self.current_token.value.parse::<i64>().unwrap());
        self.eat(TokenKind::Int);
        node
    }

    fn handle_float(&mut self) -> ExprNode {
        let node = ExprNode::float(self.current_token.value.parse::<f64>().unwrap());
        self.eat(TokenKind::Float);
        node
    }

    fn handle_string(&mut self) -> ExprNode {
        let node = ExprNode::string(self.current_token.value.clone());
        self.eat(TokenKind::String);
        node
    }

    fn handle_boolean(&mut self) -> ExprNode {
        match self.current_token.kind {
            TokenKind::True => {
                self.eat(TokenKind::True);
                ExprNode::bool(true)
            }
            TokenKind::False => {
                self.eat(TokenKind::False);
                ExprNode::bool(false)
            }
            _ => panic!(
                "Expected a boolean literal, got {:?}",
                self.current_token.kind
            ),
        }
    }

    fn handle_minus(&mut self) -> ExprNode {
        self.eat(TokenKind::Minus);
        ExprNode::binary(TokenKind::Asterisk, self.expr(), ExprNode::int(-1))
    }

    fn handle_ampersand(&mut self) -> ExprNode {
        self.eat(TokenKind::Ampersand);
        let maybe_identity_node = match self.current_token.kind {
            TokenKind::Identifier => self.expr(),
            _ => panic!("Cannot reference non identifier"),
        };
        let ExprNode::Identity(identity_node) = maybe_identity_node else {
            unreachable!(
                "Identity handlers returned {:?} instead of Identity",
                maybe_identity_node
            )
        };
        ExprNode::reference(identity_node)
    }

    fn eat_current_token_string(&mut self) -> ExprNode {
        let token_string = ExprNode::string(self.current_token.value.clone());
        self.eat(TokenKind::Identifier);
        token_string
    }

    fn handle_identity(&mut self) -> ExprNode {
        let mut address = vec![self.eat_current_token_string()];
        while matches!(
            self.current_token.kind,
            TokenKind::Dot | TokenKind::LeftBracket | TokenKind::LeftParen
        ) {
            match self.current_token.kind {
                TokenKind::Dot => self.push_address_access_attribute(&mut address),
                TokenKind::LeftBracket => self.push_address_access_constant(&mut address),
                TokenKind::LeftParen => self.push_address_func_call(&mut address),
                _ => unreachable!(),
            };
        }
        let identity = ExprNode::identity(address);
        if matches!(
            self.current_token.kind,
            TokenKind::Increment | TokenKind::Decrement
        ) {
            return self.handle_increment_decrement_post(identity);
        }
        identity
    }

    fn push_address_access_attribute(&mut self, address: &mut Vec<ExprNode>) {
        self.eat(TokenKind::Dot);
        let accessed_attr = self.eat_current_token_string();
        address.push(ExprNode::access_attribute(accessed_attr));
    }

    fn push_address_access_constant(&mut self, address: &mut Vec<ExprNode>) {
        self.eat(TokenKind::LeftBracket);
        let expr = self.expr();
        self.eat(TokenKind::RightBracket);
        address.push(ExprNode::access_constant(expr));
    }

    fn push_address_func_call(&mut self, address: &mut Vec<ExprNode>) {
        self.eat(TokenKind::LeftParen);
        let args = self.get_args(TokenKind::RightParen);
        self.eat(TokenKind::RightParen);
        let func_call_identity = IdentityNode::new(mem::take(address));
        *address = vec![ExprNode::call(func_call_identity, args)]
    }

    fn handle_map(&mut self) -> ExprNode {
        let mut map_properties: Vec<MapProperty> = Vec::new();
        self.eat(TokenKind::LeftCurly);
        while self.current_token.kind != TokenKind::RightCurly {
            let key = self.expr();
            self.eat(TokenKind::Colon);
            map_properties.push(MapProperty {
                key,
                value: self.expr(),
            });
            if self.current_token.kind == TokenKind::Comma {
                self.eat(TokenKind::Comma);
            }
        }
        self.eat(TokenKind::RightCurly);
        ExprNode::map(map_properties)
    }

    fn handle_list(&mut self) -> ExprNode {
        self.eat(TokenKind::LeftBracket);
        let elements = self.get_args(TokenKind::RightBracket);
        self.eat(TokenKind::RightBracket);
        ExprNode::list(elements)
    }

    fn handle_increment_decrement_pre(&mut self) -> ExprNode {
        let token_kind = self.current_token.kind;
        self.eat(token_kind);
        let maybe_identity_expr = self.expr();
        if let ExprNode::Identity(identity_node) = maybe_identity_expr.clone() {
            let binary = ExprNode::binary(token_kind, maybe_identity_expr, ExprNode::int(1));
            return ExprNode::assign(identity_node, binary, true);
        }
        panic!("Increment / Decrement operation can only be applied to identities.")
    }

    fn handle_increment_decrement_post(&mut self, identity_expr: ExprNode) -> ExprNode {
        let token_kind = self.current_token.kind;
        self.eat(token_kind);
        if let ExprNode::Identity(identity_node) = identity_expr.clone() {
            let binary = ExprNode::binary(token_kind, identity_expr, ExprNode::int(1));
            return ExprNode::assign(identity_node, binary, false);
        }
        panic!("Increment / Decrement operation can only be applied to identities.")
    }

    fn parse_function_definition_args(&mut self) -> Vec<String> {
        self.get_args(TokenKind::RightParen)
            .into_iter()
            .map(|arg_expr| match arg_expr {
                ExprNode::Identity(arg) if arg.address.len() == 1 => match arg.address.first() {
                    Some(ExprNode::String(string_arg)) => string_arg.value.clone(),
                    _ => panic!("Function argument identity must be a string"),
                },
                _ => panic!("Function argument must be a simple identity"),
            })
            .collect()
    }

    fn handle_anonymous_function(&mut self) -> ExprNode {
        self.eat(TokenKind::LeftParen);
        let string_args = self.parse_function_definition_args();
        self.eat(TokenKind::RightParen);

        self.eat(TokenKind::LeftCurly);
        let func_node = ExprNode::function(string_args, self.parse(TokenKind::RightCurly));
        self.eat(TokenKind::RightCurly);
        func_node
    }

    fn handle_constructor(&mut self) -> ExprNode {
        let func_name = ExprNode::string(self.current_token.value.clone());
        self.eat(TokenKind::Constructor);
        let func_node = self.handle_anonymous_function();

        ExprNode::assign(IdentityNode::new(vec![func_name]), func_node, true)
    }

    fn handle_function(&mut self) -> ExprNode {
        self.eat(TokenKind::Function);
        let func_name = self.eat_current_token_string();
        let func_node = self.handle_anonymous_function();

        ExprNode::assign(IdentityNode::new(vec![func_name]), func_node, true)
    }

    fn handle_class(&mut self) -> ExprNode {
        self.eat(TokenKind::Class);

        let class_name = self.eat_current_token_string();

        let superclass = if self.current_token.kind == TokenKind::LeftParen {
            self.eat(TokenKind::LeftParen);
            let superclass = self.expr();
            self.eat(TokenKind::RightParen);
            Some(ExprNode::identity(vec![superclass]))
        } else {
            None
        };

        self.eat(TokenKind::LeftCurly);
        let class_body = self.parse(TokenKind::RightCurly);
        self.eat(TokenKind::RightCurly);

        ExprNode::assign(
            IdentityNode::new(vec![class_name]),
            ExprNode::class(superclass, class_body),
            true,
        )
    }

    fn handle_return(&mut self) -> ExprNode {
        self.eat(TokenKind::Return);
        let return_value = if self.current_token.kind == TokenKind::Semicolon {
            ExprNode::null()
        } else {
            self.expr()
        };
        ExprNode::return_n(return_value)
    }

    fn handle_if(&mut self) -> ExprNode {
        self.eat(TokenKind::If);
        let condition = self.expr();

        self.eat(TokenKind::LeftCurly);
        let body = self.parse(TokenKind::RightCurly);
        self.eat(TokenKind::RightCurly);

        let else_body = if self.current_token.kind == TokenKind::Else {
            self.eat(TokenKind::Else);
            match self.current_token.kind {
                TokenKind::If => vec![self.handle_if()],
                TokenKind::LeftCurly => {
                    self.eat(TokenKind::LeftCurly);
                    let body = self.parse(TokenKind::RightCurly);
                    self.eat(TokenKind::RightCurly);
                    body
                }
                _ => panic!(
                    "Unexpected token after else statement: {:?}",
                    self.current_token.kind
                ),
            }
        } else {
            Vec::new()
        };

        ExprNode::if_n(condition, body, else_body)
    }

    fn handle_while(&mut self) -> ExprNode {
        self.eat(TokenKind::While);
        let condition = self.expr();
        self.eat(TokenKind::LeftCurly);
        let body = self.parse(TokenKind::RightCurly);
        self.eat(TokenKind::RightCurly);
        ExprNode::while_n(condition, body)
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
                Some(arithmetic) => ExprNode::binary(*arithmetic, expr_node, value_node),
                _ => panic!("Invalid assignment type {:?}", assignment_type),
            }
        };
        ExprNode::assign(identity, value_node, true)
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
            node = ExprNode::binary(token_kind, node, self.factor());
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

            node = ExprNode::binary(token_kind, node, self.exponent());
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
            node = ExprNode::binary(token_kind, node, self.term());
        }
        node
    }

    fn comparison(&mut self) -> ExprNode {
        let mut node = self.add_sub();
        while matches!(
            self.current_token.kind,
            TokenKind::GreaterThan
                | TokenKind::GreaterThanEquals
                | TokenKind::LessThan
                | TokenKind::LessThanEquals
        ) {
            let token_kind = self.current_token.kind;
            self.eat(token_kind);
            node = ExprNode::comparison(token_kind, node, self.add_sub());
        }
        node
    }

    fn equality(&mut self) -> ExprNode {
        let mut node = self.comparison();
        while matches!(
            self.current_token.kind,
            TokenKind::Equals | TokenKind::NotEquals
        ) {
            let token_kind = self.current_token.kind;
            self.eat(token_kind);
            node = ExprNode::comparison(token_kind, node, self.comparison());
        }
        node
    }

    fn logical_and(&mut self) -> ExprNode {
        let mut node = self.equality();
        while self.current_token.kind == TokenKind::LogicalAND {
            self.eat(self.current_token.kind);
            node = ExprNode::logical(TokenKind::LogicalAND, node, self.equality())
        }
        node
    }

    fn logical_or(&mut self) -> ExprNode {
        let mut node = self.logical_and();
        while self.current_token.kind == TokenKind::LogicalOR {
            self.eat(self.current_token.kind);
            node = ExprNode::logical(TokenKind::LogicalOR, node, self.equality())
        }
        node
    }

    fn assign(&mut self) -> ExprNode {
        let mut node = self.logical_or();
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

    pub fn parse(&mut self, closing_token: TokenKind) -> Vec<ExprNode> {
        let mut ast: Vec<ExprNode> = Vec::new();
        while self.current_token.kind != closing_token {
            if self.current_token.kind == TokenKind::Semicolon {
                self.eat(TokenKind::Semicolon);
                continue;
            }
            ast.push(self.expr());
        }
        ast
    }
}
