use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

use grammar::StmtParser;

use crate::ast::*;

// TODO: Get rid of this struct.
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

        if let Stmt::Print(Expr::Factor(Factor::Term(Term::String(message)))) = parsed {
            assert_eq!(message, String::from("hello world"));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_parses_print_statements_with_number_literals() {
        let parsed = Parser::parse_code(String::from("print 313;"));

        if let Stmt::Print(Expr::Factor(Factor::Term(Term::Num(number)))) = parsed {
            assert_eq!(number, 313);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_parses_print_statements_with_add_expressions() {
        let parsed = Parser::parse_code(String::from("print 2 + 3;"));

        if let Stmt::Print(Expr::Oper(left, op_kind, right)) = parsed {
            // TODO: box_patterns feature may make this uncessecary when stable.
            let left = *left;
            assert!(matches!(left, Expr::Factor(Factor::Term(Term::Num(_)))));
            assert!(matches!(op_kind, OpKind::Add));
            assert!(matches!(right, Factor::Term(Term::Num(_))));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_parses_print_statements_with_add_expressions_three_terms() {
        let parsed = Parser::parse_code(String::from("print 2 + 3 + 4;"));

        if let Stmt::Print(Expr::Oper(left, op_kind, right)) = parsed {
            // TODO: box_patterns feature may make this uncessecary when stable.
            let left = *left;
            // TODO: box_patterns can also allow the first _ here to be replaced with
            // a more precise pattern.
            assert!(matches!(
                left,
                Expr::Oper(_, OpKind::Add, Factor::Term(Term::Num(_)))
            ));
            assert!(matches!(op_kind, OpKind::Add));
            assert!(matches!(right, Factor::Term(Term::Num(_))));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_parses_print_statements_with_mult_expressions() {
        let parsed = Parser::parse_code(String::from("print 2 * 4;"));

        if let Stmt::Print(Expr::Factor(Factor::Oper(left, op_kind, right))) = parsed {
            // TODO: box_patterns feature may make this uncessecary when stable.
            let left = *left;
            assert!(matches!(left, Factor::Term(Term::Num(_))));
            assert!(matches!(op_kind, OpKind::Mult));
            assert!(matches!(right, Term::Num(_)));
        } else {
            panic!();
        }
    }
}
