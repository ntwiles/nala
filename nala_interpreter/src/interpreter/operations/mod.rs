mod arithmatic;
pub mod equals;
mod errors;
pub mod gt;
pub mod lt;

use crate::{
    ast::math::*, errors::RuntimeError, interpreter::eval_term, io_context::IoContext,
    resolved::value::Value, scopes::Scopes,
};

use super::functions::*;

use self::arithmatic::*;

pub fn eval_addend(
    addend: &Addend,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match addend {
        Addend::Add(left, right) => {
            let left = eval_addend(left, scopes, current_scope, ctx)?;
            let right = eval_factor(right, scopes, current_scope, ctx)?;

            do_add(left, right, scopes, current_scope)
        }
        Addend::Sub(left, right) => {
            let left = eval_addend(left, scopes, current_scope, ctx)?;
            let right = eval_factor(right, scopes, current_scope, ctx)?;

            do_subtract(left, right, scopes, current_scope)
        }
        Addend::Factor(factor) => eval_factor(factor, scopes, current_scope, ctx),
    }
}

pub fn eval_factor(
    factor: &Factor,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match factor {
        Factor::Mult(left, right) => {
            let left = eval_factor(left, scopes, current_scope, ctx)?;
            let right = eval_term(right.clone(), scopes, current_scope)?;

            do_multiply(left, right, scopes, current_scope)
        }
        Factor::Div(left, right) => {
            let left = eval_factor(left, scopes, current_scope, ctx)?;
            let right = eval_term(right.clone(), scopes, current_scope)?;

            do_divide(left, right, scopes, current_scope)
        }
        Factor::Call(call) => eval_call(call, scopes, current_scope, ctx),
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

        ($tree: expr, $interpreter: expr, $ctx: expr) => {{
            let mut scopes = Scopes::new();
            let top_scope = scopes.new_scope(None);
            $interpreter($tree, &mut scopes, top_scope, &mut $ctx)
        }};
    }

    #[test]
    pub fn it_evaluates_add_with_2_terms() {
        let parsed = grammar::AddendParser::new().parse("7.0 + 4.0");
        let result = interpret!(&parsed.unwrap(), eval_addend).unwrap();
        assert_eq!(Value::Num(11.0), result);
    }

    #[test]
    pub fn it_evaluates_add_with_3_terms() {
        let parsed = grammar::AddendParser::new().parse("3.0 + 5.0 + 4.0");
        let result = interpret!(&parsed.unwrap(), eval_addend).unwrap();
        assert_eq!(Value::Num(12.0), result);
    }

    #[test]
    pub fn it_evaluates_sub() {
        let parsed = grammar::AddendParser::new().parse("5 - 3").unwrap();
        let result = interpret!(&parsed, eval_addend).unwrap();
        assert_eq!(Value::Num(2.0), result);
    }

    #[test]
    pub fn it_evaluates_mult() {
        let parsed = grammar::FactorParser::new().parse("5.0 * 3.0").unwrap();
        let result = interpret!(&parsed, eval_factor).unwrap();
        assert_eq!(Value::Num(15.0), result);
    }

    #[test]
    pub fn it_evaluates_div() {
        let parsed = grammar::FactorParser::new().parse("5.0 / 2.0").unwrap();
        let result = interpret!(&parsed, eval_factor).unwrap();
        assert_eq!(Value::Num(2.5), result);
    }

    #[test]
    pub fn it_disallows_div_by_zero() {
        let parsed = grammar::FactorParser::new().parse("5.0 / 0.0").unwrap();
        let actual = interpret!(&parsed, eval_factor);

        assert!(matches!(actual, Err(_)));

        let error = actual.unwrap_err();
        assert_eq!(error.message, "Cannot divide by zero.");
    }
}
