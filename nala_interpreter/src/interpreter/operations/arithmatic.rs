use crate::{ast::terms::Value, errors::RuntimeError, scopes::Scopes};

pub fn do_add(
    left: Value,
    right: Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    if left.get_type(scopes, current_scope)? != right.get_type(scopes, current_scope)? {
        return Err(RuntimeError::new(
            "Cannot add between values of two different types.",
        ));
    }

    match left {
        Value::Num(left) => {
            if let Value::Num(right) = right {
                Ok(Value::Num(left + right))
            } else {
                unreachable!()
            }
        }
        Value::String(left) => {
            if let Value::String(right) = right {
                Ok(Value::String(left + &right))
            } else {
                unreachable!()
            }
        }
        _ => unreachable!(),
    }
}

pub fn do_subtract(
    left: Value,
    right: Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    if left.get_type(scopes, current_scope)? != right.get_type(scopes, current_scope)? {
        return Err(RuntimeError::new(
            "Cannot subtract between values of two different types.",
        ));
    }

    if let Value::Num(left) = left {
        if let Value::Num(right) = right {
            Ok(Value::Num(left - right))
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

pub fn do_multiply(
    left: Value,
    right: Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    if left.get_type(scopes, current_scope)? != right.get_type(scopes, current_scope)? {
        return Err(RuntimeError::new(
            "Cannot multiply between values of two different types.",
        ));
    }

    if let Value::Num(left) = left {
        if let Value::Num(right) = right {
            Ok(Value::Num(left * right))
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

pub fn do_divide(
    left: Value,
    right: Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    if left.get_type(scopes, current_scope)? != right.get_type(scopes, current_scope)? {
        return Err(RuntimeError::new(
            "Cannot divide between values of two different types.",
        ));
    }

    if let Value::Num(left) = left {
        if let Value::Num(right) = right {
            if right != 0.0 {
                Ok(Value::Num(left / right))
            } else {
                Err(RuntimeError::new("Cannot divide by zero."))
            }
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}
