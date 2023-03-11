use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use reqwest;
use serde_json;

use crate::{
    ast::{
        funcs::*,
        terms::*,
        types::{
            primitive_type::PrimitiveType, type_literal::TypeLiteral,
            type_literal_variant::TypeLiteralVariant,
        },
        *,
    },
    errors::RuntimeError,
    io_context::IoContext,
    scopes::{type_binding::TypeBinding, Scopes},
    types::{struct_field::StructField, type_variant::TypeVariant, NalaType},
};

pub fn get_http_block(scopes: &mut Scopes, scope: usize) -> Result<Func, RuntimeError> {
    // TODO: Once we have generics implemented we can let require that users pass the return type.
    let return_type = TypeLiteralVariant::Type(TypeLiteral::PrimitiveType(PrimitiveType::Any));

    let options_fields = vec![
        StructField {
            ident: String::from("method"),
            value: TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::String)),
        },
        StructField {
            ident: String::from("url"),
            value: TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::String)),
        },
        StructField {
            ident: String::from("body"),
            value: TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::String)),
        },
    ];

    scopes.add_type_binding("HttpOptions", scope, TypeBinding::Struct(options_fields))?;

    let params = vec![Param {
        ident: String::from("options"),
        param_type: TypeLiteralVariant::Type(TypeLiteral::UserDefined("HttpOptions".to_string())),
    }];

    Ok(Func {
        ident: "http".to_string(),
        params,
        return_type,
        block: Box::new(FuncVariant::Builtin(builtin_http)),
    })
}

fn builtin_http(
    args: HashMap<String, Value>,
    _context: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let options = args.get("options").unwrap();

    let mutex = if let Value::Object(reference) = options {
        Arc::clone(&reference)
    } else {
        unreachable!()
    };

    let options = mutex.lock().unwrap();

    let url = options["url"].as_string().unwrap();
    let method = options["method"].as_string().unwrap();

    let body = if let Some(Value::String(body)) = options.get("body") {
        Some(body)
    } else {
        None
    };

    let add_method = |client: reqwest::blocking::Client| match method.as_str() {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        _ => todo!(),
    };

    let client = add_method(reqwest::blocking::Client::new());

    let response = if let Some(body) = body {
        client.body(body.clone()).send()
    } else {
        client.send()
    };

    let mut fields = HashMap::<String, Value>::new();

    match response {
        Ok(response) => {
            fields.insert(
                String::from("statusCode"),
                Value::String(response.status().to_string()),
            );

            let value = response.json::<serde_json::Value>().unwrap();
            fields.insert(String::from("body"), build_value(value));
        }
        Err(error) => {
            // TODO: Status is optional because the error might not have been generated from a response.
            // Defaulting to an empty string probably isn't the best way to handle that case.
            fields.insert(
                String::from("statusCode"),
                Value::String(
                    error
                        .status()
                        .map(|code| code.to_string())
                        .unwrap_or("".to_string())
                        .to_string(),
                ),
            );
        }
    }

    Ok(Value::Object(Arc::new(Mutex::new(fields))))
}

fn build_value(value: serde_json::Value) -> Value {
    match value {
        serde_json::Value::Array(items) => Value::Array(Arc::new(Mutex::new(
            items.into_iter().map(build_value).collect::<Vec<Value>>(),
        ))),
        serde_json::Value::Null => Value::String(String::from("null")), // TODO: This is a placeholder until options are implemented. (These are now implemented but not yet builtin.)
        serde_json::Value::Bool(value) => Value::Bool(value),
        serde_json::Value::Number(_) => todo!(),
        serde_json::Value::String(value) => Value::String(value),
        serde_json::Value::Object(fields) => Value::Object(Arc::new(Mutex::new(
            fields
                .into_iter()
                .map(|(key, value)| (key, build_value(value)))
                .collect(),
        ))),
    }
}
