use crate::ast::patterns::*;

use super::*;

pub fn evaluate_is_pattern(
    is_pattern: &IsPattern,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Term, NalaRuntimeError> {
    let (expr, pattern) = match is_pattern {
        IsPattern::Literal(expr, pattern) => (expr, pattern.clone()),
        IsPattern::Symbol(expr, ident) => {
            let pattern = scopes.get_value(ident, current_scope, context)?;

            if let Term::Pattern(pattern) = pattern {
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
    Ok(Term::Bool(check_is_pattern(&term, &pattern)))
}

fn check_is_pattern(term: &Term, pattern: &Pattern) -> bool {
    match pattern {
        Pattern::Enum(patt_enum_name, patt_variant, _) => {
            if let Term::Variant(enum_name, variant, _) = term {
                let enums_match = enum_name == patt_enum_name;
                let variant_match = variant == patt_variant;
                enums_match && variant_match
            } else {
                false
            }
        }
    }
}

pub fn evaluate_unwrap(
    unwrap: &Unwrap,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Term, NalaRuntimeError> {
    let (expr, pattern) = match unwrap {
        Unwrap::Literal(expr, pattern) => (expr, pattern.clone()),
        Unwrap::Symbol(expr, ident) => {
            let pattern = scopes.get_value(ident, current_scope, context)?;

            if let Term::Pattern(pattern) = pattern {
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

    match pattern {
        Pattern::Enum(_patt_enum_name, _patt_variant, patt_capture) => {
            if let Term::Variant(_enum_name, _variant, data) = term {
                if let Capture::Capture = patt_capture {
                    if let Some(data) = data {
                        Ok(*data)
                    } else {
                        Err(NalaRuntimeError {
                            message: "Nothing to capture!".to_string(),
                        })
                    }
                } else {
                    Ok(Term::Void)
                }
            } else {
                unreachable!()
            }
        }
    }
}
