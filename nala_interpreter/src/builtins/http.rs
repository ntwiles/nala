use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use reqwest;
use serde_json;

use crate::{
    ast::{types::primitive_type::PrimitiveType, *},
    errors::RuntimeError,
    io_context::IoContext,
    resolved::{
        func_value::{FuncValue, Param},
        struct_field::StructField,
        value::{EnumVariantValue, Value},
    },
    types::{nala_type::NalaType, type_variant::TypeVariant},
};

pub fn get_http_block() -> FuncValue {
    let return_type = TypeVariant::Type(NalaType::Generic(String::from("T")));

    let options_fields = vec![
        StructField {
            ident: String::from("method"),
            value_type: TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::String)),
        },
        StructField {
            ident: String::from("url"),
            value_type: TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::String)),
        },
        StructField {
            ident: String::from("body"),
            value_type: TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::String)),
        },
    ];

    let params = vec![Param {
        ident: String::from("options"),
        param_type: TypeVariant::Type(NalaType::Struct(options_fields)),
    }];

    FuncValue {
        params,
        return_type,
        type_param: None,
        closure_scope: 0,
        block: Box::new(FuncVariant::Builtin(builtin_http)),
    }
}

fn builtin_http(
    args: HashMap<String, Value>,
    _ctx: &mut dyn IoContext,
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

    let fields = match response {
        Ok(response) => {
            let status_code = build_some(Value::String(response.status().to_string()));

            let body = match response.json::<serde_json::Value>() {
                Ok(value) => build_some(build_value(value)),
                Err(_) => build_none(),
            };

            HashMap::from([
                (String::from("statusCode"), status_code),
                (String::from("body"), body),
            ])
        }
        Err(error) => {
            let status_code = error
                .status()
                .map(|code| build_some(Value::String(code.to_string())))
                .unwrap_or(build_none());

            HashMap::from([
                (String::from("statusCode"), status_code),
                (String::from("body"), build_none()),
            ])
        }
    };

    Ok(Value::Object(Arc::new(Mutex::new(fields))))
}

fn build_value(value: serde_json::Value) -> Value {
    match value {
        serde_json::Value::Array(items) => Value::Array(Arc::new(Mutex::new(
            items.into_iter().map(build_value).collect::<Vec<Value>>(),
        ))),
        serde_json::Value::Null => build_none(),
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

// TODO: Build_none and build_some should be put somewhere besides this file.
fn build_some(data: Value) -> Value {
    let variant = EnumVariantValue {
        enum_ident: String::from("Option"),
        variant_ident: String::from("Some"),
        data: Some(Box::new(data)),
    };

    Value::Variant(variant)
}

fn build_none() -> Value {
    let variant = EnumVariantValue {
        enum_ident: String::from("Option"),
        variant_ident: String::from("None"),
        data: None,
    };

    Value::Variant(variant)
}
