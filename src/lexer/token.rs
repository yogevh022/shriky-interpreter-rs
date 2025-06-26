#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Identifier,
    Int,
    Float,
    String,
    EOF,

    Equals,
    NotEquals,
    
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,

    LogicalAND,
    LogicalOR,
    LogicalNOT,

    Plus,
    Minus,
    Asterisk,
    Slash,
    DoubleSlash,
    Modulo,
    Exponent,

    Increment,
    Decrement,

    Assign,
    PlusAssign,
    MinusAssign,
    AsteriskAssign,
    SlashAssign,
    DoubleSlashAssign,
    ModuloAssign,
    ExponentAssign,

    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    LeftBracket,
    RightBracket,

    If,
    Else,
    True,
    False,
    While,
    Break,
    Continue,
    Fn,
    Return,
    Null,

    Comma,
    Colon,
    Semicolon,
    Dot,
    Ampersand,
}

pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

impl Token {
    pub fn new(kind: TokenKind, value: String) -> Token {
        Self { kind, value }
    }
}
