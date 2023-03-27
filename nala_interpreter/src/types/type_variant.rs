use std::fmt;

use crate::{
    ast::types::type_literal_variant::TypeVariantLiteral, errors::RuntimeError,
    resolved::from_literal::FromLiteral, scopes::Scopes, utils::accept_results,
};

use super::{composite_type::CompositeType, nala_type::NalaType};

#[derive(Eq, Debug, Clone)]
pub enum TypeVariant {
    Composite(CompositeType),
    Type(NalaType),
}

impl TypeVariant {
    pub fn find_generic_type_param(&self) -> Option<String> {
        match self {
            TypeVariant::Composite(CompositeType { inner, .. }) => inner
                .iter()
                .find(|i| i.find_generic_type_param().is_some())
                .map(|i| i.find_generic_type_param().unwrap()),
            TypeVariant::Type(t) => t.find_generic_type_param(),
        }
    }

    pub fn make_concrete(self, generic_ident: Option<String>, concrete_type: &TypeVariant) -> Self {
        if let Some(generic_ident) = generic_ident {
            match self {
                TypeVariant::Composite(CompositeType { outer, inner, .. }) => {
                    TypeVariant::Composite(CompositeType {
                        outer: outer.make_concrete(Some(generic_ident.clone()), concrete_type),
                        inner: inner
                            .into_iter()
                            .map(|i| i.make_concrete(Some(generic_ident.clone()), concrete_type))
                            .collect(),
                        generic_type_param: None,
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
        } else {
            self
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

                let generic_type_param = outer.find_generic_type_param();

                let concrete_type = inner[0].clone();

                let composite = TypeVariant::Composite(CompositeType {
                    outer,
                    inner,
                    generic_type_param: generic_type_param.clone(),
                });

                Ok(composite.make_concrete(generic_type_param, &concrete_type))
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
