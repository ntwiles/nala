lalrpop_mod!(pub test); // synthesized by LALRPOP

use crate::ast::Stmt;

pub struct Parser;

impl Parser {
    pub fn parse_code(code: String) -> Stmt {
        let parser = test::StmtParser::new();
        parser.parse(&code).unwrap()
    }
}
