use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Source {
    scheme: String,
    host: String,
    path: String,
    query: String,
}

#[derive(Serialize, Deserialize)]
struct Transformer {
    type_: String,
    options: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
struct Marker {
    name: String,
    regex: String,
    transformers: Vec<Transformer>,
}

#[derive(Serialize, Deserialize)]
struct BodyFilter {
    action: String,
    value: String,
    element_tree: Vec<String>,
    x_path_matcher: String,
}

#[derive(Serialize, Deserialize)]
struct HeaderFilter {
    action: String,
    header: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct Rule {
    id: String,
    source: Option<Source>,
    target: String,
    redirect_code: u16,
    rank: u16,
    markers: Vec<Marker>,
    match_on_response_status: u16,
    body_filters: Vec<BodyFilter>,
    header_filters: Vec<HeaderFilter>,
    #[serde(skip)]
    regex: String,
}
