use std::usize;

use crate::{
    ast::*,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn interpret_tree(program: Program, context: &mut impl IoContext) {
    let mut scopes = Scopes::new();
    let top_scope = scopes.new_scope(None);
    
    match program {
        Program::Block(block) => interpret_block(&block, &mut scopes, top_scope, context),
        Program::Stmts(stmts) => interpret_stmts(&stmts, &mut scopes, top_scope, context),
    }
}

fn interpret_block(
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) {
    let block_scope = scopes.new_scope(Some(current_scope));
    interpret_stmts(&block.stmts, scopes, block_scope, context);
}

fn interpret_stmts(
    stmts: &Stmts,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) {
    match stmts {
        Stmts::Stmts(stmts, stmt) => {
            interpret_stmts(&*stmts, scopes, current_scope, context);
            interpret_stmt(stmt, scopes, current_scope, context);
        }
        Stmts::Stmt(stmt) => interpret_stmt(stmt, scopes, current_scope, context),
    }
}

fn interpret_stmt(
    stmt: &Stmt,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) {
    match stmt {
        Stmt::Print(expr) => interpret_print(expr, scopes, current_scope, context),
        Stmt::Declare(ident, expr, is_mutable) => interpret_declare(
            ident,
            expr,
            scopes,
            current_scope,
            context,
            is_mutable.clone(),
        ),
        Stmt::Assign(ident, expr) => interpret_assign(ident, expr, scopes, current_scope, context),
        Stmt::If(cond, block) => interpret_if(cond, block, scopes, current_scope, context),
        Stmt::For(ident, expr, block) => {
            interpret_for(ident, &expr, block, scopes, current_scope, context)
        },
        Stmt::Func(ident, block) => interpret_func(ident, block, scopes, current_scope, context),
    }
}

fn interpret_print(
    expr: &Expr,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) {
    let result = evaluate_expr(&expr, scopes, current_scope, context);

    if let Term::Symbol(ident) = result {
        context.print(&scopes.get_value(&ident, current_scope).to_string());
    } else {
        context.print(&result.to_string());
    }
}

fn interpret_declare(
    ident: &String,
    expr: &Expr,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
    is_mutable: bool,
) {
    if scopes.binding_exists_local(&ident, current_scope) {
        panic!("Binding for {} already exists in local scope.", ident);
    } else {
        let value = evaluate_expr(&expr, scopes, current_scope, context);
        scopes.add_binding(&ident, current_scope, value, is_mutable);
    }
}

fn interpret_assign(
    ident: &String,
    expr: &Expr,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) {
    if scopes.binding_exists(&ident, current_scope) {
        let value = evaluate_expr(&expr, scopes, current_scope, context);
        scopes.mutate_value(&ident, current_scope, value);
    } else {
        panic!("Unknown identifier `{}`", ident);
    }
}

fn interpret_if(
    cond: &Expr,
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) {
    let resolved = evaluate_expr(&cond, scopes, current_scope, context);

    if let Term::Bool(bool) = resolved {
        if bool {
            interpret_block(&block, scopes, current_scope, context)
        }
    } else {
        panic!("Cannot use non-boolean expressions inside 'if' conditions.")
    }
}

fn interpret_for(
    ident: &String,
    expr: &Expr,
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) {
    let resolved = evaluate_expr(expr, scopes, current_scope, context);

    if let Term::Array(array) = resolved {
        for (_, item) in array.iter().enumerate() {
            let block_scope = scopes.new_scope(Some(current_scope));
            scopes.add_binding(ident, block_scope, item.clone(), false);
            interpret_block(&block, scopes, block_scope, context);
        }
    } else {
        panic!(
            "Cannot iterate over values of non-Array types. Found '{}' of type {:?}",
            ident, resolved
        )
    }
}

fn interpret_func(
    ident: &String,
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) {
    if scopes.binding_exists_local(&ident, current_scope) {
        panic!("Binding for {} already exists in local scope.", ident);
    } else {
        scopes.add_binding(&ident, current_scope, Term::Func(Box::new(block.clone())), false);
    }
}

fn evaluate_expr(
    expr: &Expr,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    match expr {
        Expr::Eq(left, right) => {
            let left = evaluate_expr(left, scopes, current_scope, context);
            let right = evaluate_addend(right, scopes, current_scope);
            evaluate_equals(left, right, scopes, current_scope)
        }
        Expr::Gt(left, right) => {
            let left = evaluate_expr(left, scopes, current_scope, context);
            let right = evaluate_addend(right, scopes, current_scope);
            evaluate_gt(left, right, scopes, current_scope)
        }
        Expr::Lt(left, right) => {
            let left = evaluate_expr(left, scopes, current_scope, context);
            let right = evaluate_addend(right, scopes, current_scope);
            evaluate_lt(left, right, scopes, current_scope)
        }
        Expr::Addend(addend) => evaluate_addend(addend, scopes, current_scope),
        Expr::Array(elems) => evaluate_array(elems, scopes, current_scope, context),
        Expr::Index(ident, expr) => evaluate_index(ident, expr, scopes, current_scope, context),
        Expr::Read => evaluate_read(context),
        Expr::ReadNum => evaluate_readnum(context),
    }
}

fn evaluate_read(context: &mut impl IoContext) -> Term {
    let input = context.read();
    Term::String(input.trim().to_string())
}

fn evaluate_readnum(context: &mut impl IoContext) -> Term {
    let mut input = context.read();
    input = input.trim().to_string();
    let result = input.parse::<f32>();
    match result {
        Ok(num) => Term::Num(num),
        Err(_) => panic!("Could not parse input '{}' as type Num.", input),
    }
}

fn evaluate_array(
    array: &Array,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    let elems = &*array.elems;
    let terms = evaluate_elems(elems, scopes, current_scope, context);
    Term::Array(terms)
}

fn evaluate_elems(
    elems: &Elems,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Vec<Term> {
    match elems {
        Elems::Elems(elems, expr) => {
            let mut elems = evaluate_elems(elems, scopes, current_scope, context);
            elems.push(evaluate_expr(&expr, scopes, current_scope, context));
            elems
        }
        Elems::Expr(expr) => vec![evaluate_expr(&expr, scopes, current_scope, context)],
    }
}

fn evaluate_index(
    ident: &str,
    expr: &Expr,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    let index = evaluate_expr(expr, scopes, current_scope, context);

    if let Term::Num(index) = index {
        let array = scopes.get_value(ident, current_scope);
        // TODO: Check that this cast is safe first.
        let index = index as usize;

        if let Term::Array(array) = array {
            array.get(index).unwrap().clone()
        } else {
            panic!("Cannot index into a value which is not an array.");
        }
    } else {
        panic!("Cannot index using non-numeric value.");
    }
}

fn evaluate_addend(addend: &Addend, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    match addend {
        Addend::Add(left, right) => {
            let left = evaluate_addend(left, scopes, current_scope);
            let right = evaluate_factor(right, scopes, current_scope);
            evaluate_oper(left, OpKind::Add, right, scopes, current_scope)
        }
        Addend::Sub(left, right) => {
            let left = evaluate_addend(left, scopes, current_scope);
            let right = evaluate_factor(right, scopes, current_scope);
            evaluate_oper(left, OpKind::Sub, right, scopes, current_scope)
        }
        Addend::Factor(factor) => evaluate_factor(factor, scopes, current_scope),
    }
}

fn evaluate_factor(factor: &Factor, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    match factor {
        Factor::Mult(left, right) => evaluate_oper(
            evaluate_factor(left, scopes, current_scope),
            OpKind::Mult,
            right.clone(),
            scopes,
            current_scope,
        ),
        Factor::Div(left, right) => evaluate_oper(
            evaluate_factor(left, scopes, current_scope),
            OpKind::Div,
            right.clone(),
            scopes,
            current_scope,
        ),
        Factor::Term(term) => {
            if let Term::Symbol(ident) = term {
                scopes.get_value(ident, current_scope)
            } else {
                term.clone()
            }
        }
    }
}

fn evaluate_equals(left: Term, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    match left {
        Term::Num(left) => match right {
            Term::Num(right) => Term::Bool(left == right),
            Term::String(_) => panic!("Cannot perform comparisons between types Num and String."),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_equals(Term::Num(left), right, scopes, current_scope)
            }
            Term::Bool(_) => panic!("Cannot perform comparisons between types Num and Bool."),
            Term::Array(_) => panic!("Cannot perform comparisons between types Num and Array."),
            Term::Func(_) => panic!("Cannot perform comparisons between types Num and Func.")
        },
        Term::String(left) => match right {
            Term::Num(_) => panic!("Cannot perform comparisons between types String and Num."),
            Term::String(right) => Term::Bool(left == right),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_equals(Term::String(left), right, scopes, current_scope)
            }
            Term::Bool(_) => panic!("Cannot perform comparisons between types String and Bool."),
            Term::Array(_) => panic!("Cannot perform comparisons between types String and Array."),
            Term::Func(_) => panic!("Cannot perform comparisons between types String and Func."),
        },
        Term::Symbol(left) => {
            let left = scopes.get_value(&left, current_scope);
            evaluate_equals(left, right, scopes, current_scope)
        }
        Term::Bool(left) => match right {
            Term::Num(_) => panic!("Cannot perform comparisons between types Bool and Num."),
            Term::String(_) => panic!("Cannot perform comparisons between types Bool and String."),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_equals(Term::Bool(left), right, scopes, current_scope)
            }
            Term::Bool(right) => Term::Bool(left == right),
            Term::Array(_) => panic!("Cannot perform comparisons between types Bool and Array."),
            Term::Func(_) => panic!("Cannot perform comparisons between types Bool and Func.")
        },
        Term::Array(_) => panic!("Cannot perform comparions against values of type Array."),
        Term::Func(_) => panic!("Cannot perform comparisons against values of type Func.")
    }
}

