use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

use grammar::ProgramParser;

use crate::ast::*;

pub fn parse_code(code: String) -> Option<Program> {
    match ProgramParser::new().parse(&code) {
        Ok(parsed) => Some(parsed),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{grammar::StmtsParser, *};

    #[test]
    pub fn it_parses_const_statements() {
        let parsed = StmtsParser::new().parse("const foo = 7;");

        assert!(matches!(
            parsed,
            Ok(Stmts::Stmt(Stmt::Declare(
                _,
                Expr::KindValue(KindValue::Addend(Addend::Factor(Factor::Call(
                    Call::Index(Index::Term(Term::Num(_)))
                )))),
                false
            ),),),
        ));
    }
}
