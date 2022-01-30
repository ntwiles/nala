use super::functions::*;

use crate::{
    ast::*,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

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
            evaluate_oper(left, OpKind::Add, right, scopes, current_scope)
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

pub fn evaluate_equals(
    left: Term,
    right: Term,
    scopes: &mut Scopes,
    current_scope: ScopeId,
) -> Term {
    match left {
        Term::Num(left) => match right {
            Term::Num(right) => Term::Bool(left == right),
            Term::String(_) => panic!("Cannot perform comparisons between types Num and String."),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_equals(Term::Num(left), right, scopes, current_scope)
            }
            Term::Bool(_) => panic!("Cannot perform comparisons between types Num and Bool."),
            Term::Array(_) => panic!("Cannot perform comparisons between types Num and Array."),
            Term::Func(_, _) => panic!("Cannot perform comparisons between types Num and Func."),
            Term::Void => panic!("Cannot perform comparisons between types Num and Void."),
            Term::Break(_) => panic!("Cannot perform comparisons between types Num and Break."),
            Term::Type(type_kind) => {
                panic!("Cannot perform comparisons between types Num and Enum.")
            }
            Term::Kind(_) => panic!("Cannot perform comparisons between types Num and Kind"),
        },
        Term::String(left) => match right {
            Term::Num(_) => panic!("Cannot perform comparisons between types String and Num."),
            Term::String(right) => Term::Bool(left == right),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_equals(Term::String(left), right, scopes, current_scope)
            }
            Term::Bool(_) => panic!("Cannot perform comparisons between types String and Bool."),
            Term::Array(_) => panic!("Cannot perform comparisons between types String and Array."),
            Term::Func(_, _) => panic!("Cannot perform comparisons between types String and Func."),
            Term::Void => panic!("Cannot perform comparisons between types String and Void."),
            Term::Break(_) => panic!("Cannot perform comparisons between types String and Break."),
            Term::Type(type_kind) => {
                panic!("Cannot perform comparisons between types String and Enum.")
            }
            Term::Kind(_) => panic!("Cannot perform comparisons between types String and Kind."),
        },
        Term::Symbol(left) => {
            let left = scopes.get_value(&left, current_scope);
            evaluate_equals(left, right, scopes, current_scope)
        }
        Term::Bool(left) => match right {
            Term::Num(_) => panic!("Cannot perform comparisons between types Bool and Num."),
            Term::String(_) => panic!("Cannot perform comparisons between types Bool and String."),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_equals(Term::Bool(left), right, scopes, current_scope)
            }
            Term::Bool(right) => Term::Bool(left == right),
            Term::Array(_) => panic!("Cannot perform comparisons between types Bool and Array."),
            Term::Func(_, _) => panic!("Cannot perform comparisons between types Bool and Func."),
            Term::Void => panic!("Cannot perform comparisons between types Bool and Void."),
            Term::Break(_) => panic!("Cannot perform comparisons between types Bool and Break."),
            Term::Type(type_kind) => {
                panic!("Cannot perform comparisons between types Bool and Enum.")
            }
            Term::Kind(_) => panic!("Cannot perform comparisons between types Bool and Kind."),
        },
        Term::Array(_) => panic!("Cannot perform comparions against values of type Array."),
        Term::Func(_, _) => panic!("Cannot perform comparisons against values of type Func."),
        Term::Void => panic!("Cannot perform comparisons against values of type Void."),
        Term::Break(_) => panic!("Cannot perform comparisons against values of type Break."),
        Term::Type(type_kind) => panic!("Cannot perform comparisons against values of type Enum."),
        Term::Kind(left) => {
            if let Term::Kind(right) = right {
                Term::Bool(left == right)
            } else {
                panic!(
                    "Cannot perform comparisons against values of type {}.",
                    right.get_type().to_string()
                )
            }
        }
    }
}

