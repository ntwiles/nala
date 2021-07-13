use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

use grammar::ProgramParser;

use crate::ast::*;

pub fn parse_code(code: String) -> Program {
    ProgramParser::new().parse(&code).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_identifies_print_statements() {
        let parsed = parse_code(String::from("print 'hello world';"));
        if let Program::Stmt(stmt) = parsed {
            assert!(matches!(stmt, Stmt::Print(_)));
        }
    }

    #[test]
    pub fn it_parses_print_statements_with_string_literals() {
        let parsed = parse_code(String::from("print 'hello world';"));

        if let Program::Stmt(Stmt::Print(Expr::Factor(Factor::Term(Term::String(message))))) =
            parsed
        {
            assert_eq!(message, String::from("hello world"));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_parses_print_statements_with_number_literals() {
        let parsed = parse_code(String::from("print 313;"));

        if let Program::Stmt(Stmt::Print(Expr::Factor(Factor::Term(Term::Num(number))))) = parsed {
            assert_eq!(number, 313.0);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_parses_print_statements_with_add_expressions() {
        let parsed = parse_code(String::from("print 2 + 3;"));

        if let Program::Stmt(Stmt::Print(Expr::Add(left, right))) = parsed {
            // TODO: box_patterns feature may make this uncessecary when stable.
            let left = *left;
            assert!(matches!(left, Expr::Factor(Factor::Term(Term::Num(_)))));
            assert!(matches!(right, Factor::Term(Term::Num(_))));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_parses_print_statements_with_add_expressions_three_terms() {
        let parsed = parse_code(String::from("print 2 + 3 + 4;"));

        if let Program::Stmt(Stmt::Print(Expr::Add(left, right))) = parsed {
            // TODO: box_patterns feature may make this uncessecary when stable.
            let left = *left;
            // TODO: box_patterns can also allow the first _ here to be replaced with
            // a more precise pattern.
            assert!(matches!(left, Expr::Add(_, Factor::Term(Term::Num(_)))));
            assert!(matches!(right, Factor::Term(Term::Num(_))));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_parses_print_statements_with_mult_expressions() {
        let parsed = parse_code(String::from("print 2 * 4;"));

        if let Program::Stmt(Stmt::Print(Expr::Factor(Factor::Mult(left, right)))) = parsed {
            // TODO: box_patterns feature may make this uncessecary when stable.
            let left = *left;
            assert!(matches!(left, Factor::Term(Term::Num(_))));
            assert!(matches!(right, Term::Num(_)));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_parses_print_statements_with_div_expressions() {
        let parsed = parse_code(String::from("print 4 / 2;"));

        if let Program::Stmt(Stmt::Print(Expr::Factor(Factor::Div(left, right)))) = parsed {
            // TODO: box_patterns feature may make this uncessecary when stable.
            let left = *left;
            assert!(matches!(left, Factor::Term(Term::Num(_))));
            assert!(matches!(right, Term::Num(_)));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_parses_print_statements_with_identifiers() {
        let parsed = parse_code(String::from("const foo = bar;"));

        assert!(matches!(
            parsed,
            Program::Stmt(Stmt::Declare(_, Expr::Factor(Factor::Term(Term::Symbol(_))))),
        ));
    }

    #[test]
    pub fn it_parses_const_statements() {
        let parsed = parse_code(String::from("const foo = 7;"));

        assert!(matches!(
            parsed,
            Program::Stmt(Stmt::Declare(_, Expr::Factor(Factor::Term(Term::Num(_))))),
        ));
    }
}
