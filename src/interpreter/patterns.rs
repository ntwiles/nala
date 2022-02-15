use crate::ast::patterns::*;

use super::*;

pub fn evaluate_is_pattern(
    expr: &Expr,
    pattern: &Pattern,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Term, NalaRuntimeError> {
    let term = evaluate_expr(expr, scopes, current_scope, context)?;
    Ok(Term::Bool(is_pattern(&term, pattern)))
}

fn is_pattern(term: &Term, pattern: &Pattern) -> bool {
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
    expr: &Expr,
    pattern: &Pattern,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Term, NalaRuntimeError> {
    let term = evaluate_expr(expr, scopes, current_scope, context)?;

    if !is_pattern(&term, pattern) {
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
