lalrpop_mod!(pub test); // synthesized by LALRPOP

use crate::ast::Stmt;

pub struct Parser;

impl Parser {
    pub fn parse_code(code: String) -> Stmt {
        let parser = grammar::StmtParser::new();
        parser.parse(&code).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;

    #[test]
    pub fn it_identifies_print_statements() {
        let parsed = Parser::parse_code(String::from("print 'hello world';"));
        assert!(matches!(parsed, ast::Stmt::Print(_)));
    }

    #[test]
    pub fn it_parses_print_statements_with_string_literals() {
        let parsed = Parser::parse_code(String::from("print 'hello world';"));

        if let ast::Stmt::Print(ast::Literal::String(message)) = parsed {
            assert_eq!(message, String::from("hello world"));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_parses_print_statements_with_number_literals() {
        let parsed = Parser::parse_code(String::from("print 313;"));

        if let ast::Stmt::Print(ast::Literal::Num(number)) = parsed {
            assert_eq!(number, 313);
        } else {
            panic!();
        }
    }
}
