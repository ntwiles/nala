use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

use grammar::ProgramParser;

use crate::ast::*;

pub fn parse_code(code: String) -> Result<Program, String> {
    match ProgramParser::new().parse(&code) {
        Ok(parsed) => Ok(parsed),
        Err(error) => Err(error.to_string()),
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
