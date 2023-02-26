use std::fmt;

use crate::{
    ast::types::{primitive_type::PrimitiveType, type_literal::TypeLiteral},
    scopes::Scopes,
};

use self::struct_field::StructField;

pub mod struct_field;
pub mod type_variant;

#[derive(Eq, Debug, Clone)]
pub enum NalaType {
    Enum(String, String),
    PrimitiveType(PrimitiveType),
    Struct(Vec<StructField>),
}

impl NalaType {
    pub fn from_literal(literal: TypeLiteral, scopes: &mut Scopes, current_scope: usize) -> Self {
        match literal {
            TypeLiteral::PrimitiveType(t) => NalaType::PrimitiveType(t),
            TypeLiteral::UserDefined(ident) => {
                let binding = scopes
                    .get_type(&ident, current_scope)
                    .unwrap()
                    .as_struct()
                    .unwrap();

                NalaType::Struct(binding)
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
        match self {
            NalaType::Enum(_enum_ident, _variant_ident) => {
                todo!()
            }
            NalaType::PrimitiveType(sp) => {
                if let NalaType::PrimitiveType(op) = other {
                    sp.is_assignable_to(op)
                } else {
                    false
                }
            }
            NalaType::Struct(fields) => {
                if let NalaType::Struct(ot) = other {
                    for ot in ot.iter() {
                        if let Some(found) = fields.iter().find(|f| *f == ot) {
                            return found.field_type.is_assignable_to(&ot.field_type);
                        }
                    }

                    true
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
            NalaType::Enum(enum_ident, variant_ident) => todo!(),
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
            NalaType::Enum(_, _) => todo!(),
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
