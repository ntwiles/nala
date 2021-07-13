use crate::{ast::*, scope::Scope};

pub fn interpret_tree(program: Program, scope: &mut Scope) {
    match program {
        Program::Stmt(stmt) => interpret_stmt(stmt, scope),
        Program::Stmts(prog, stmt) => {
            interpret_tree(*prog, scope);
            interpret_stmt(stmt, scope);
        }
    }
}

fn interpret_stmt(stmt: Stmt, scope: &mut Scope) {
    match stmt {
        Stmt::Print(expr) => interpret_print(expr, scope),
        Stmt::Declare(ident, expr) => interpret_declare(ident, expr, scope),
    }
}

fn interpret_print(expr: Expr, scope: &mut Scope) {
    let result = evaluate_expr(expr, scope);

    if let Term::Symbol(ident) = result {
        println!("{}", scope.get_value(ident).to_string());
    } else {
        println!("{}", result.to_string());
    }
}

fn interpret_declare(ident: String, expr: Expr, scope: &mut Scope) {
    let value = evaluate_expr(expr, scope);
    scope.add_binding(ident, value);
}

fn evaluate_expr(expr: Expr, scope: &mut Scope) -> Term {
    match expr {
        Expr::Add(left, right) => {
            let left = evaluate_expr(*left, scope);
            let right = evaluate_factor(right, scope);
            evaluate_oper(left, OpKind::Add, right, scope)
        }
        Expr::Sub(left, right) => {
            let left = evaluate_expr(*left, scope);
            let right = evaluate_factor(right, scope);
            evaluate_oper(left, OpKind::Sub, right, scope)
        }
        Expr::Factor(factor) => evaluate_factor(factor, scope),
    }
}

fn evaluate_factor(factor: Factor, scope: &mut Scope) -> Term {
    match factor {
        Factor::Mult(left, right) => {
            evaluate_oper(evaluate_factor(*left, scope), OpKind::Mult, right, scope)
        }
        Factor::Div(left, right) => {
            evaluate_oper(evaluate_factor(*left, scope), OpKind::Div, right, scope)
        }
        Factor::Term(term) => term,
    }
}

// TODO: Can this be simplified?
fn evaluate_oper(left: Term, op_kind: OpKind, right: Term, scope: &mut Scope) -> Term {
    match left {
        Term::Num(left) => match right {
            Term::Num(right) => match op_kind {
                OpKind::Add => Term::Num(left + right),
                OpKind::Sub => Term::Num(left - right),
                OpKind::Mult => Term::Num(left * right),
                OpKind::Div => Term::Num(do_divide(left, right)),
            },
            Term::String(_) => {
                panic!("Cannot perform operations between types Num and String.")
            }
            Term::Symbol(right) => {
                let right = scope.get_value(right.to_owned());
                evaluate_oper(Term::Num(left), op_kind, right, scope)
            }
        },
        Term::String(left) => match right {
            Term::Num(_) => panic!("Cannot perform operations types Num and String."),
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
                let right = scope.get_value(right.to_owned());
                evaluate_oper(Term::String(left), op_kind, right, scope)
            }
        },
        Term::Symbol(left) => {
            let left = scope.get_value(left.to_owned());
            evaluate_oper(left, op_kind, right, scope)
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

    #[test]
    pub fn it_evaluates_add_with_2_terms() {
        let left = Box::new(Expr::Factor(Factor::Term(Term::Num(7.0))));
        let right = Factor::Term(Term::Num(4.0));
        let operation = Expr::Add(left, right);
        let actual = evaluate_expr(operation, &mut Scope::new(None));

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
        let actual = evaluate_expr(operation_b, &mut Scope::new(None));

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
        let actual = evaluate_expr(operation, &mut Scope::new(None));

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
        let actual = evaluate_factor(operation, &mut Scope::new(None));

        if let Term::Num(actual) = actual {
            assert_eq!(15.0, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_evaluates_div() {
        let left = Factor::Term(Term::Num(5.0));
        let right = Term::Num(2.0);
        let operation = Factor::Div(Box::new(left), right);
        let actual = evaluate_factor(operation, &mut Scope::new(None));

        if let Term::Num(actual) = actual {
            assert_eq!(2.5, actual);
        } else {
            panic!();
        }
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero.")]
    pub fn it_disallows_div_by_zero() {
        let left = Factor::Term(Term::Num(5.0));
        let right = Term::Num(0.0);
        let operation = Factor::Div(Box::new(left), right);
        evaluate_factor(operation, &mut Scope::new(None));
    }
}
