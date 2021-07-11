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

fn evaluate_oper(left: Term, op_kind: OpKind, right: Term) -> Term {
    if let Term::Num(left) = left {
        if let Term::Num(right) = right {
            Term::Num(left + right)
        } else {
            unimplemented!();
        }
    } else {
        unimplemented!();
    }
}
