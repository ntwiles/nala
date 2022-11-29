use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use reqwest;

use crate::{
    ast::{
        funcs::*,
        terms::*,
        types::{
            primitive_type::PrimitiveType, type_literal::TypeLiteral,
            type_literal_variant::TypeLiteralVariant, StructLiteralField,
        },
        *,
    },
    io_context::IoContext,
};

pub fn get_http_block() -> Func {
    let options_fields = vec![
        StructLiteralField::new(
            "method",
            TypeLiteralVariant::Type(TypeLiteral::PrimitiveType(PrimitiveType::String)),
        ),
        StructLiteralField::new(
            "url",
            TypeLiteralVariant::Type(TypeLiteral::PrimitiveType(PrimitiveType::String)),
        ),
    ];

    let params = vec![Param {
        ident: String::from("options"),
        param_type: TypeLiteralVariant::Type(TypeLiteral::Struct(options_fields)),
    }];

    Func {
        ident: "http".to_string(),
        params,
        block: Box::new(Block::RustBlock(builtin_http)),
    }
}

fn builtin_http(args: HashMap<String, Value>, _context: &mut dyn IoContext) -> Value {
    let options = args.get("options").unwrap();

    let mutex = if let Value::Object(reference) = options {
        Arc::clone(&reference)
    } else {
        unreachable!()
    };

    let options = mutex.lock().unwrap();

    let url = options["url"].unwrap_string();
    let method = options["method"].unwrap_string();

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

    Value::Object(Arc::new(Mutex::new(fields)))
}
