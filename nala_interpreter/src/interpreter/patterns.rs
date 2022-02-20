use crate::ast::patterns::*;

use super::*;

pub fn evaluate_is_pattern(
    is_pattern: &IsPattern,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Value, NalaRuntimeError> {
    let (expr, pattern) = match is_pattern {
        IsPattern::Literal(expr, pattern) => (expr, pattern.clone()),
        IsPattern::Symbol(expr, ident) => {
            let pattern = scopes.get_value(ident, current_scope, context)?;

            if let Value::Pattern(pattern) = pattern {
                (expr, pattern)
            } else {
                return Err(NalaRuntimeError {
                    message: format!(
                        "Expected pattern provided to pattern declaration, instead got {}",
                        0
                    ),
                });
            }
        }
    };

    let term = evaluate_expr(expr, scopes, current_scope, context)?;
    Ok(Value::Bool(check_is_pattern(&term, &pattern)))
}

fn check_is_pattern(value: &Value, pattern: &Pattern) -> bool {
    match pattern {
        Pattern::Enum(patt_enum_name, patt_variant, _) => {
            if let Value::Variant(enum_name, variant, _) = value {
                let enums_match = enum_name == patt_enum_name;
                let variant_match = variant == patt_variant;
                enums_match && variant_match
            } else {
                false
            }
        }
        Pattern::Capture(_capture) => true,
    }
}

pub fn evaluate_unwrap(
    unwrap: &Unwrap,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Value, NalaRuntimeError> {
    let (expr, pattern) = match unwrap {
        Unwrap::Literal(expr, pattern) => (expr, pattern.clone()),
        Unwrap::Symbol(expr, ident) => {
            let pattern = scopes.get_value(ident, current_scope, context)?;

            if let Value::Pattern(pattern) = pattern {
                (expr, pattern)
            } else {
                return Err(NalaRuntimeError {
                    message: format!(
                        "Expected pattern provided to pattern declaration, instead got {}",
                        0
                    ),
                });
            }
        }
    };

    let term = evaluate_expr(expr, scopes, current_scope, context)?;

    if !check_is_pattern(&term, &pattern) {
        return Err(NalaRuntimeError {
            message: format!("Expression does not match pattern."),
        });
    };

    unwrap_with_pattern(term, pattern)
}

fn unwrap_with_pattern(value: Value, pattern: Pattern) -> Result<Value, NalaRuntimeError> {
    match pattern {
        Pattern::Enum(_patt_enum_name, _patt_variant, patt_data) => {
            let (_enum_name, _variant, data) = value.unwrap_variant()?;

            if let Some(patt_data) = *patt_data {
                unwrap_with_pattern(*data.unwrap(), patt_data)
            } else {
                Ok(Value::Void)
            }
        }
        Pattern::Capture(capture) => {
            if let Capture::Capture = capture {
                Ok(value)
            } else {
                Ok(Value::Void)
            }
        }
    }
}
