#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowset,      //
    Equals,      // ==
    LessGreater, // < or >
    Sum,         // +
    Product,     // =
    Prefix,      // !X or -X
    Call,        // function()
}
