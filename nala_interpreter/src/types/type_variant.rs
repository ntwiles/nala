use std::fmt;

use crate::{
    ast::types::type_literal_variant::TypeLiteralVariant, errors::RuntimeError, scopes::Scopes,
    utils::accept_results,
};

use super::NalaType;

#[derive(Eq, Debug, Clone)]
pub enum TypeVariant {
    Composite(NalaType, Vec<TypeVariant>),
    Type(NalaType),
}

impl TypeVariant {
    pub fn from_literal(
        literal: TypeLiteralVariant,
        scopes: &mut Scopes,
        current_scope: usize,
    ) -> Result<Self, RuntimeError> {
        match literal {
            TypeLiteralVariant::Composite(p, c) => {
                let variants = c
                    .into_iter()
                    .map(|l| TypeVariant::from_literal(l, scopes, current_scope))
                    .collect();

                let variants = accept_results(variants)?;

                Ok(TypeVariant::Composite(
                    NalaType::from_literal(p, scopes, current_scope)?,
                    variants,
                ))
            }
            TypeLiteralVariant::Type(t) => Ok(TypeVariant::Type(NalaType::from_literal(
                t,
                scopes,
                current_scope,
            )?)),
        }
    }

    pub fn get_generic_ident(&self) -> Option<String> {
        match self {
            TypeVariant::Composite(_outer, inner) => inner
                .iter()
                .find(|i| i.get_generic_ident().is_some())
                .map(|i| i.get_generic_ident().unwrap()),
            TypeVariant::Type(t) => t.get_generic_ident(),
        }
    }
}

impl fmt::Display for TypeVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeVariant::Composite(v, vv) => {
                let children = vv
                    .iter()
                    .map(|vv| vv.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                write!(f, "{0}<{1}>", v, children)
            }
            TypeVariant::Type(t) => write!(f, "{}", t),
        }
    }
}

impl PartialEq for TypeVariant {
    fn eq(&self, other: &Self) -> bool {
        match self {
            TypeVariant::Composite(mv, mg) => {
                if let TypeVariant::Composite(ov, og) = other {
                    mv == ov && mg == og
                } else {
                    false
                }
            }
            TypeVariant::Type(me) => {
                if let TypeVariant::Type(other) = other {
                    me == other
                } else {
                    false
                }
            }
        }
    }
}
