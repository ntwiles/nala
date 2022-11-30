use std::fmt;

use crate::{
    ast::types::{primitive_type::PrimitiveType, type_literal::TypeLiteral},
    scope::{ScopeId, Scopes},
    utils::intersection,
};

use self::struct_field::StructField;

pub mod struct_field;
pub mod type_variant;

#[derive(Eq, Debug, Clone)]
pub enum NalaType {
    PrimitiveType(PrimitiveType),
    Struct(Vec<StructField>),
}

impl NalaType {
    pub fn from_literal(literal: TypeLiteral, scopes: &mut Scopes, current_scope: ScopeId) -> Self {
        match literal {
            TypeLiteral::PrimitiveType(t) => NalaType::PrimitiveType(t),
            TypeLiteral::UserDefined(ident) => {
                NalaType::Struct(scopes.get_struct(&ident, current_scope).unwrap())
            }
            TypeLiteral::Struct(fields) => NalaType::Struct(
                fields
                    .into_iter()
                    .map(|f| StructField::from_literal(f, scopes, current_scope))
                    .collect(),
            ),
        }
    }

    pub fn is_assignable_to(&self, other: &Self) -> bool {
        // println!(
        //     "Checking assignable: {}, {}",
        //     self.to_string(),
        //     other.to_string()
        // );
        match self {
            NalaType::PrimitiveType(sp) => {
                if let NalaType::PrimitiveType(op) = other {
                    sp.is_assignable_to(op)
                } else {
                    false
                }
            }
            NalaType::Struct(fields) => {
                if let NalaType::Struct(ot) = other {
                    intersection(fields, ot).len() == ot.len()
                } else {
                    false
                }
            }
        }
    }
}

impl fmt::Display for NalaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NalaType::PrimitiveType(primitive) => write!(f, "{}", primitive),
            NalaType::Struct(fields) => {
                write!(f, "{{ ")?;

                write!(
                    f,
                    "{}",
                    fields
                        .iter()
                        .map(|field| format!("{}: {}", field.ident, field.field_type.to_string()))
                        .fold(String::new(), |a, b| a + &b + ", ")
                )?;

                write!(f, "}}")?;

                Ok(())
            }
        }
    }
}

impl PartialEq for NalaType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            NalaType::PrimitiveType(sp) => {
                if let NalaType::PrimitiveType(op) = other {
                    sp == op
                } else {
                    false
                }
            }
            NalaType::Struct(fields) => {
                if let NalaType::Struct(of) = other {
                    // TODO: Can this be done without cloning?
                    let mut fields = fields.clone();
                    let mut of = of.clone();

                    fields.sort();
                    of.sort();

                    fields == of
                } else {
                    false
                }
            }
        }
    }
}
