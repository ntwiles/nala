mod arithmatic;
pub mod equals;
mod errors;
pub mod gt;
pub mod lt;

use crate::{
    ast::{math::*, terms::*, types::PrimitiveInterface::*},
    errors::NalaRuntimeError,
    interpreter::evaluate_term,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

use super::functions::*;

use self::arithmatic::*;
use self::errors::check_operator_implemented_both;

pub fn evaluate_addend(
    addend: &Addend,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Value, NalaRuntimeError> {
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

            do_add(left, right)
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

            do_subtract(left, right)
        }
        Addend::Factor(factor) => evaluate_factor(factor, scopes, current_scope, context),
    }
}

pub fn evaluate_factor(
    factor: &Factor,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Value, NalaRuntimeError> {
    match factor {
        Factor::Mult(left, right) => {
            let left = evaluate_factor(left, scopes, current_scope, context)?;
            let right = evaluate_term(right.clone(), scopes, current_scope, context)?;

            check_operator_implemented_both(
                left.get_type(),
                right.get_type(),
                "*".to_string(),
                IMultiply,
            )?;

            do_multiply(left, right)
        }
        Factor::Div(left, right) => {
            let left = evaluate_factor(left, scopes, current_scope, context)?;
            let right = evaluate_term(right.clone(), scopes, current_scope, context)?;

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

    macro_rules! interpret {
        ($tree: expr, $interpreter: expr) => {{
            let mut test_context = TestContext::new();

            let mut scopes = Scopes::new();
            let top_scope = scopes.new_scope(None);
            $interpreter($tree, &mut scopes, top_scope, &mut test_context)
        }};

        ($tree: expr, $interpreter: expr, $context: expr) => {{
            let mut scopes = Scopes::new();
            let top_scope = scopes.new_scope(None);
            $interpreter($tree, &mut scopes, top_scope, &mut $context)
        }};
    }

    #[test]
    pub fn it_evaluates_add_with_2_terms() {
        let parsed = grammar::AddendParser::new().parse("7.0 + 4.0");
        let result = interpret!(&parsed.unwrap(), evaluate_addend).unwrap();
        assert_eq!(Value::Num(11.0), result);
    }

    #[test]
    pub fn it_evaluates_add_with_3_terms() {
        let parsed = grammar::AddendParser::new().parse("3.0 + 5.0 + 4.0");
        let result = interpret!(&parsed.unwrap(), evaluate_addend).unwrap();
        assert_eq!(Value::Num(12.0), result);
    }

    #[test]
    pub fn it_evaluates_sub() {
        let parsed = grammar::AddendParser::new().parse("5 - 3").unwrap();
        let result = interpret!(&parsed, evaluate_addend).unwrap();
        assert_eq!(Value::Num(2.0), result);
    }

    #[test]
    pub fn it_evaluates_mult() {
        let parsed = grammar::FactorParser::new().parse("5.0 * 3.0").unwrap();
        let result = interpret!(&parsed, evaluate_factor).unwrap();
        assert_eq!(Value::Num(15.0), result);
    }

    #[test]
    pub fn it_evaluates_div() {
        let parsed = grammar::FactorParser::new().parse("5.0 / 2.0").unwrap();
        let result = interpret!(&parsed, evaluate_factor).unwrap();
        assert_eq!(Value::Num(2.5), result);
    }

    #[test]
    pub fn it_disallows_div_by_zero() {
        let parsed = grammar::FactorParser::new().parse("5.0 / 0.0").unwrap();
        let actual = interpret!(&parsed, evaluate_factor);

        assert!(matches!(actual, Err(_)));

        let error = actual.unwrap_err();
        assert_eq!(error.message, "Cannot divide by zero.");
    }
}
