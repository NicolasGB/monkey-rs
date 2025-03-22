#[cfg(test)]
mod test {
    use crate::{
        Lexer,
        lexer::token::TokenKind,
        parser::{
            self,
            ast::{Program, Statement},
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
            Statement::Return(_) => panic!("Not a let statement"),
        }
    }

    fn test_return_statement(stmt: &Statement, ident: &str) {
        match stmt {
            Statement::Return(_) => {
                // TODO: Make indepth tests
            }
            Statement::Let(_) => panic!("Not a return statement"),
        }
    }

    fn parsing_errors(errs: Vec<String>) -> Program {
        for err in errs.iter() {
            eprintln!("Parsing error: {}", err)
        }
        panic!("Parsing exited with errors")
    }

    #[test]
    fn test_let_statements() {
        let input = r"
            let x = 5;
            let y = 10;
            let foobar = 838383;
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
}
