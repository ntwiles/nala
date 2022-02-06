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
            do_add(left, right)
        }
        Addend::Sub(left, right) => {
            let left = evaluate_addend(left, scopes, current_scope, context);
            let right = evaluate_factor(right, scopes, current_scope, context);
            evaluate_oper(left, OpKind::Sub, right, scopes, current_scope)
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
        Factor::Mult(left, right) => evaluate_oper(
            evaluate_factor(left, scopes, current_scope, context),
            OpKind::Mult,
            right.clone(),
            scopes,
            current_scope,
        ),
        Factor::Div(left, right) => evaluate_oper(
            evaluate_factor(left, scopes, current_scope, context),
            OpKind::Div,
            right.clone(),
            scopes,
            current_scope,
        ),
        Factor::Call(call) => evaluate_call(call, scopes, current_scope, context),
    }
}

// TODO: Simplify this.
// TODO: Use existing or new macro in errors.rs.
fn evaluate_oper(
    left: Term,
    op_kind: OpKind,
    right: Term,
    scopes: &mut Scopes,
    current_scope: ScopeId,
) -> Term {
    let left = evaluate_if_symbol(left, scopes, current_scope);
    let right = evaluate_if_symbol(right, scopes, current_scope);

    match left {
        Term::Num(left) => match right {
            Term::Num(right) => match op_kind {
                OpKind::Sub => Term::Num(left - right),
                OpKind::Mult => Term::Num(left * right),
                OpKind::Div => Term::Num(do_divide(left, right)),
                _ => todo!("Get rid of this catchall."),
            },
            right => {
                panic!(
                    "Cannot perform arithmetic operations between types of Num and {}.",
                    right.get_type().to_string()
                )
            }
        },
        Term::String(left) => match right {
            Term::String(right) => {
                if let OpKind::Add = op_kind {
                    Term::String(left + &right)
                } else {
                    panic!(
                        "Operation not supported between values of type String: {:?}",
                        op_kind
                    )
                }
            }
            right => {
                panic!(
                    "Cannot perform arithmetic operations between types of String and {}.",
                    right.get_type().to_string()
                )
            }
        },
        left => {
            panic!(
                "Cannot perform arithmetic operations between values of type {}.",
                left.get_type().to_string()
            )
        }
    }
}

fn do_divide(left: f32, right: f32) -> f32 {
    if right != 0.0 {
        left / right
    } else {
        panic!("Cannot divide by zero.")
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
