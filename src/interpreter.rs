use crate::ast::*;

pub fn interpret(parsed: Stmt) {
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
        Expr::Oper(left, op_kind, right) => {
            let left = evaluate_expr(*left);
            let right = evaluate_factor(right);
            evaluate_oper(left, op_kind, right)
        }
        Expr::Factor(factor) => evaluate_factor(factor),
    }
}

fn evaluate_factor(factor: Factor) -> Term {
    match factor {
        Factor::Oper(left, op_kind, right) => {
            let left = evaluate_factor(*left);
            evaluate_oper(left, op_kind, right)
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
        let left = Box::new(Expr::Factor(Factor::Term(Term::Num(7))));
        let right = Factor::Term(Term::Num(4));
        let operation = Expr::Oper(left, OpKind::Add, right);
        let actual = evaluate_expr(operation);

        if let Term::Num(actual) = actual {
            assert_eq!(11, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_evaluates_add_with_3_terms() {
        let left = Expr::Factor(Factor::Term(Term::Num(3)));
        let middle = Factor::Term(Term::Num(5));
        let right = Factor::Term(Term::Num(4));
        let operation_a = Expr::Oper(Box::new(left), OpKind::Add, middle);
        let operation_b = Expr::Oper(Box::new(operation_a), OpKind::Add, right);
        let actual = evaluate_expr(operation_b);

        if let Term::Num(actual) = actual {
            assert_eq!(12, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_evaluates_sub() {
        let left = Expr::Factor(Factor::Term(Term::Num(5)));
        let right = Factor::Term(Term::Num(3));
        let operation = Expr::Oper(Box::new(left), OpKind::Sub, right);
        let actual = evaluate_expr(operation);

        if let Term::Num(actual) = actual {
            assert_eq!(2, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_evaluates_mult() {
        let left = Expr::Factor(Factor::Term(Term::Num(5)));
        let right = Factor::Term(Term::Num(3));
        let operation = Expr::Oper(Box::new(left), OpKind::Mult, right);
        let actual = evaluate_expr(operation);

        if let Term::Num(actual) = actual {
            assert_eq!(15, actual);
        } else {
            panic!();
        }
    }
}
