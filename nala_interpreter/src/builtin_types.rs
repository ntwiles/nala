// TODO: Consider refacatoring so this is in the builtins directory.

use crate::ast::types::{
    primitive_type::PrimitiveType, type_literal::TypeLiteral,
    type_literal_variant::TypeVariantLiteral, variant_declare::VariantDeclare, StructLiteralField,
    StructLiteralFieldValue,
};

pub fn get_builtin_structs() -> Vec<(String, Option<String>, Vec<StructLiteralField>)> {
    vec![(
        String::from("HttpResult"),
        Some(String::from("T")),
        get_http_result_struct(),
    )]
}

pub fn get_builtin_enums() -> Vec<(String, Option<String>, Vec<VariantDeclare>)> {
    vec![(
        String::from("Option"),
        Some(String::from("T")),
        get_option_enum(),
    )]
}

fn get_option_enum() -> Vec<VariantDeclare> {
    vec![
        VariantDeclare::Data(
            String::from("Some"),
            TypeVariantLiteral::Type(TypeLiteral::UserDefined(String::from("T"))),
        ),
        VariantDeclare::Empty(String::from("None")),
    ]
}

// TODO: Make a helper function to simplify creating these structs.
fn get_http_result_struct() -> Vec<StructLiteralField> {
    vec![
        StructLiteralField {
            ident: String::from("statusCode"),
            value: StructLiteralFieldValue::Type(TypeVariantLiteral::Composite(
                TypeLiteral::UserDefined(String::from("Option")),
                vec![TypeVariantLiteral::Type(TypeLiteral::PrimitiveType(
                    PrimitiveType::String,
                ))],
            )),
        },
        StructLiteralField {
            ident: String::from("body"),
            value: StructLiteralFieldValue::Type(TypeVariantLiteral::Composite(
                TypeLiteral::UserDefined(String::from("Option")),
                vec![TypeVariantLiteral::Type(TypeLiteral::UserDefined(
                    String::from("T"),
                ))],
            )),
        },
    ]
}
