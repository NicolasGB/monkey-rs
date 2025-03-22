pub mod ast;
mod parser_test;

use ast::{Let, Program, Return, Statement};

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
                kind, self.current_token.kind
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
            crate::lexer::token::TokenKind::Let => self.parse_let_statement()?,
            crate::lexer::token::TokenKind::Return => self.parse_return_statement()?,
            _ => {
                return Err(format!(
                    "Tried to parse a statement from a non statement token {}",
                    self.current_token.kind
                ));
            }
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
}