fn evaluate_gt(left: Term, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    match left {
        Term::Num(left) => match right {
            Term::Num(right) => Term::Bool(left > right),
            Term::String(_) => panic!("Cannot perform comparisons between types Num and String."),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_gt(Term::Num(left), right, scopes, current_scope)
            }
            Term::Bool(_) => panic!("Cannot perform comparisons between types Num and Bool."),
            Term::Array(_) => panic!("Cannot perform comparisons between types Num and Array."),
            Term::Func(_) => panic!("Cannot perform comparisons between types Num and Func."),
        },
        Term::String(left) => match right {
            Term::Num(_) => panic!("Cannot perform comparisons between types String and Num."),
            Term::String(right) => Term::Bool(left > right),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_gt(Term::String(left), right, scopes, current_scope)
            }
            Term::Bool(_) => panic!("Cannot perform comparisons between types String and Bool."),
            Term::Array(_) => panic!("Cannot perform comparisons between types String and Array."),
            Term::Func(_) => panic!("Cannot perform comparisons between types String and Func."),
        },
        Term::Symbol(left) => {
            let left = scopes.get_value(&left, current_scope);
            evaluate_gt(left, right, scopes, current_scope)
        }
        Term::Bool(left) => match right {
            Term::Num(_) => panic!("Cannot perform comparisons between types Bool and Num."),
            Term::String(_) => panic!("Cannot perform comparisons between types Bool and String."),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_gt(Term::Bool(left), right, scopes, current_scope)
            }
            Term::Bool(right) => Term::Bool(left > right),
            Term::Array(_) => panic!("Cannot perform comparisons between types Bool and Array."),
            Term::Func(_) => panic!("Cannot perform comparisons between types Bool and Func.")
        },
        Term::Array(_) => panic!("Cannot perform comparions against values of type Array."),
        Term::Func(_) => panic!("Cannot perform comparisons against values of type Func."),
    }
}

