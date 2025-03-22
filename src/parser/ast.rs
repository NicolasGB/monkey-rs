use std::fmt::Display;

use crate::lexer::token::{Span, Token};
pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Program(program) => write!(f, "{}", program),
            Node::Statement(stmt) => todo!(),
            Node::Expression => todo!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self { statements: vec![] }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.statements.first())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Let(Let),
    Return(Return),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Let {
    pub identifier: Token,
    pub exp: Option<Expression>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Return {
    pub exp: Option<Expression>,
    pub span: Span,
}

impl Statement {}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {}

impl Expression {}
