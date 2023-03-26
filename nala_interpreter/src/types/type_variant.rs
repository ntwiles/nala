use std::fmt;

use crate::{
    ast::types::type_literal_variant::TypeVariantLiteral, errors::RuntimeError,
    resolved::from_literal::FromLiteral, scopes::Scopes, utils::accept_results,
};

use super::{composite_type::CompositeType, NalaType};

#[derive(Eq, Debug, Clone)]
pub enum TypeVariant {
    Composite(CompositeType),
    Type(NalaType),
}

impl TypeVariant {
    pub fn get_generic_ident(&self) -> Option<String> {
        match self {
            TypeVariant::Composite(CompositeType { inner, .. }) => inner
                .iter()
                .find(|i| i.get_generic_ident().is_some())
                .map(|i| i.get_generic_ident().unwrap()),
            TypeVariant::Type(t) => t.get_generic_ident(),
        }
    }

    pub fn make_concrete(self, generic_ident: &str, concrete_type: &TypeVariant) -> Self {
        match self {
            TypeVariant::Composite(CompositeType { outer, inner, .. }) => {
                TypeVariant::Composite(CompositeType {
                    outer: outer.make_concrete(generic_ident, concrete_type),
                    inner: inner
                        .into_iter()
                        .map(|i| i.make_concrete(generic_ident, concrete_type))
                        .collect(),
                })
            }
            TypeVariant::Type(t) => {
                if let NalaType::Generic(ident) = t.clone() {
                    if ident == generic_ident {
                        return concrete_type.clone();
                    } else {
                        TypeVariant::Type(t)
                    }
                } else {
                    TypeVariant::Type(t)
                }
            }
        }
    }
}

impl FromLiteral<TypeVariantLiteral> for TypeVariant {
    fn from_literal(
        literal: TypeVariantLiteral,
        scopes: &mut Scopes,
        current_scope: usize,
    ) -> Result<Self, RuntimeError> {
        match literal {
            TypeVariantLiteral::Composite(outer, inner) => {
                let outer = NalaType::from_literal(outer, scopes, current_scope)?;

                let inner = accept_results(
                    inner
                        .into_iter()
                        .map(|l| TypeVariant::from_literal(l, scopes, current_scope))
                        .collect(),
                )?;

                let composite = if let Some(type_params) = outer.get_generic_ident() {
                    let concrete_type = inner[0].clone();

                    let composite = TypeVariant::Composite(CompositeType { outer, inner });
                    composite.make_concrete(&type_params, &concrete_type)
                } else {
                    TypeVariant::Composite(CompositeType { outer, inner })
                };

                Ok(composite)
            }
            TypeVariantLiteral::Type(t) => Ok(TypeVariant::Type(NalaType::from_literal(
                t,
                scopes,
                current_scope,
            )?)),
        }
    }
}

impl fmt::Display for TypeVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeVariant::Composite(CompositeType {
                outer: v,
                inner: vv,
                ..
            }) => {
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
            TypeVariant::Composite(CompositeType {
                inner: mv,
                outer: mg,
                ..
            }) => {
                if let TypeVariant::Composite(CompositeType {
                    inner: ov,
                    outer: og,
                    ..
                }) = other
                {
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
