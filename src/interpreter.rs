use crate::ast::*;
use crate::scope::Scope;

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
        Stmt::Print(expr) => interpret_print(expr),
        Stmt::Declare(ident) => interpret_declare(ident, scope),
    }
}

fn interpret_print(expr: Expr) {
    let result = evaluate_expr(expr);
    println!("{}", result.to_string());
}

fn interpret_declare(ident: String, scope: &mut Scope) {
    scope.add_binding(ident, None);
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
        Factor::Mult(left, right) => evaluate_oper(evaluate_factor(*left), OpKind::Mult, right),
        Factor::Div(left, right) => evaluate_oper(evaluate_factor(*left), OpKind::Div, right),
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
                OpKind::Div => Term::Num(do_divide(left, right)),
            }
        } else {
            unimplemented!();
        }
    } else {
        unimplemented!();
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

    #[test]
    pub fn it_evaluates_div() {
        let left = Factor::Term(Term::Num(5.0));
        let right = Term::Num(2.0);
        let operation = Factor::Div(Box::new(left), right);
        let actual = evaluate_factor(operation);

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
        evaluate_factor(operation);
    }
}
