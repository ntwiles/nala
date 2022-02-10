use crate::{ast::terms::Term, errors::NalaRuntimeError};

pub fn do_add(left: Term, right: Term) -> Term {
    if left.get_type() != right.get_type() {
        panic!("Cannot add between values of two different types.")
    }

    match left {
        Term::Num(left) => {
            if let Term::Num(right) = right {
                Term::Num(left + right)
            } else {
                unreachable!()
            }
        }
        Term::String(left) => {
            if let Term::String(right) = right {
                Term::String(left + &right)
            } else {
                unreachable!()
            }
        }
        _ => unreachable!(),
    }
}

pub fn do_subtract(left: Term, right: Term) -> Term {
    if left.get_type() != right.get_type() {
        panic!("Cannot subtract between values of two different types.")
    }

    if let Term::Num(left) = left {
        if let Term::Num(right) = right {
            Term::Num(left - right)
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

pub fn do_multiply(left: Term, right: Term) -> Term {
    if left.get_type() != right.get_type() {
        panic!("Cannot multiply between values of two different types.")
    }

    if let Term::Num(left) = left {
        if let Term::Num(right) = right {
            Term::Num(left * right)
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

pub fn do_divide(left: Term, right: Term) -> Result<Term, NalaRuntimeError> {
    if left.get_type() != right.get_type() {
        panic!("Cannot divide between values of two different types.")
    }
    if let Term::Num(left) = left {
        if let Term::Num(right) = right {
            if right != 0.0 {
                Ok(Term::Num(left / right))
            } else {
                Err(NalaRuntimeError {
                    message: "Cannot divide by zero.".to_string(),
                })
            }
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}
