use regex::Regex;
use std::iter::Peekable;
use std::str::Chars;

use crate::token::{Token, Token::*};

// Note: For now this is being ignored in favor of LALRPOP's default tokenizer.
pub struct Lexer;

impl Lexer {
    pub fn lex_code(code: String) -> Vec<Token> {
        const SPECIAL_CHARS: &str = ";";
        const WHITESPACE: &str = " \n";

        let mut stream = code.chars().peekable();
        let mut tokens = Vec::<Token>::new();

        while let Some(c) = stream.next() {
            if WHITESPACE.contains(c) {
                continue;
            } else if SPECIAL_CHARS.contains(c) {
                tokens.push(Semicolon)
            } else if c == '\'' {
                tokens.push(Lexer::scan_string(&mut stream))
            } else if Lexer::is_letter(&c) {
                tokens.push(Lexer::scan_symbol(c, &mut stream))
            }
        }

        tokens
    }

    fn scan_symbol(first: char, stream: &mut Peekable<Chars<'_>>) -> Token {
        let mut ret = String::from(first);

        while let Some(c) = stream.peek() {
            if Lexer::is_letter(c) {
                ret.push(stream.next().unwrap())
            } else {
                stream.next();
                break;
            }
        }

        Symbol(ret)
    }

    fn scan_string(stream: &mut Peekable<Chars<'_>>) -> Token {
        let mut ret = String::new();

        while let Some(c) = stream.peek() {
            if c.clone() != '\'' {
                ret.push(stream.next().unwrap())
            } else {
                stream.next();
                break;
            }
        }

        Str(ret)
    }

    fn is_letter(c: &char) -> bool {
        let mut tmp = [0; 4];
        Regex::new(r"[a-zA-Z]")
            .unwrap()
            .is_match(c.encode_utf8(&mut tmp))
    }
}
