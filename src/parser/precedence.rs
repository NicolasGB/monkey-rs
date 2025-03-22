#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowset,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}
