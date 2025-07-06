use crate::lexer::token;
use std::collections::HashMap;

pub struct Lexer<'a> {
    chars: std::str::Chars<'a>,
    current_char: Option<char>,
    reserved_keywords: HashMap<&'static str, token::TokenKind>,
    single_char_tokens: HashMap<char, token::TokenKind>,
    special_token_handlers: HashMap<char, fn(&mut Self) -> token::Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut chars = source.chars();
        let current_char = chars.next();
        let reserved_keywords = HashMap::from([
            ("if", token::TokenKind::If),
            ("else", token::TokenKind::Else),
            ("true", token::TokenKind::True),
            ("false", token::TokenKind::False),
            ("while", token::TokenKind::While),
            ("break", token::TokenKind::Break),
            ("continue", token::TokenKind::Continue),
            ("fn", token::TokenKind::Function),
            ("class", token::TokenKind::Class),
            ("return", token::TokenKind::Return),
            ("con", token::TokenKind::Constructor),
            ("and", token::TokenKind::LogicalAND),
            ("or", token::TokenKind::LogicalOR),
            ("null", token::TokenKind::Null),
        ]);
        let single_char_tokens = HashMap::from([
            ('(', token::TokenKind::LeftParen),
            (')', token::TokenKind::RightParen),
            ('[', token::TokenKind::LeftBracket),
            (']', token::TokenKind::RightBracket),
            ('{', token::TokenKind::LeftCurly),
            ('}', token::TokenKind::RightCurly),
            (':', token::TokenKind::Colon),
            (';', token::TokenKind::Semicolon),
            (',', token::TokenKind::Comma),
            ('.', token::TokenKind::Dot),
        ]);
        let special_tokenizers = HashMap::from([
            ('+', Lexer::plus_token as fn(&mut Self) -> token::Token), // implies cast on all v
            ('-', Lexer::minus_token),
            ('*', Lexer::asterisk_token),
            ('/', Lexer::slash_token),
            ('%', Lexer::modulo_token),
            ('=', Lexer::equal_token),
            ('&', Lexer::ampersand_token),
            ('!', Lexer::exclamation_token),
            ('>', Lexer::greater_than_token),
            ('<', Lexer::less_than_token),
        ]);
        Self {
            chars,
            current_char,
            reserved_keywords,
            single_char_tokens,
            special_token_handlers: special_tokenizers,
        }
    }

    pub fn advance(&mut self) {
        self.current_char = self.chars.next();
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.map_or(false, |c| c.is_whitespace()) {
            self.advance();
        }
    }

    fn string_token(&mut self, opening_quote: char) -> token::Token {
        let mut result = String::new();
        self.advance();
        while let Some(c) = self.current_char {
            if c == opening_quote {
                break;
            }
            if c == '\\' {
                self.advance();
                if let Some(c) = self.current_char {
                    match c {
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        _ => result.push(c),
                    }
                }
            } else {
                result.push(c);
            }
            self.advance();
            if self.current_char.is_none() {
                panic!("Unterminated string literal");
            }
        }
        self.advance();
        token::Token::new(token::TokenKind::String, result)
    }

    fn number_token(&mut self) -> token::Token {
        let mut result = String::new();
        let mut is_float = false;
        while let Some(c) = self.current_char {
            if !(c.is_digit(10) || (!is_float && c == '.')) {
                break;
            }
            if c == '.' {
                is_float = true;
            }
            result.push(c);
            self.advance();
        }
        token::Token::new(
            if is_float {
                token::TokenKind::Float
            } else {
                token::TokenKind::Int
            },
            result,
        )
    }

    fn keyword_token(&mut self) -> token::Token {
        let mut result = String::new();
        while let Some(c) = self.current_char {
            if !(c.is_alphanumeric() || c == '_') {
                break;
            }
            result.push(c);
            self.advance();
        }
        if let Some(keyword) = self.reserved_keywords.get(&result.as_str()) {
            return token::Token::new(*keyword, result);
        }
        token::Token::new(token::TokenKind::Identifier, result)
    }

    fn plus_token(&mut self) -> token::Token {
        self.advance();
        match self.current_char {
            Some('+') => {
                self.advance();
                token::Token::new(token::TokenKind::Increment, "++".to_string())
            }
            Some('=') => {
                self.advance();
                token::Token::new(token::TokenKind::PlusAssign, "+=".to_string())
            }
            _ => token::Token::new(token::TokenKind::Plus, "+".to_string()),
        }
    }

    fn minus_token(&mut self) -> token::Token {
        self.advance();
        match self.current_char {
            Some('-') => {
                self.advance();
                token::Token::new(token::TokenKind::Decrement, "--".to_string())
            }
            Some('=') => {
                self.advance();
                token::Token::new(token::TokenKind::MinusAssign, "-=".to_string())
            }
            _ => token::Token::new(token::TokenKind::Minus, "-".to_string()),
        }
    }

    fn asterisk_token(&mut self) -> token::Token {
        self.advance();
        match self.current_char {
            Some('*') => {
                self.advance();
                if self.current_char.map_or(false, |c| c == '=') {
                    self.advance();
                    return token::Token::new(token::TokenKind::ExponentAssign, "**=".to_string());
                }
                token::Token::new(token::TokenKind::Exponent, "**".to_string())
            }
            Some('=') => {
                self.advance();
                token::Token::new(token::TokenKind::AsteriskAssign, "*=".to_string())
            }
            _ => token::Token::new(token::TokenKind::Asterisk, "*".to_string()),
        }
    }

    fn slash_token(&mut self) -> token::Token {
        self.advance();
        match self.current_char {
            Some('=') => {
                self.advance();
                token::Token::new(token::TokenKind::SlashAssign, "/=".to_string())
            }
            Some('/') => {
                self.advance();
                match self.current_char {
                    Some('=') => {
                        self.advance();
                        token::Token::new(token::TokenKind::DoubleSlashAssign, "//=".to_string())
                    }
                    _ => token::Token::new(token::TokenKind::DoubleSlash, "//".to_string()),
                }
            }
            _ => token::Token::new(token::TokenKind::Slash, "/".to_string()),
        }
    }

    fn modulo_token(&mut self) -> token::Token {
        self.advance();
        match self.current_char {
            Some('=') => {
                self.advance();
                token::Token::new(token::TokenKind::ModuloAssign, "%=".to_string())
            }
            _ => token::Token::new(token::TokenKind::Modulo, "%".to_string()),
        }
    }

    fn equal_token(&mut self) -> token::Token {
        self.advance();
        match self.current_char {
            Some('=') => {
                self.advance();
                token::Token::new(token::TokenKind::Equals, "==".to_string())
            }
            _ => token::Token::new(token::TokenKind::Assign, "=".to_string()),
        }
    }

    fn ampersand_token(&mut self) -> token::Token {
        self.advance();
        match self.current_char {
            Some('&') => {
                self.advance();
                token::Token::new(token::TokenKind::LogicalAND, "&&".to_string())
            }
            _ => token::Token::new(token::TokenKind::Ampersand, "&".to_string()),
        }
    }

    fn exclamation_token(&mut self) -> token::Token {
        self.advance();
        match self.current_char {
            Some('=') => {
                self.advance();
                token::Token::new(token::TokenKind::NotEquals, "!=".to_string())
            }
            _ => token::Token::new(token::TokenKind::LogicalNOT, "!".to_string()),
        }
    }

    fn greater_than_token(&mut self) -> token::Token {
        self.advance();
        match self.current_char {
            Some('=') => {
                self.advance();
                token::Token::new(token::TokenKind::GreaterThanEquals, ">=".to_string())
            }
            _ => token::Token::new(token::TokenKind::GreaterThan, ">".to_string()),
        }
    }

    fn less_than_token(&mut self) -> token::Token {
        self.advance();
        match self.current_char {
            Some('=') => {
                self.advance();
                token::Token::new(token::TokenKind::LessThanEquals, "<=".to_string())
            }
            _ => token::Token::new(token::TokenKind::LessThan, "<".to_string()),
        }
    }

    pub fn next(&mut self) -> token::Token {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.skip_whitespace();
                continue;
            }
            if c.is_digit(10) {
                return self.number_token();
            }
            if c.is_alphanumeric() || c == '_' {
                return self.keyword_token();
            }
            if c == '\"' || c == '\'' {
                return self.string_token(c);
            }
            if let Some(token_kind) = self.single_char_tokens.get(&c) {
                let tok = token::Token::new(*token_kind, c.to_string());
                self.advance();
                return tok;
            }
            if let Some(handler) = self.special_token_handlers.get(&c) {
                return handler(self);
            }
            panic!("Unrecognized character: {}", c);
        }
        token::Token::new(token::TokenKind::EOF, String::new())
    }
}
