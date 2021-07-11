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
        Expr::Term(term) => term,
        Expr::Oper(left, op_kind, right) => evaluate_oper(left, op_kind, right),
    }
}

fn evaluate_oper(left: Box<Expr>, op_kind: OpKind, right: Term) -> Term {
    let left = evaluate_expr(*left);

    if let Term::Num(left) = left {
        if let Term::Num(right) = right {
            if let OpKind::Add = op_kind {
                Term::Num(left + right)
            } else {
                unimplemented!();
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
    pub fn it_evaluates_addition_with_2_terms() {
        let left = Box::new(Expr::Term(Term::Num(7)));
        let right = Term::Num(4);
        let operation = Expr::Oper(left, OpKind::Add, right);
        let actual = evaluate_expr(operation);

        if let Term::Num(actual) = actual {
            assert_eq!(11, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_evaluates_addition_with_3_terms() {
        let left = Expr::Term(Term::Num(3));
        let middle = Term::Num(5);
        let right = Term::Num(4);
        let operation_a = Expr::Oper(Box::new(left), OpKind::Add, middle);
        let operation_b = Expr::Oper(Box::new(operation_a), OpKind::Add, right);
        let actual = evaluate_expr(operation_b);

        if let Term::Num(actual) = actual {
            assert_eq!(12, actual);
        } else {
            panic!();
        }
    }
}
