#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Identifier,
    Keyword,
    Int,
    Float,
    String,
    Boolean,
    EOF,
    
    Equals,

    Plus,
    Minus,
    Asterisk,
    Slash,
    Modulo,
    Exponent,

    Increment,
    Decrement,
    
    Assign,
    PlusAssign,
    MinusAssign,
    AsteriskAssign,
    SlashAssign,
    ModuloAssign,
    ExponentAssign,

    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    LeftBracket,
    RightBracket,

    Comma,
    Colon,
    Semicolon,
    Dot,
}

pub struct Token {
    pub kind: TokenKind,
    pub value: String
}

impl Token {
    pub fn new(kind: TokenKind, value: String) -> Token {
        Self { kind, value }
    }
}