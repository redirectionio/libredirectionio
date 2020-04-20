use crate::api::Transformer;
use heck::{KebabCase, MixedCase};
use std::collections::HashMap;
use std::str::FromStr;

pub fn transform(data: String, transformer: &Transformer) -> String {
    match transformer.kind.as_ref() {
        None => data,
        Some(kind) => match kind.as_str() {
            "camelize" => camelize(data),
            "dasherize" => dasherize(data),
            "lowercase" => lowercase(data),
            "replace" => replace(data, transformer.options.as_ref()),
            "slice" => slice(data, transformer.options.as_ref()),
            "underscorize" => underscorize(data),
            "uppercase" => uppercase(data),
            _ => data,
        },
    }
}

fn camelize(data: String) -> String {
    data.to_mixed_case()
}

fn dasherize(data: String) -> String {
    data.to_kebab_case()
}

fn lowercase(data: String) -> String {
    data.to_lowercase()
}

fn replace(data: String, options: Option<&HashMap<String, String>>) -> String {
    if options.is_none() {
        return data;
    }

    if !options.unwrap().contains_key("something") {
        return data;
    }

    if !options.unwrap().contains_key("with") {
        return data;
    }

    data.replace(
        options.unwrap().get("something").unwrap(),
        options.unwrap().get("with").unwrap(),
    )
}

fn slice(data: String, options: Option<&HashMap<String, String>>) -> String {
    if options.is_none() {
        return data;
    }

    let from_opt = options.unwrap().get("from");
    let to_opt = options.unwrap().get("to");

    if from_opt.is_none() {
        return data;
    }

    if to_opt.is_none() {
        return data;
    }

    let from = usize::from_str(from_opt.unwrap()).unwrap_or(0);
    let mut to = usize::from_str(to_opt.unwrap()).unwrap_or_else(|_| data.len());

    if from > data.len() {
        return "".to_string();
    }

    if from + to > data.len() {
        to = data.len();
    }

    data[from..to].to_string()
}

fn underscorize(data: String) -> String {
    dasherize(data).replace("-", "_")
}

fn uppercase(data: String) -> String {
    data.to_uppercase()
}