fn evaluate_lt(left: Term, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    match left {
        Term::Num(left) => match right {
            Term::Num(right) => Term::Bool(left < right),
            Term::String(_) => panic!("Cannot perform comparisons between types Num and String."),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_lt(Term::Num(left), right, scopes, current_scope)
            }
            Term::Bool(_) => panic!("Cannot perform comparisons between types Num and Bool."),
            Term::Array(_) => panic!("Cannot perform comparisons between types Num and Array."),
            Term::Func(_) => panic!("Cannot perform comparisons between types Num and Func.")
        },
        Term::String(left) => match right {
            Term::Num(_) => panic!("Cannot perform comparisons between types String and Num."),
            Term::String(right) => Term::Bool(left < right),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_lt(Term::String(left), right, scopes, current_scope)
            }
            Term::Bool(_) => panic!("Cannot perform comparisons between types String and Bool."),
            Term::Array(_) => panic!("Cannot perform comparisons between types String and Array."),
            Term::Func(_) => panic!("Cannot perform comparisons between types String and Func.")
        },
        Term::Symbol(left) => {
            let left = scopes.get_value(&left, current_scope);
            evaluate_lt(left, right, scopes, current_scope)
        }
        Term::Bool(left) => match right {
            Term::Num(_) => panic!("Cannot perform comparisons between types Bool and Num."),
            Term::String(_) => panic!("Cannot perform comparisons between types Bool and String."),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_lt(Term::Bool(left), right, scopes, current_scope)
            }
            Term::Bool(right) => Term::Bool(left < right),
            Term::Array(_) => panic!("Cannot perform comparisons between types Bool and Array."),
            Term::Func(_) => panic!("Cannot perform comparisons between types Bool and Func.")
        },
        Term::Array(_) => panic!("Cannot perform comparisons against values of type Array."),
        Term::Func(_) => panic!("Cannot perform comparisons against values of type Func.")
    }
}

