#[cfg(test)]
mod test {
    use crate::{
        Lexer,
        lexer::token::{Span, Token, TokenKind},
        parser::{
            self, Parser,
            ast::{Let, Program, Statement},
        },
    };

    fn test_let_statement(stmt: &Statement, ident: &str) {
        match stmt {
            Statement::Let(ls) => {
                // Should match a token kind ident with the given identifier
                assert_eq!(
                    TokenKind::Ident {
                        name: ident.to_string()
                    },
                    ls.identifier.kind,
                    "Expected ident {} found {}",
                    ident,
                    ls.identifier.kind,
                )
            }
            _ => panic!("Not a let statement"),
        }
    }

    fn test_return_statement(stmt: &Statement, ident: &str) {
        match stmt {
            Statement::Return(_) => {
                // TODO: Make indepth tests
            }
            _ => panic!("Not a return statement"),
        }
    }

    fn parsing_errors(errs: Vec<String>) -> Program {
        for err in errs.iter() {
            eprintln!("Parsing error: {}", err)
        }
        panic!("Parsing exited with errors")
    }

    // Panics if the program cannot be parsed
    fn parse_program(mut p: Parser) -> Program {
        // Check no errors
        let program = p.parse_program();

        match program {
            Ok(p) => p,
            Err(errs) => parsing_errors(errs),
        }
    }

    #[test]
    fn test_let_statements() {
        let input = r"
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

        let l = Lexer::new(input);
        let p = parser::Parser::new(l);

        // Check no errors
        let program = parse_program(p);

        assert_eq!(
            3,
            program.statements.len(),
            "program does not contain 3 statements got: {}",
            program.statements.len()
        );

        let idents = vec!["x", "y", "foobar"];

        for (index, ident) in idents.into_iter().enumerate() {
            let stmt = program.statements.get(index).unwrap();
            test_let_statement(stmt, ident);
        }
    }

    #[test]
    fn test_return_statements() {
        let input = r"
            return 5;
            return 10;
            return 993322;
        ";

        let l = Lexer::new(input);
        let mut p = parser::Parser::new(l);

        // Check no errors
        let program = p.parse_program();

        let program = match program {
            Ok(p) => p,
            Err(errs) => parsing_errors(errs),
        };

        assert_eq!(
            3,
            program.statements.len(),
            "program does not contain 3 statements got: {}",
            program.statements.len()
        );

        // TODO check expression matches
        // for (index, ident) in idents.into_iter().enumerate() {
        //     let stmt = program.statements.get(index).unwrap();
        //     test_let_statement(stmt, ident);
        // }
    }

    #[test]
    fn test_string() {
        let _p = Program {
            statements: vec![Statement::Let(Let {
                identifier: Token::new(
                    TokenKind::Ident {
                        name: "x".to_string(),
                    },
                    0,
                    1,
                ),
                exp: None,
                span: Span { start: 0, end: 1 },
            })],
        };
    }

    #[test]
    fn test_identifier() {
        let input = "foobar;";

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program = parse_program(parser);

        assert_eq!(
            1,
            program.statements.len(),
            "program does not contain 1 statement got: {}",
            program.statements.len()
        );

        assert_eq!("foobar", program.to_string(), "Identifier doesn't match")
    }

    #[test]
    fn test_integer_literal() {
        let input = "5;";

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program = parse_program(parser);

        assert_eq!(
            1,
            program.statements.len(),
            "program does not contain 1 statement got: {}",
            program.statements.len()
        );

        assert_eq!("5", program.to_string(), "Identifier doesn't match")
    }
}