pub fn evaluate_gt(left: Term, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    match left {
        Term::Num(left) => match right {
            Term::Num(right) => Term::Bool(left > right),
            Term::String(_) => panic!("Cannot perform comparisons between types Num and String."),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_gt(Term::Num(left), right, scopes, current_scope)
            }
            Term::Bool(_) => panic!("Cannot perform comparisons between types Num and Bool."),
            Term::Array(_) => panic!("Cannot perform comparisons between types Num and Array."),
            Term::Func(_, _) => panic!("Cannot perform comparisons between types Num and Func."),
            Term::Void => panic!("Cannot perform comparisons between types Num and Void."),
            Term::Break(_) => panic!("Cannot perform comparisons between types Num and Break."),
            Term::Type(_) => panic!("Cannot perform comparisons between types Num and Enum."),
            Term::Kind(_) => panic!("Cannot perform comparisons between types Num and adn Kind."),
        },
        Term::String(left) => match right {
            Term::Num(_) => panic!("Cannot perform comparisons between types String and Num."),
            Term::String(right) => Term::Bool(left > right),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_gt(Term::String(left), right, scopes, current_scope)
            }
            Term::Bool(_) => panic!("Cannot perform comparisons between types String and Bool."),
            Term::Array(_) => panic!("Cannot perform comparisons between types String and Array."),
            Term::Func(_, _) => panic!("Cannot perform comparisons between types String and Func."),
            Term::Void => panic!("Cannot perform comparisons between types String and Void."),
            Term::Break(_) => panic!("Cannot perform comparisons between types String and Break."),
            Term::Type(_) => panic!("Cannot perform comparisons between types String and Enum."),
            Term::Kind(_) => panic!("Cannot perform comparisons between types String and Kind."),
        },
        Term::Symbol(left) => {
            let left = scopes.get_value(&left, current_scope);
            evaluate_gt(left, right, scopes, current_scope)
        }
        Term::Bool(left) => match right {
            Term::Num(_) => panic!("Cannot perform comparisons between types Bool and Num."),
            Term::String(_) => panic!("Cannot perform comparisons between types Bool and String."),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_gt(Term::Bool(left), right, scopes, current_scope)
            }
            Term::Bool(right) => Term::Bool(left > right),
            Term::Array(_) => panic!("Cannot perform comparisons between types Bool and Array."),
            Term::Func(_, _) => panic!("Cannot perform comparisons between types Bool and Func."),
            Term::Void => panic!("Cannot perform comparisons between types Bool and Void."),
            Term::Break(_) => panic!("Cannot perform comparisons between types Bool and Break."),
            Term::Type(_) => panic!("Cannot perform comparisons between types Bool and Enum."),
            Term::Kind(_) => panic!("Cannot perform comparisons between types Bool and Kind."),
        },
        Term::Array(_) => panic!("Cannot perform comparions against values of type Array."),
        Term::Func(_, _) => panic!("Cannot perform comparisons against values of type Func."),
        Term::Void => panic!("Cannot perform comparisons against values of type Void."),
        Term::Break(_) => panic!("Cannot perform comparisons against values of type Break."),
        Term::Type(_) => panic!("Cannot perform comparisons against values of type Enum."),
        Term::Kind(_) => panic!("Cannot perform comparisons against values of type Kind."),
    }
}

pub fn evaluate_lt(left: Term, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    match left {
        Term::Num(left) => match right {
            Term::Num(right) => Term::Bool(left < right),
            Term::String(_) => panic!("Cannot perform comparisons between types Num and String."),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_lt(Term::Num(left), right, scopes, current_scope)
            }
            Term::Bool(_) => panic!("Cannot perform comparisons between types Num and Bool."),
            Term::Array(_) => panic!("Cannot perform comparisons between types Num and Array."),
            Term::Func(_, _) => panic!("Cannot perform comparisons between types Num and Func."),
            Term::Void => panic!("Cannot perform comparisons between types Num and Void."),
            Term::Break(_) => panic!("Cannot perform comparisons between types Num and Break."),
            Term::Type(_) => panic!("Cannot perform comparisons between types Num and Enum."),
            Term::Kind(_) => panic!("Cannot perform comparisons between types Num and Kind."),
        },
        Term::String(left) => match right {
            Term::Num(_) => panic!("Cannot perform comparisons between types String and Num."),
            Term::String(right) => Term::Bool(left < right),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_lt(Term::String(left), right, scopes, current_scope)
            }
            Term::Bool(_) => panic!("Cannot perform comparisons between types String and Bool."),
            Term::Array(_) => panic!("Cannot perform comparisons between types String and Array."),
            Term::Func(_, _) => panic!("Cannot perform comparisons between types String and Func."),
            Term::Void => panic!("Cannot perform comparisons between types String and Void."),
            Term::Break(_) => panic!("Cannot perform comparisons between types String and Break."),
            Term::Type(_) => panic!("Cannot perform comparisons between types String and Enum."),
            Term::Kind(_) => panic!("Cannot perform comparisons between types String and Kind."),
        },
        Term::Symbol(left) => {
            let left = scopes.get_value(&left, current_scope);
            evaluate_lt(left, right, scopes, current_scope)
        }
        Term::Bool(left) => match right {
            Term::Num(_) => panic!("Cannot perform comparisons between types Bool and Num."),
            Term::String(_) => panic!("Cannot perform comparisons between types Bool and String."),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_lt(Term::Bool(left), right, scopes, current_scope)
            }
            Term::Bool(right) => Term::Bool(left < right),
            Term::Array(_) => panic!("Cannot perform comparisons between types Bool and Array."),
            Term::Func(_, _) => panic!("Cannot perform comparisons between types Bool and Func."),
            Term::Void => panic!("Cannot perform comparisons between types Bool and Void."),
            Term::Break(_) => panic!("Cannot perform comparisons between types Bool and Break."),
            Term::Type(_) => panic!("Cannot perform comparisons between types Bool and Enum."),
            Term::Kind(_) => panic!("Cannot perform comparisons between types Bool and Kind."),
        },
        Term::Array(_) => panic!("Cannot perform comparisons against values of type Array."),
        Term::Func(_, _) => panic!("Cannot perform comparisons against values of type Func."),
        Term::Void => panic!("Cannot perform comparisons against values of type Void."),
        Term::Break(_) => panic!("Cannot perform comparisons against values of type Break."),
        Term::Type(_) => panic!("Cannot perform comparisons against values of type Enum."),
        Term::Kind(_) => panic!("Cannot perform comparisons against values of type Kind."),
    }
}

