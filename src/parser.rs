lalrpop_mod!(pub grammar);

use grammar::ProgramParser;
use lalrpop_util::lalrpop_mod;

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
    fn it_parses_const_statements() {
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

    // #[test]
    // fn it_fails_parse_when_assigning_type_void() {
    //     let code = fs::read_to_string("tests/nala/error/parse/assign-void.nl").unwrap();
    //     let parsed = ProgramParser::new().parse(&code);

    //     let re = regex!("Unrecognized token `break`");
    //     let error = parsed.unwrap_err().to_string();

    //     assert_regex_match!(re, &error);
    // }
}
