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

    match pattern {
        Pattern::Enum(patt_enum_name, patt_variant) => {
            if let Term::Variant(enum_name, variant, _) = term {
                let enums_match = &enum_name == patt_enum_name;
                let variant_match = &variant == patt_variant;
                Ok(Term::Bool(variant_match))
            } else {
                Ok(Term::Bool(false))
            }
        }
    }
}
