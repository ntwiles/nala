mod arithmatic;
pub mod equals;
mod errors;
pub mod gt;
pub mod lt;

use crate::{
    ast::{math::*, terms::*},
    interpreter::evaluate_if_symbol,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

use super::functions::*;

use arithmatic::*;

pub fn evaluate_addend(
    addend: &Addend,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    match addend {
        Addend::Add(left, right) => {
            let left = evaluate_addend(left, scopes, current_scope, context);
            let right = evaluate_factor(right, scopes, current_scope, context);

            let left = evaluate_if_symbol(left, scopes, current_scope);
            let right = evaluate_if_symbol(right, scopes, current_scope);

            do_add(left, right)
        }
        Addend::Sub(left, right) => {
            let left = evaluate_addend(left, scopes, current_scope, context);
            let right = evaluate_factor(right, scopes, current_scope, context);

            let left = evaluate_if_symbol(left, scopes, current_scope);
            let right = evaluate_if_symbol(right, scopes, current_scope);

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
) -> Term {
    match factor {
        Factor::Mult(left, right) => {
            let left = evaluate_factor(left, scopes, current_scope, context);
            let right = right.clone();

            let left = evaluate_if_symbol(left, scopes, current_scope);
            let right = evaluate_if_symbol(right, scopes, current_scope);

            do_multiply(left, right)
        }
        Factor::Div(left, right) => {
            let left = evaluate_factor(left, scopes, current_scope, context);
            let right = right.clone();

            let left = evaluate_if_symbol(left, scopes, current_scope);
            let right = evaluate_if_symbol(right, scopes, current_scope);

            do_divide(left, right)
        }
        Factor::Call(call) => evaluate_call(call, scopes, current_scope, context),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        ast::{arrays::*, funcs::*},
        io_context::TestContext,
    };

    #[test]
    pub fn it_evaluates_add_with_2_terms() {
        let mut test_context = TestContext::new();

        let left = Box::new(Addend::Factor(Factor::Call(Call::Index(Index::Term(
            Term::Num(7.0),
        )))));
        let right = Factor::Call(Call::Index(Index::Term(Term::Num(4.0))));

        let operation = Addend::Add(left, right);
        let mut scopes = Scopes::new();
        let top_scope = scopes.new_scope(None);
        let actual = evaluate_addend(&operation, &mut scopes, top_scope, &mut test_context);

        if let Term::Num(actual) = actual {
            assert_eq!(11.0, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_evaluates_add_with_3_terms() {
        let mut test_context = TestContext::new();

        let left = Addend::Factor(Factor::Call(Call::Index(Index::Term(Term::Num(3.0)))));
        let middle = Factor::Call(Call::Index(Index::Term(Term::Num(5.0))));
        let right = Factor::Call(Call::Index(Index::Term(Term::Num(4.0))));

        let operation_a = Addend::Add(Box::new(left), middle);
        let operation_b = Addend::Add(Box::new(operation_a), right);
        let mut scopes = Scopes::new();
        let top_scope = scopes.new_scope(None);
        let actual = evaluate_addend(&operation_b, &mut scopes, top_scope, &mut test_context);

        if let Term::Num(actual) = actual {
            assert_eq!(12.0, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_evaluates_sub() {
        let mut test_context = TestContext::new();

        let left = Addend::Factor(Factor::Call(Call::Index(Index::Term(Term::Num(5.0)))));
        let right = Factor::Call(Call::Index(Index::Term(Term::Num(3.0))));

        let operation = Addend::Sub(Box::new(left), right);
        let mut scopes = Scopes::new();
        let top_scope = scopes.new_scope(None);
        let actual = evaluate_addend(&operation, &mut scopes, top_scope, &mut test_context);

        if let Term::Num(actual) = actual {
            assert_eq!(2.0, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_evaluates_mult() {
        let mut test_context = TestContext::new();

        let left = Factor::Call(Call::Index(Index::Term(Term::Num(5.0))));
        let right = Term::Num(3.0);

        let operation = Factor::Mult(Box::new(left), right);
        let mut scopes = Scopes::new();
        let top_scope = scopes.new_scope(None);
        let actual = evaluate_factor(&operation, &mut scopes, top_scope, &mut test_context);

        if let Term::Num(actual) = actual {
            assert_eq!(15.0, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_evaluates_div() {
        let mut test_context = TestContext::new();

        let left = Factor::Call(Call::Index(Index::Term(Term::Num(5.0))));
        let right = Term::Num(2.0);

        let operation = Factor::Div(Box::new(left), right);
        let mut scopes = Scopes::new();
        let top_scope = scopes.new_scope(None);
        let actual = evaluate_factor(&operation, &mut scopes, top_scope, &mut test_context);

        if let Term::Num(actual) = actual {
            assert_eq!(2.5, actual);
        } else {
            panic!();
        }
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero.")]
    pub fn it_disallows_div_by_zero() {
        let mut test_context = TestContext::new();

        let left = Factor::Call(Call::Index(Index::Term(Term::Num(5.0))));
        let right = Term::Num(0.0);

        let operation = Factor::Div(Box::new(left), right);
        let mut scopes = Scopes::new();
        let top_scope = scopes.new_scope(None);
        evaluate_factor(&operation, &mut scopes, top_scope, &mut test_context);
    }
}
