use crate::ast::terms::Term;

pub fn do_add(left: Term, right: Term) -> Term {
    match left {
        Term::Num(left) => {
            if let Term::Num(right) = right {
                Term::Num(left + right)
            } else {
                panic!("todo")
            }
        }
        Term::String(left) => {
            if let Term::String(right) = right {
                Term::String(left + &right)
            } else {
                panic!("todo")
            }
        }
        _ => panic!("todo"),
    }
}

pub fn do_subtract(left: Term, right: Term) -> Term {
    if let Term::Num(left) = left {
        if let Term::Num(right) = right {
            Term::Num(left - right)
        } else {
            panic!("todo")
        }
    } else {
        panic!("todo")
    }
}

pub fn do_multiply(left: Term, right: Term) -> Term {
    if let Term::Num(left) = left {
        if let Term::Num(right) = right {
            Term::Num(left * right)
        } else {
            panic!("todo")
        }
    } else {
        panic!("todo")
    }
}

pub fn do_divide(left: Term, right: Term) -> Term {
    if let Term::Num(left) = left {
        if let Term::Num(right) = right {
            if right != 0.0 {
                Term::Num(left / right)
            } else {
                panic!("Cannot divide by zero.")
            }
        } else {
            panic!("todo")
        }
    } else {
        panic!("todo")
    }
}
