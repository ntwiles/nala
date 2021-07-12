use crate::ast::*;

pub fn interpret_tree(parsed: Stmt) {
    match parsed {
        Stmt::Print(expr) => interpret_print(expr),
    }
}

fn interpret_print(expr: Expr) {
    let result = evaluate_expr(expr);
    println!("{}", result.to_string());
}

fn evaluate_expr(expr: Expr) -> Term {
    match expr {
        Expr::Add(left, right) => {
            let left = evaluate_expr(*left);
            let right = evaluate_factor(right);
            evaluate_oper(left, OpKind::Add, right)
        }
        Expr::Sub(left, right) => {
            let left = evaluate_expr(*left);
            let right = evaluate_factor(right);
            evaluate_oper(left, OpKind::Sub, right)
        }
        Expr::Factor(factor) => evaluate_factor(factor),
    }
}

fn evaluate_factor(factor: Factor) -> Term {
    match factor {
        Factor::Mult(left, right) => {
            let left = evaluate_factor(*left);
            evaluate_oper(left, OpKind::Mult, right)
        }
        Factor::Term(term) => term,
    }
}

fn evaluate_oper(left: Term, op_kind: OpKind, right: Term) -> Term {
    if let Term::Num(left) = left {
        if let Term::Num(right) = right {
            match op_kind {
                OpKind::Add => Term::Num(left + right),
                OpKind::Sub => Term::Num(left - right),
                OpKind::Mult => Term::Num(left * right),
            }
        } else {
            unimplemented!();
        }
    } else {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_evaluates_add_with_2_terms() {
        let left = Box::new(Expr::Factor(Factor::Term(Term::Num(7.0))));
        let right = Factor::Term(Term::Num(4.0));
        let operation = Expr::Add(left, right);
        let actual = evaluate_expr(operation);

        if let Term::Num(actual) = actual {
            assert_eq!(11.0, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_evaluates_add_with_3_terms() {
        let left = Expr::Factor(Factor::Term(Term::Num(3.0)));
        let middle = Factor::Term(Term::Num(5.0));
        let right = Factor::Term(Term::Num(4.0));
        let operation_a = Expr::Add(Box::new(left), middle);
        let operation_b = Expr::Add(Box::new(operation_a), right);
        let actual = evaluate_expr(operation_b);

        if let Term::Num(actual) = actual {
            assert_eq!(12.0, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_evaluates_sub() {
        let left = Expr::Factor(Factor::Term(Term::Num(5.0)));
        let right = Factor::Term(Term::Num(3.0));
        let operation = Expr::Sub(Box::new(left), right);
        let actual = evaluate_expr(operation);

        if let Term::Num(actual) = actual {
            assert_eq!(2.0, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_evaluates_mult() {
        let left = Factor::Term(Term::Num(5.0));
        let right = Term::Num(3.0);
        let operation = Factor::Mult(Box::new(left), right);
        let actual = evaluate_factor(operation);

        if let Term::Num(actual) = actual {
            assert_eq!(15.0, actual);
        } else {
            panic!();
        }
    }
}
