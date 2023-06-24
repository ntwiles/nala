use std::fmt;

use crate::{
    ast::types::{
        primitive_type::PrimitiveType, type_literal::TypeLiteral,
        type_literal_variant::TypeVariantLiteral,
    },
    errors::RuntimeError,
    resolved::enum_variants::EnumVariant,
    scopes::Scopes,
    utils::accept_results,
};

use super::{composite_type::CompositeType, nala_type::NalaType};

#[derive(Eq, Debug, Clone)]
pub enum TypeVariant {
    Composite(CompositeType),
    Type(NalaType),
}

impl TypeVariant {
    pub fn generic(ident: String) -> Self {
        TypeVariant::Type(NalaType::Generic(ident))
    }

    pub fn find_generic_type_param(&self) -> Option<String> {
        match self {
            TypeVariant::Composite(CompositeType { inner, .. }) => inner
                .iter()
                .find(|i| i.find_generic_type_param().is_some())
                .map(|i| i.find_generic_type_param().unwrap()),
            TypeVariant::Type(t) => t.find_generic_type_param(),
        }
    }

    pub fn as_composite(self) -> Option<CompositeType> {
        match self {
            Self::Composite(composite) => Some(composite),
            _ => None,
        }
    }

    // TODO: Should this be returning a result? Maybe this should just be returning an Option as
    // above. If we're only calling this in places where we think we can safely assume that the
    // value is an enum, then we should be unwrapping and panicking rather than showing the user a
    // runtime error.
    pub fn as_enum(&self) -> Result<(Vec<EnumVariant>, Option<String>), RuntimeError> {
        match self {
            Self::Composite(composite) => {
                if let NalaType::Enum(_ident, variants) = &composite.outer {
                    Ok((variants.clone(), composite.generic_type_param.clone()))
                } else {
                    Err(RuntimeError::new("Expected an enum type."))
                }
            }
            Self::Type(NalaType::Enum(_ident, variants)) => Ok((variants.clone(), None)),
            _ => Err(RuntimeError::new("Expected an enum type.")),
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

    /**
     * TODO: This is really bad. This will error if called on a type that does not support composite
     * This has to be done because our TypeLiteral representing enum outer cannot be easily processed
     * into a NalaType like one might expect, but instead has to be processed into a TypeVariant. This
     * is because we could have a TypeLiteral::UserDefined() which will resolve into a TypeVariant (as
     * that's how types are currently stored in scope).
     *
     * A potential solution is to move UserDefined up one level so that it is a TypeLiteralVariant
     * in the AST instead of a TypeLiteral.
     */

    fn from_outer_literal_type(
        literal: TypeLiteral,
        inner: Vec<TypeVariant>,
        scopes: &mut Scopes,
        current_scope: usize,
    ) -> Result<Self, RuntimeError> {
        match literal.clone() {
            TypeLiteral::PrimitiveType(t) => Ok(match t {
                PrimitiveType::Array => TypeVariant::Composite(CompositeType {
                    outer: NalaType::PrimitiveType(PrimitiveType::Array),
                    inner,
                    generic_type_param: None,
                }),

                PrimitiveType::Break => {
                    TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Break))
                }
                PrimitiveType::Func => TypeVariant::Composite(CompositeType {
                    outer: NalaType::PrimitiveType(PrimitiveType::Func),
                    inner,
                    generic_type_param: None,
                }),
                _ => Err(RuntimeError::new(&format!(
                    "Type `{literal}` does not support type arguments. Type `{literal}<{}>` is invalid.",
                    inner[0]
                )))?,
            }),
            TypeLiteral::UserDefined(ident) => scopes.get_type(&ident, current_scope),
        }
    }

    fn from_literal_type(
        literal: TypeLiteral,
        inner: Vec<TypeVariant>,
        scopes: &mut Scopes,
        current_scope: usize,
    ) -> Result<Self, RuntimeError> {
        match literal {
            TypeLiteral::PrimitiveType(t) => Ok(match t {
                PrimitiveType::Array => TypeVariant::Composite(CompositeType {
                    outer: NalaType::PrimitiveType(PrimitiveType::Array),
                    inner,
                    generic_type_param: None,
                }),
                PrimitiveType::Bool => {
                    TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Bool))
                }
                PrimitiveType::Break => {
                    TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Break))
                }
                PrimitiveType::Func => TypeVariant::Composite(CompositeType {
                    outer: NalaType::PrimitiveType(PrimitiveType::Func),
                    inner,
                    generic_type_param: None,
                }),
                PrimitiveType::Number => {
                    TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number))
                }
                PrimitiveType::String => {
                    TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::String))
                }
                PrimitiveType::Void => {
                    TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Void))
                }
            }),
            TypeLiteral::UserDefined(ident) => scopes.get_type(&ident, current_scope),
        }
    }

    pub fn from_literal(
        literal: TypeVariantLiteral,
        scopes: &mut Scopes,
        current_scope: usize,
    ) -> Result<Self, RuntimeError> {
        match literal {
            TypeVariantLiteral::Composite(outer, inner) => {
                let inner = accept_results(
                    inner
                        .into_iter()
                        .map(|l| Self::from_literal(l, scopes, current_scope))
                        .collect(),
                )?;

                let outer =
                    Self::from_outer_literal_type(outer, inner.clone(), scopes, current_scope)?;

                let generic_type_param = outer.find_generic_type_param();

                let concrete_type = inner[0].clone();

                Ok(outer.make_concrete(generic_type_param, &concrete_type))
            }
            TypeVariantLiteral::Type(t) => {
                Self::from_literal_type(t, vec![], scopes, current_scope)
            }
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
