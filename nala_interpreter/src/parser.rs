lalrpop_mod!(pub grammar);

use grammar::ProgramParser;
use lalrpop_util::{lalrpop_mod, ParseError};

use crate::ast::*;

// TODO: Get error line numbers working properly.
pub fn parse_code(code: String) -> Result<Program, String> {
    match ProgramParser::new().parse(&code) {
        Ok(parsed) => Ok(parsed),
        Err(error) => match error {
            ParseError::InvalidToken { location } => {
                // NOTE: `location` is a single usize ignoring lines.
                let snippet: String = code.chars().skip(location).collect();
                Err(format!(
                    "Invalid token at location {}:\n\n{}",
                    location, snippet
                ))
            }
            ParseError::UnrecognizedEOF { location, expected } => Err(format!(
                "Unrecognized EOF at location {}. Expected one of: {:?}",
                location, expected
            )),
            ParseError::UnrecognizedToken { token, expected } => Err(format!(
                "Unrecognized token {:?}. Expected one of: {:?}",
                token, expected
            )),
            _ => todo!("Unprocessed ParseError: {}", error.to_string()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::grammar::StmtsParser;

    #[test]
    fn it_parses_const_statements() {
        let parsed = StmtsParser::new().parse("const foo = 7;");
        assert!(parsed.is_ok());
    }
}
