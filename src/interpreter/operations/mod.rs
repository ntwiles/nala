mod arithmatic;
pub mod equals;
mod errors;
pub mod gt;
pub mod lt;

use crate::{
    ast::{math::*, terms::*, types::PrimitiveInterface::*},
    errors::NalaRuntimeError,
    interpreter::evaluate_if_symbol,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

use super::functions::*;

use arithmatic::*;
use errors::check_operator_implemented_both;

pub fn evaluate_addend(
    addend: &Addend,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Term, NalaRuntimeError> {
    match addend {
        Addend::Add(left, right) => {
            let left = evaluate_addend(left, scopes, current_scope, context)?;
            let right = evaluate_factor(right, scopes, current_scope, context)?;

            check_operator_implemented_both(
                left.get_type(),
                right.get_type(),
                "+".to_string(),
                IAdd,
            )?;

            Ok(do_add(left, right))
        }
        Addend::Sub(left, right) => {
            let left = evaluate_addend(left, scopes, current_scope, context)?;
            let right = evaluate_factor(right, scopes, current_scope, context)?;

            check_operator_implemented_both(
                left.get_type(),
                right.get_type(),
                "-".to_string(),
                ISubtract,
            )?;

            Ok(do_subtract(left, right))
        }
        Addend::Factor(factor) => evaluate_factor(factor, scopes, current_scope, context),
    }
}

pub fn evaluate_factor(
    factor: &Factor,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Term, NalaRuntimeError> {
    match factor {
        Factor::Mult(left, right) => {
            let left = evaluate_factor(left, scopes, current_scope, context)?;
            let right = evaluate_if_symbol(right.clone(), scopes, current_scope, context)?;

            check_operator_implemented_both(
                left.get_type(),
                right.get_type(),
                "*".to_string(),
                IMultiply,
            )?;

            Ok(do_multiply(left, right))
        }
        Factor::Div(left, right) => {
            let left = evaluate_factor(left, scopes, current_scope, context)?;
            let right = evaluate_if_symbol(right.clone(), scopes, current_scope, context)?;

            check_operator_implemented_both(
                left.get_type(),
                right.get_type(),
                "/".to_string(),
                IDivide,
            )?;

            do_divide(left, right)
        }
        Factor::Call(call) => evaluate_call(call, scopes, current_scope, context),
    }
}

#[cfg(test)]
mod tests {
    lalrpop_mod!(pub grammar);

    use lalrpop_util::lalrpop_mod;

    use super::*;

    use crate::io_context::TestContext;

    #[test]
    pub fn it_evaluates_add_with_2_terms() {
        let mut test_context = TestContext::new();

        let nala = "7.0 + 4.0";

        let parsed = grammar::AddendParser::new().parse(nala);

        let actual = if let Ok(parsed) = parsed {
            let mut scopes = Scopes::new();
            let top_scope = scopes.new_scope(None);

            evaluate_addend(&parsed, &mut scopes, top_scope, &mut test_context).unwrap()
        } else {
            panic!()
        };

        if let Term::Num(actual) = actual {
            assert_eq!(11.0, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_evaluates_add_with_3_terms() {
        let mut test_context = TestContext::new();

        let nala = "3.0 + 5.0 + 4.0";
        let parsed = grammar::AddendParser::new().parse(nala);

        let actual = if let Ok(parsed) = parsed {
            let mut scopes = Scopes::new();
            let top_scope = scopes.new_scope(None);

            evaluate_addend(&parsed, &mut scopes, top_scope, &mut test_context).unwrap()
        } else {
            panic!()
        };

        if let Term::Num(actual) = actual {
            assert_eq!(12.0, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_evaluates_sub() {
        let mut test_context = TestContext::new();

        let nala = "5 - 3";

        let parsed = grammar::AddendParser::new().parse(nala);

        let actual = if let Ok(parsed) = parsed {
            let mut scopes = Scopes::new();
            let top_scope = scopes.new_scope(None);

            evaluate_addend(&parsed, &mut scopes, top_scope, &mut test_context).unwrap()
        } else {
            panic!();
        };

        if let Term::Num(actual) = actual {
            assert_eq!(2.0, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_evaluates_mult() {
        let mut test_context = TestContext::new();

        let nala = "5.0 * 3.0";
        let parsed = grammar::FactorParser::new().parse(nala);

        let actual = if let Ok(parsed) = parsed {
            let mut scopes = Scopes::new();
            let top_scope = scopes.new_scope(None);

            evaluate_factor(&parsed, &mut scopes, top_scope, &mut test_context).unwrap()
        } else {
            panic!();
        };

        if let Term::Num(actual) = actual {
            assert_eq!(15.0, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_evaluates_div() {
        let mut test_context = TestContext::new();

        let nala = "5.0 / 2.0";
        let parsed = grammar::FactorParser::new().parse(nala);

        let actual = if let Ok(parsed) = parsed {
            let mut scopes = Scopes::new();
            let top_scope = scopes.new_scope(None);

            evaluate_factor(&parsed, &mut scopes, top_scope, &mut test_context).unwrap()
        } else {
            panic!();
        };

        if let Term::Num(actual) = actual {
            assert_eq!(2.5, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_disallows_div_by_zero() {
        let mut test_context = TestContext::new();

        let nala = "5.0 / 0.0";
        let parsed = grammar::FactorParser::new().parse(nala);

        let actual = if let Ok(parsed) = parsed {
            let mut scopes = Scopes::new();
            let top_scope = scopes.new_scope(None);

            evaluate_factor(&parsed, &mut scopes, top_scope, &mut test_context)
        } else {
            panic!();
        };

        assert!(matches!(actual, Err(_)));

        let error = actual.unwrap_err();
        assert_eq!(error.message, "Cannot divide by zero.");
    }
}
