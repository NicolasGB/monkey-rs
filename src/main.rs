mod lexer;
mod parser;

use std::io::stdin;

use lexer::{Lexer, token};

fn main() {
    println!("Welcome to the Monkey REPL!");

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input.trim_end().is_empty() {
            std::process::exit(0);
        }

        let mut l = Lexer::new(&input);
        loop {
            let t = l.next_token();
            if t.kind == token::TokenKind::Eof {
                break;
            } else {
                println!(
                    "token: {}, literal value: {}",
                    t,
                    &input[t.span.start..=t.span.end]
                )
            }
        }
    }
}
