use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

use grammar::StmtParser;

use crate::ast::*;

pub struct Parser;

impl Parser {
    pub fn parse_code(code: String) -> Stmt {
        let parser = StmtParser::new();
        parser.parse(&code).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_identifies_print_statements() {
        let parsed = Parser::parse_code(String::from("print 'hello world';"));
        assert!(matches!(parsed, Stmt::Print(_)));
    }

    #[test]
    pub fn it_parses_print_statements_with_string_literals() {
        let parsed = Parser::parse_code(String::from("print 'hello world';"));

        if let Stmt::Print(Expr::Term(Term::String(message))) = parsed {
            assert_eq!(message, String::from("hello world"));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_parses_print_statements_with_number_literals() {
        let parsed = Parser::parse_code(String::from("print 313;"));

        if let Stmt::Print(Expr::Term(Term::Num(number))) = parsed {
            assert_eq!(number, 313);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_parses_print_statements_with_add_expressions() {
        let parsed = Parser::parse_code(String::from("print 2 + 3;"));

        if let Stmt::Print(Expr::Oper(Term::Num(left), op_kind, Term::Num(right))) = parsed {
            assert_eq!(left, 2);
            assert!(matches!(op_kind, OpKind::Add));
            assert_eq!(right, 3);
        } else {
            panic!();
        }
    }
}
