pub mod ast;
mod parser_test;
pub mod precedence;

use ast::{
    BlockStatement, Boolean, Expression, Ident, IfExp, InfixExp, Integer, Let, Literal, PrefixExp,
    Program, Return, Statement,
};
use precedence::{Precedence, get_token_precedence};

use crate::lexer::{
    Lexer,
    token::{Span, Token, TokenKind},
};

type ParseError = String;
type ParserErrors = Vec<ParseError>;

pub struct Parser<'s> {
    lexer: Lexer<'s>,

    current_token: Token,
    peek_token: Token,

    pub errors: ParserErrors,
}

impl<'s> Parser<'s> {
    pub fn new(mut lexer: Lexer<'s>) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Self {
            lexer,
            current_token,
            peek_token,
            errors: vec![],
        }
    }

    /// Bump andvances the cursors
    fn bump(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    /// Checks if the peeked token matches the token kind, if so bumps the cursors
    fn expect_peek(&mut self, kind: &TokenKind) -> Result<(), ParseError> {
        if self.peek_token.kind.eq(kind) {
            // If the next one matches advance the tokens
            self.bump();
            Ok(())
        } else {
            // If it did no tmatch return false and no nothing
            Err(format!(
                "Expected next token to be: {} got: {}",
                kind, self.peek_token.kind
            ))
        }
    }

    /// Returns true if the current token matches the given kind
    fn current_token_is(&self, kind: &TokenKind) -> bool {
        self.current_token.kind.eq(kind)
    }

    /// Returns true if the peek token matches the given kind
    fn peek_token_is(&self, kind: &TokenKind) -> bool {
        self.peek_token.kind.eq(kind)
    }

    /// Entry Point of the parser, starts parsing the lexer tokens and builds a program
    pub fn parse_program(&mut self) -> Result<Program, ParserErrors> {
        let mut prog = Program::new();
        // Wile not finding the EOF we keep looping
        while !self.current_token_is(&TokenKind::Eof) {
            match self.parse_statement() {
                Ok(s) => prog.statements.push(s),
                Err(e) => self.errors.push(e),
            }

            // Bump the internal tokens
            self.bump();
        }

        if self.errors.is_empty() {
            Ok(prog)
        } else {
            Err(self.errors.clone())
        }
    }

    /// Parses statements
    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        let st = match self.current_token.kind {
            TokenKind::Let => self.parse_let_statement()?,
            TokenKind::Return => self.parse_return_statement()?,
            _ => self.parse_expression_statement()?,
        };

        Ok(st)
    }

    /// Parses a LET statement and returns it
    fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        if !self.current_token_is(&TokenKind::Let) {
            return Err(format!("Expected LET found: {}", self.current_token.kind));
        }

        // get the current span start
        let start = self.current_token.span.start;

        self.bump();

        let ident = self.current_token.clone();
        // let mut _ident_name = ParseError::new();
        match &self.current_token.kind {
            TokenKind::Ident { .. } => {
                // _ident_name = name.clone();
            }
            _ => return Err(format!("Not an identifier {}", self.current_token.kind)),
        }

        self.expect_peek(&TokenKind::Assign)?;

        //TODO: Parse expressions later on
        while !self.current_token_is(&TokenKind::SemiColon) {
            // Otherwise advance the cursors
            self.bump();
        }

        // End of the statement span
        let end = self.current_token.span.end;

        Ok(Statement::Let(Let {
            identifier: ident,
            exp: None,
            span: Span { start, end },
        }))
    }

    /// Parses a return statement
    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        if !self.current_token_is(&TokenKind::Return) {
            return Err(format!(
                "Expected RETURN found: {}",
                self.current_token.kind
            ));
        }

        let start = self.current_token.span.start;

        //TODO: Here we parse the expression so in the mean time
        while !self.current_token_is(&TokenKind::SemiColon) {
            // Otherwise advance the cursors
            self.bump();
        }

        let end = self.current_token.span.end;
        Ok(Statement::Return(Return {
            exp: None,
            span: Span { start, end },
        }))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let exp = self.parse_expression(Precedence::Lowset)?;
        if self.peek_token_is(&TokenKind::SemiColon) {
            self.bump();
        }
        Ok(Statement::Expression(exp))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParseError> {
        let left_start = self.current_token.span.start;
        let mut left_exp = self.parse_prefix_expression()?;

        // While the next token is not a semicolon or the precedence is lower we iterate
        while !self.peek_token_is(&TokenKind::SemiColon)
            && precedence < get_token_precedence(&self.peek_token.kind)
        {
            let infix = self.parse_infix_expression(&left_exp, left_start);
            match infix {
                Some(infix) => {
                    // Now the left part becomes the infix for the next iteration
                    left_exp = infix?;
                }
                None => {
                    return Ok(left_exp);
                }
            }
        }

        Ok(left_exp)
    }

    /// Parses prefix expressions and returns an Expression node
    fn parse_prefix_expression(&mut self) -> Result<Expression, ParseError> {
        let span = self.current_token.clone().span;
        match &self.current_token.kind {
            TokenKind::Ident { name } => Ok(Expression::Identifier(Ident {
                name: name.clone(),
                span,
            })),
            TokenKind::Int(value) => Ok(Expression::Literal(Literal::Integer(Integer {
                value: *value,
                span,
            }))),
            TokenKind::Bang | TokenKind::Minus => {
                let operator = self.current_token.clone();

                self.bump();

                let right = self.parse_expression(Precedence::Prefix)?;

                // Calculate the span
                let start = span.start;
                let end = self.current_token.span.end;
                Ok(Expression::Prefix(PrefixExp {
                    operator,
                    right: Box::new(right),
                    span: Span { start, end },
                }))
            }
            TokenKind::True => Ok(Expression::Literal(Literal::Boolean(Boolean {
                value: true,
                span,
            }))),
            TokenKind::False => Ok(Expression::Literal(Literal::Boolean(Boolean {
                value: false,
                span,
            }))),
            TokenKind::LeftParen => {
                // Bump to the token after the (
                self.bump();

                let exp = self.parse_expression(Precedence::Lowset)?;

                // Expect the Closing parenthesis, and bump if so, otherwise return an error
                self.expect_peek(&TokenKind::RightParen)?;

                Ok(exp)
            }
            TokenKind::If => self.parse_if_expression(),
            _ => Err(format!(
                "Prefix parse expression not implemented for {}",
                self.current_token.kind
            )),
        }
    }

    /// Parses infix expressions and returns an Expression node
    fn parse_infix_expression(
        &mut self,
        left: &Expression,
        left_start: usize,
    ) -> Option<Result<Expression, ParseError>> {
        let kind = self.peek_token.kind.clone();
        match kind {
            TokenKind::Eq
            | TokenKind::NotEq
            | TokenKind::LT
            | TokenKind::GT
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash => {
                // Advance the cursors to be on top of the token
                self.bump();

                // Get the operator
                let op = self.current_token.clone();

                // Get the precedence of the infix token
                let prec = get_token_precedence(&self.current_token.kind);

                // Bump again to skip the operator since it's already part of the infix expression,
                // if we don't do so we'll try to parse an infix as a prefix and won't work
                self.bump();

                // Advance again since now we know what precedence to call the next parsing with
                let right = match self.parse_expression(prec) {
                    Err(e) => return Some(Err(e)),
                    Ok(exp) => exp,
                };

                let end = self.current_token.span.end;

                Some(Ok(Expression::Infix(InfixExp {
                    left: Box::new(left.clone()),
                    operator: op,
                    right: Box::new(right),
                    span: Span {
                        start: left_start,
                        end,
                    },
                })))
            }
            _ => None,
        }
    }

    /// Parses a if expression, the current index must be at an IF
    fn parse_if_expression(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_token.span.start;

        // Expect peek a left parenthesis
        self.expect_peek(&TokenKind::LeftParen)?;
        // Consume now the left parenthesis
        self.bump();

        // parse the condition
        let cond = self.parse_expression(Precedence::Lowset)?;

        // Expect peek a right parenthesis
        self.expect_peek(&TokenKind::RightParen)?;

        // Expect peek a left brace
        self.expect_peek(&TokenKind::LeftBrace)?;

        // Parse the block statement
        let consequence = self.parse_block_statement()?;

        // Check if there is an else condition to the if
        let alternative = if self.peek_token_is(&TokenKind::Else) {
            // Bump the else
            self.bump();
            // Expect peek {
            self.expect_peek(&TokenKind::LeftBrace)?;
            // Parse the inner else block statement
            Some(self.parse_block_statement()?)
        } else {
            None
        };

        // Get the current token
        let end = self.current_token.span.end;
        println!("{}", self.current_token);

        Ok(Expression::If(IfExp {
            cond: Box::new(cond),
            consequence,
            alternative,
            span: Span { start, end },
        }))
    }

    /// Parses a block statement, the current index must be at a { token and it will end on the
    /// matching }
    fn parse_block_statement(&mut self) -> Result<BlockStatement, ParseError> {
        let start = self.current_token.span.start;
        // Bump the {
        self.bump();

        let mut statements = vec![];
        // Loop through statements as long as we don't match a } or an EOF
        while !self.current_token_is(&TokenKind::RightBrace)
            && !self.current_token_is(&TokenKind::Eof)
        {
            if let Ok(s) = self.parse_statement() {
                statements.push(s);
            }

            // Here we bump since the statement leaves the current pointer to the last token parsed
            self.bump();
        }

        let end = self.current_token.span.end;

        Ok(BlockStatement {
            statements,
            span: Span { start, end },
        })
    }
}