// TODO: Can this be simplified?
fn evaluate_oper(
    left: Term,
    op_kind: OpKind,
    right: Term,
    scopes: &mut Scopes,
    current_scope: ScopeId,
) -> Term {
    match left {
        Term::Num(left) => match right {
            Term::Num(right) => match op_kind {
                OpKind::Add => Term::Num(left + right),
                OpKind::Sub => Term::Num(left - right),
                OpKind::Mult => Term::Num(left * right),
                OpKind::Div => Term::Num(do_divide(left, right)),
            },
            Term::String(str) => {
                if let OpKind::Add = op_kind {
                    Term::String(left.to_string() + &str)
                } else {
                    panic!(
                        "Operation not supported between types Num and String: {:?}",
                        op_kind
                    )
                }
            }
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_oper(Term::Num(left), op_kind, right, scopes, current_scope)
            }
            Term::Bool(_) => {
                panic!("Cannot perform arithmetic operations between types of Num and Bool.")
            }
            Term::Array(_) => {
                panic!("Cannot perform arithmetic operations between types Num and Array.")
            }
            Term::Func(_) => {
                panic!("Cannot perform arithmetic operations between types Num and Func.")
            }
        },
        Term::String(left) => match right {
            Term::Num(num) => {
                if let OpKind::Add = op_kind {
                    Term::String(left + &num.to_string())
                } else {
                    panic!(
                        "Operation not supported between values of type String and Num: {:?}",
                        op_kind
                    )
                }
            }
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
                let right = scopes.get_value(&right, current_scope);
                evaluate_oper(Term::String(left), op_kind, right, scopes, current_scope)
            }
            Term::Bool(_) => {
                panic!("Cannot perform arithmetic operations between types String and Bool.")
            }
            Term::Array(_) => {
                panic!("Cannot perform arithmetic operations between types String and Array.")
            }
            Term::Func(_) => {
                panic!("Cannot perform arithmetic operations between types String and Func.")
            }
        },
        Term::Symbol(left) => {
            let left = scopes.get_value(&left, current_scope);
            evaluate_oper(left, op_kind, right, scopes, current_scope)
        }
        Term::Bool(_) => {
            panic!("Cannot perform arithmetic operations between values of type Bool.")
        }
        Term::Array(_) => {
            panic!("Cannot perform arithmetic operations between values of type Array.")
        }
        Term::Func(_) => {
            panic!("Cannot perform arithmetic operations between values of type Func.")
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
        let left = Box::new(Addend::Factor(Factor::Term(Term::Num(7.0))));
        let right = Factor::Term(Term::Num(4.0));
        let operation = Addend::Add(left, right);
        let mut scopes = Scopes::new();
        let top_scope = scopes.new_scope(None);
        let actual = evaluate_addend(&operation, &mut scopes, top_scope);

        if let Term::Num(actual) = actual {
            assert_eq!(11.0, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_evaluates_add_with_3_terms() {
        let left = Addend::Factor(Factor::Term(Term::Num(3.0)));
        let middle = Factor::Term(Term::Num(5.0));
        let right = Factor::Term(Term::Num(4.0));
        let operation_a = Addend::Add(Box::new(left), middle);
        let operation_b = Addend::Add(Box::new(operation_a), right);
        let mut scopes = Scopes::new();
        let top_scope = scopes.new_scope(None);
        let actual = evaluate_addend(&operation_b, &mut scopes, top_scope);

        if let Term::Num(actual) = actual {
            assert_eq!(12.0, actual);
        } else {
            panic!();
        }
    }

    #[test]
    pub fn it_evaluates_sub() {
        let left = Addend::Factor(Factor::Term(Term::Num(5.0)));
        let right = Factor::Term(Term::Num(3.0));
        let operation = Addend::Sub(Box::new(left), right);
        let mut scopes = Scopes::new();
        let top_scope = scopes.new_scope(None);
        let actual = evaluate_addend(&operation, &mut scopes, top_scope);

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
        let mut scopes = Scopes::new();
        let top_scope = scopes.new_scope(None);
        let actual = evaluate_factor(&operation, &mut scopes, top_scope);

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
        let mut scopes = Scopes::new();
        let top_scope = scopes.new_scope(None);
        let actual = evaluate_factor(&operation, &mut scopes, top_scope);

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
        let mut scopes = Scopes::new();
        let top_scope = scopes.new_scope(None);
        evaluate_factor(&operation, &mut scopes, top_scope);
    }
}
