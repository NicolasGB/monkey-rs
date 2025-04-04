//TODO: remove when done
#![allow(unused)]

use std::fmt::Display;

use crate::lexer::token::{Span, Token, TokenKind};
pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Program(program) => write!(f, "{program}"),
            Node::Statement(stmt) => write!(f, "{stmt}"),
            Node::Expression(exp) => write!(f, "{exp}"),
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
        let statements = self
            .statements
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("");

        write!(f, "{statements}")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Let(Let),
    Return(Return),
    Expression(Expression),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(Let {
                identifier, exp, ..
            }) => {
                if let TokenKind::Ident { name } = &identifier.kind {
                    return write!(f, "let {name} = {exp:?};");
                }
                unreachable!("Cannot have let without identifier")
            }
            Statement::Return(Return { exp, .. }) => write!(f, "return {exp:?};"),
            Statement::Expression(exp) => write!(f, "{exp}"),
        }
    }
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

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Identifier(Ident),
    Literal(Literal),
    Prefix(PrefixExp),
    Infix(InfixExp),
    If(IfExp),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(ident) => write!(f, "{}", ident),
            Expression::Literal(lit) => write!(f, "{}", lit),
            Expression::Prefix(PrefixExp {
                operator, right, ..
            }) => write!(f, "({}{})", operator.kind, right),
            Expression::Infix(InfixExp {
                left,
                operator,
                right,
                ..
            }) => write!(f, "({} {} {})", left, operator.kind, right),
            Expression::If(IfExp {
                cond,
                consequence,
                alternative,
                ..
            }) => match alternative {
                Some(block) => write!(f, "if {} {{ {} }} else {{ {} }}", cond, consequence, block),
                None => write!(f, "if {} {{ {} }}", cond, consequence),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Integer(Integer),
    Boolean(Boolean),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Integer(integer) => write!(f, "{}", integer.value),
            Literal::Boolean(boolean) => write!(f, "{}", boolean.value),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Integer {
    pub value: i64,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Boolean {
    pub value: bool,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrefixExp {
    pub operator: Token,
    pub right: Box<Expression>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InfixExp {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfExp {
    pub cond: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
    pub span: Span,
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .statements
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("");

        write!(f, "{s}")
    }
}
