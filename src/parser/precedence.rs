use crate::lexer::token::TokenKind;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowset,      //
    Equals,      // ==
    LessGreater, // < or >
    Sum,         // +
    Product,     // *
    Prefix,      // !X or -X
    Call,        // function()
}

pub fn get_token_precedence(kind: &TokenKind) -> Precedence {
    match kind {
        TokenKind::Eq => Precedence::Equals,
        TokenKind::NotEq => Precedence::Equals,
        TokenKind::LT => Precedence::LessGreater,
        TokenKind::GT => Precedence::LessGreater,
        TokenKind::Plus => Precedence::Sum,
        TokenKind::Minus => Precedence::Sum,
        TokenKind::Star => Precedence::Product,
        TokenKind::Slash => Precedence::Product,
        _ => Precedence::Lowset,
    }
}
