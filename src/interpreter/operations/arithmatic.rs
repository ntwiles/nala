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
