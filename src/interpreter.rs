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

// TODO: These types don't make sense. An expression doesn't evaluate to a literal.
fn evaluate_expr(expr: Expr) -> Literal {
    match expr {
        Expr::Literal(literal) => literal,
        Expr::Oper(left, op_kind, right) => evaluate_oper(left, op_kind, right),
    }
}

fn evaluate_oper(left: Literal, op_kind: OpKind, right: Literal) -> Literal {
    if let Literal::Num(left) = left {
        if let Literal::Num(right) = right {
            if let OpKind::Add = op_kind {
                Literal::Num(left + right)
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
