use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "start: {} end: {}, kind: {}",
            self.span.start, self.span.end, self.kind
        )
    }
    // add code here
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Self {
            kind,
            span: Span { start, end },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Eof,
    Illegal,

    // Identifiers and literals
    Ident { name: String },
    Int(i64),
    String(String),

    // Operators
    Assign, // -
    Plus,   // +
    Bang,   // !
    Minus,  // -
    Slash,  // /
    Star,   // *

    LT, // <
    GT, // >

    Eq,    // ==
    NotEq, // !=

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    SemiColon,

    //Keyword
    Let,
    Fn,
    True,
    False,
    If,
    Else,
    Return,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Eof => write!(f, "EOF"),
            TokenKind::Illegal => write!(f, "ILLEGAL"),
            TokenKind::Assign => write!(f, "="),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::SemiColon => write!(f, ";"),
            TokenKind::Ident { name } => write!(f, "{name}"),
            TokenKind::Int(i) => write!(f, "{i}"),
            TokenKind::String(s) => write!(f, "{s}"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::Fn => write!(f, "fn"),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::LT => write!(f, "<"),
            TokenKind::GT => write!(f, ">"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Eq => write!(f, "=="),
            TokenKind::NotEq => write!(f, "!="),
        }
    }
}

pub fn lookup_identifier(identifier: &str) -> TokenKind {
    match identifier {
        "let" => TokenKind::Let,
        "fn" => TokenKind::Fn,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "return" => TokenKind::Return,
        _ => TokenKind::Ident {
            name: identifier.to_string(),
        },
    }
}
