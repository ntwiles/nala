use std::{collections::HashMap, sync::Arc};

use crate::{
    ast::{funcs::*, terms::*, types::*, *},
    io_context::IoContext,
};

pub fn get_request_block() -> Func {
    let params = Params::Param(Param {
        ident: String::from("options"),
        param_type: TypeVariant::Primitive(PrimitiveType::Object),
    });

    Func {
        ident: "request".to_string(),
        params: Box::new(params),
        block: Box::new(Block::RustBlock(builtin_request)),
    }
}

fn builtin_request(args: HashMap<String, Term>, _context: &mut dyn IoContext) -> Term {
    let options = args.get("options").unwrap();

    let mutex = if let Term::Object(reference) = options {
        Arc::clone(&reference)
    } else {
        unreachable!()
    };

    let options = mutex.lock().unwrap();

    let url = if let Term::String(url) = options["url"].clone() {
        url
    } else {
        todo!()
    };

    let method = if let Term::String(method) = options["method"].clone() {
        method
    } else {
        todo!()
    };

    let body = if let Term::String(body) = options["body"].clone() {
        Some(body)
    } else {
        None
    };

    let client = reqwest::blocking::Client::new();

    let client = match method.as_str() {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        _ => todo!(),
    };

    let client = if let Some(body) = body {
        client.body(body)
    } else {
        client
    };

    let response = client.send();

    if let Ok(response) = response {
        Term::String(response.text().unwrap())
    } else {
        Term::Void
    }
}