// TODO: Can this be simplified?
fn evaluate_oper(
    left: Term,
    op_kind: OpKind,
    right: Term,
    scopes: &mut Scopes,
    current_scope: ScopeId,
) -> Term {
    match left {
        Term::Num(left) => match right {
            Term::Num(right) => match op_kind {
                OpKind::Add => Term::Num(left + right),
                OpKind::Sub => Term::Num(left - right),
                OpKind::Mult => Term::Num(left * right),
                OpKind::Div => Term::Num(do_divide(left, right)),
            },
            Term::String(str) => {
                if let OpKind::Add = op_kind {
                    Term::String(left.to_string() + &str)
                } else {
                    panic!(
                        "Operation not supported between types Num and String: {:?}",
                        op_kind
                    )
                }
            }
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_oper(Term::Num(left), op_kind, right, scopes, current_scope)
            }
            Term::Bool(_) => {
                panic!("Cannot perform arithmetic operations between types of Num and Bool.")
            }
            Term::Array(_) => {
                panic!("Cannot perform arithmetic operations between types Num and Array.")
            }
            Term::Func(_, _) => {
                panic!("Cannot perform arithmetic operations between types Num and Func.")
            }
            Term::Void => {
                panic!("Cannot perform arithmetic operations between types Num and Void.")
            }
            Term::Break(_) => {
                panic!("Cannot perform arithmetic operations between types Num and Break.")
            }
            Term::Type(_) => {
                panic!("Cannot perform arithmetic operations between types Num and Enum.")
            }
            Term::Kind(_) => {
                panic!("Cannot perform arithmetic operations between types Num and Kind.")
            }
        },
        Term::String(left) => match right {
            Term::Num(num) => {
                if let OpKind::Add = op_kind {
                    Term::String(left + &num.to_string())
                } else {
                    panic!(
                        "Operation not supported between values of type String and Num: {:?}",
                        op_kind
                    )
                }
            }
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
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_oper(Term::String(left), op_kind, right, scopes, current_scope)
            }
            Term::Bool(_) => {
                panic!("Cannot perform arithmetic operations between types String and Bool.")
            }
            Term::Array(_) => {
                panic!("Cannot perform arithmetic operations between types String and Array.")
            }
            Term::Func(_, _) => {
                panic!("Cannot perform arithmetic operations between types String and Func.")
            }
            Term::Void => {
                panic!("Cannot perform arithmetic operations between types String and Void.")
            }
            Term::Break(_) => {
                panic!("Cannot perform arithmetic operations between types String and Break.")
            }
            Term::Type(_) => {
                panic!("Cannot perform arithmetic operations between types String and Enum.")
            }
            Term::Kind(_) => {
                panic!("Cannot perform arithmetic operations between types String and Kind.")
            }
        },
        Term::Symbol(left) => {
            let left = scopes.get_value(&left, current_scope);
            evaluate_oper(left, op_kind, right, scopes, current_scope)
        }
        Term::Bool(_) => {
            panic!("Cannot perform arithmetic operations between values of type Bool.")
        }
        Term::Array(_) => {
            panic!("Cannot perform arithmetic operations between values of type Array.")
        }
        Term::Func(_, _) => {
            panic!("Cannot perform arithmetic operations between values of type Func.")
        }
        Term::Void => {
            panic!("Cannot perform arithmetic operations between values of type Void.")
        }
        Term::Break(_) => {
            panic!("Cannot perform arithmetic operations between values of type Break.")
        }
        Term::Type(_) => {
            panic!("Cannot perform arithmetic operations between values of type Enum.")
        }
        Term::Kind(_) => {
            panic!("Cannot perform arithmetic operations between values of type Kind.")
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

    use crate::io_context::TestContext;

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
