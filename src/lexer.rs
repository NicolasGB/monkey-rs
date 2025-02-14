pub mod token;

use token::{Token, TokenKind};

pub struct Lexer<'s> {
    source: &'s str,
    pos: usize,
    next_pos: usize,
    ch: char,
}

impl<'s> Lexer<'s> {
    pub fn new(input: &'s str) -> Self {
        let mut l = Self {
            source: input,
            pos: 0,
            next_pos: 0,
            ch: 0 as char,
        };

        l.read_char();
        l
    }

    /// Read the next character and advances both current and next position
    /// If there's nothing more to advance, returns ascii 0 which is null
    fn read_char(&mut self) {
        if self.next_pos >= self.source.len() {
            self.ch = 0 as char;
        } else {
            self.ch = self.source.as_bytes()[self.next_pos] as char;
        }

        self.pos = self.next_pos;
        self.next_pos += 1;
    }

    /// Read the next character without advancing the current position
    /// If there's nothing more to advance, returns ascii 0 which is null
    /// This allows to see which is the immediate next character
    fn peek_char(&self) -> char {
        if self.next_pos >= self.source.len() {
            return 0 as char;
        }

        self.source.as_bytes()[self.next_pos] as char
    }

    /// Reads until it doesn't find an asci whitespace
    fn skip_withespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    /// Parse identifiers returns the token kind, the start and the end of the token
    /// Identifiers can either be keywords or user defined
    fn parse_identifier(&mut self) -> (TokenKind, usize, usize) {
        // Read the next char
        let start_ident = self.pos;

        // Read as long as the peeked next character stops being an ascii letter
        // We still verify if the current is a letter, this helps if the caller sent a wrong
        // char.
        while is_letter(self.ch) && is_letter(self.peek_char()) {
            self.read_char();
        }

        // Get the token kind, can either bey a keyword or a custom identifier
        let kind = token::lookup_identifier(&self.source[start_ident..=self.pos]);

        // Return
        (kind, start_ident, self.pos)
    }

    /// Parse integer returns the kind which will be an integer with the beginning and the end of the token
    fn parse_integer(&mut self) -> (TokenKind, usize, usize) {
        let start_ident = self.pos;

        // Read as long as the peeked next character stops being a digit
        // We still verify if the current is a letter, this helps if the caller sent a wrong
        // char.
        while self.ch.is_ascii_digit() && self.peek_char().is_ascii_digit() {
            self.read_char();
        }

        // Parse the integer
        let int: i64 = self.source[start_ident..=self.pos]
            .parse()
            // This is safe since we know there are only digits in the range
            .unwrap_or_default();

        // Now we build the integer kind
        let kind = TokenKind::Int(int);
        (kind, start_ident, self.pos)
    }

    // Tokenizes the next char
    pub fn next_token(&mut self) -> Token {
        // Start by skipping whitespaces
        self.skip_withespace();

        // Get start and end for simple tokens
        let (start, end) = (self.pos, self.next_pos);

        // Then parse the char to get the token
        let token = match self.ch {
            '(' => Token::new(TokenKind::LeftParen, start, end),
            ')' => Token::new(TokenKind::RightParen, start, end),
            '{' => Token::new(TokenKind::LeftBrace, start, end),
            '}' => Token::new(TokenKind::RightBrace, start, end),
            '=' => {
                // Check if its an Eq token
                if self.peek_char() == '=' {
                    self.read_char();
                    // Now the start is theone defined previously but the end is the current pos
                    Token::new(TokenKind::Eq, start, self.next_pos)
                } else {
                    Token::new(TokenKind::Assign, start, end)
                }
            }
            '+' => Token::new(TokenKind::Plus, start, end),
            ';' => Token::new(TokenKind::SemiColon, start, end),
            ',' => Token::new(TokenKind::Comma, start, end),
            '\u{0}' => Token::new(TokenKind::Eof, start, end),
            '!' => {
                // Check if it's a NotEq token
                if self.peek_char() == '=' {
                    self.read_char();
                    // Now the start is theone defined previously but the end is the current pos
                    Token::new(TokenKind::NotEq, start, self.next_pos)
                } else {
                    // Otherwise it's a simple bank
                    Token::new(TokenKind::Bang, start, end)
                }
            }
            '/' => Token::new(TokenKind::Slash, start, end),
            '<' => Token::new(TokenKind::LT, start, end),
            '>' => Token::new(TokenKind::GT, start, end),
            '-' => Token::new(TokenKind::Minus, start, end),
            '*' => Token::new(TokenKind::Star, start, end),
            _ => {
                if is_letter(self.ch) {
                    let (kind, start, end) = self.parse_identifier();
                    // Return directly otherwise we eat the next char since here we have advanced the
                    // cursors, this is due to the fact that read_char() is always called at the end,
                    // was nice debugging xD
                    Token::new(kind, start, end)
                } else if self.ch.is_ascii_digit() {
                    let (kind, start, end) = self.parse_integer();
                    // Return directly otherwise we eat the next char since here we have advanced the
                    // cursors, this is due to the fact that read_char() is always called at the end,
                    // was nice debugging xD
                    Token::new(kind, start, end)
                } else {
                    // Otherwise error out
                    eprintln!("Unexpected token found: {}", self.ch);
                    Token::new(TokenKind::Illegal, start, end)
                }
            }
        };

        self.read_char();
        token
    }
}

fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

#[cfg(test)]
mod tests {
    use token::TokenKind;

    use super::*;

    #[test]
    fn basic_symbols() {
        let input = "=+(){},;";

        let mut lex = Lexer::new(input);

        let expected = vec![
            TokenKind::Assign,
            TokenKind::Plus,
            TokenKind::LeftParen,
            TokenKind::RightParen,
            TokenKind::LeftBrace,
            TokenKind::RightBrace,
            TokenKind::Comma,
            TokenKind::SemiColon,
        ];

        for expect in expected {
            let t = lex.next_token();
            assert_eq!(t.kind, expect);
        }

        assert_eq!(lex.next_token().kind, TokenKind::Eof);
    }

    #[test]
    fn simple_monkey() {
        let input = r"
            let five = 5;
            let ten = 10;

            let add = fn(x,y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
              return true;
            } else {
              return false;
            }

            10 == 10;
            10 != 9;
        ";

        let mut lex = Lexer::new(input);

        let expected = vec![
            TokenKind::Let,
            TokenKind::Ident {
                name: "five".to_string(),
            },
            TokenKind::Assign,
            TokenKind::Int(5),
            TokenKind::SemiColon,
            TokenKind::Let,
            TokenKind::Ident {
                name: "ten".to_string(),
            },
            TokenKind::Assign,
            TokenKind::Int(10),
            TokenKind::SemiColon,
            TokenKind::Let,
            TokenKind::Ident {
                name: "add".to_string(),
            },
            TokenKind::Assign,
            TokenKind::Fn,
            TokenKind::LeftParen,
            TokenKind::Ident {
                name: "x".to_string(),
            },
            TokenKind::Comma,
            TokenKind::Ident {
                name: "y".to_string(),
            },
            TokenKind::RightParen,
            TokenKind::LeftBrace,
            TokenKind::Ident {
                name: "x".to_string(),
            },
            TokenKind::Plus,
            TokenKind::Ident {
                name: "y".to_string(),
            },
            TokenKind::SemiColon,
            TokenKind::RightBrace,
            TokenKind::SemiColon,
            TokenKind::Let,
            TokenKind::Ident {
                name: "result".to_string(),
            },
            TokenKind::Assign,
            TokenKind::Ident {
                name: "add".to_string(),
            },
            TokenKind::LeftParen,
            TokenKind::Ident {
                name: "five".to_string(),
            },
            TokenKind::Comma,
            TokenKind::Ident {
                name: "ten".to_string(),
            },
            TokenKind::RightParen,
            TokenKind::SemiColon,
            TokenKind::Bang,
            TokenKind::Minus,
            TokenKind::Slash,
            TokenKind::Star,
            TokenKind::Int(5),
            TokenKind::SemiColon,
            TokenKind::Int(5),
            TokenKind::LT,
            TokenKind::Int(10),
            TokenKind::GT,
            TokenKind::Int(5),
            TokenKind::SemiColon,
            TokenKind::If,
            TokenKind::LeftParen,
            TokenKind::Int(5),
            TokenKind::LT,
            TokenKind::Int(10),
            TokenKind::RightParen,
            TokenKind::LeftBrace,
            TokenKind::Return,
            TokenKind::True,
            TokenKind::SemiColon,
            TokenKind::RightBrace,
            TokenKind::Else,
            TokenKind::LeftBrace,
            TokenKind::Return,
            TokenKind::False,
            TokenKind::SemiColon,
            TokenKind::RightBrace,
            TokenKind::Int(10),
            TokenKind::Eq,
            TokenKind::Int(10),
            TokenKind::SemiColon,
            TokenKind::Int(10),
            TokenKind::NotEq,
            TokenKind::Int(9),
            TokenKind::SemiColon,
        ];

        for expect in expected {
            let t = lex.next_token();
            assert_eq!(t.kind, expect)
        }

        assert_eq!(lex.next_token().kind, TokenKind::Eof)
    }
}
