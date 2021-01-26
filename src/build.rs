extern crate cbindgen;
extern crate libtool;
extern crate serde;
extern crate serde_yaml;

use serde::{Deserialize, Serialize};
use serde_yaml::from_str as yaml_decode;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{read_dir, read_to_string, DirEntry, File};
use std::io::prelude::*;
use std::path::PathBuf;
use tera::{Context, Tera};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RuleSet {
    #[serde(default)]
    config: RouterConfig,
    rules: HashMap<String, RuleInput>,
    tests: Vec<RuleTest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RouterConfig {
    ignore_host_case: bool,
    ignore_header_case: bool,
    ignore_path_and_query_case: bool,
    ignore_marketing_query_params: bool,
    marketing_query_params: HashSet<String>,
    pass_marketing_query_params_to_target: bool,
}

impl Default for RouterConfig {
    fn default() -> Self {
        let mut parameters = HashSet::new();

        parameters.insert("utm_source".to_string());
        parameters.insert("utm_medium".to_string());
        parameters.insert("utm_campaign".to_string());
        parameters.insert("utm_term".to_string());
        parameters.insert("utm_content".to_string());

        Self {
            ignore_host_case: false,
            ignore_header_case: false,
            ignore_path_and_query_case: false,
            ignore_marketing_query_params: true,
            marketing_query_params: parameters,
            pass_marketing_query_params_to_target: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RuleInput {
    #[serde(rename = "agentInput")]
    agent_input: Rule,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Rule {
    id: Option<String>,
    source: Source,
    target: Option<String>,
    redirect_code: Option<u16>,
    header_filters: Option<Vec<HeaderFilter>>,
    body_filters: Option<Vec<BodyFilter>>,
    rank: Option<u16>,
    markers: Option<Vec<Marker>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Source {
    host: Option<String>,
    path: String,
    query: Option<String>,
    headers: Option<Vec<SourceHeader>>,
    methods: Option<Vec<String>>,
    response_status_codes: Option<Vec<u16>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SourceHeader {
    name: String,
    #[serde(rename = "type")]
    kind: String,
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HeaderFilter {
    action: String,
    header: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BodyFilter {
    action: String,
    value: String,
    element_tree: Vec<String>,
    css_selector: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Marker {
    name: String,
    regex: String,
    transformers: Option<Vec<Transformer>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transformer {
    #[serde(rename = "type")]
    transformer_type: String,
    options: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RuleTest {
    uri: String,
    host: Option<String>,
    scheme: Option<String>,
    method: Option<String>,
    headers: Option<Vec<RuleTestHeader>>,
    response_status_code: Option<u16>,
    #[serde(rename = "match")]
    should_match: bool,
    location: Option<String>,
    status: Option<u16>,
    should_filter_body: Option<ShouldFilterBody>,
    should_filter_header: Option<ShouldFilterHeader>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RuleTestHeader {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ShouldFilterBody {
    enable: bool,
    original_body: String,
    expected_body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ShouldFilterHeader {
    enable: bool,
    original_headers: Vec<RuleTestHeader>,
    expected_headers: Vec<RuleTestHeader>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RuleSetList {
    rule_sets: HashMap<String, RuleSet>,
}

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let mut build_dir = target_dir();

    if env::var("CARGO_CFG_TARGET_ENV").unwrap() == "musl" {
        build_dir = build_dir.join("x86_64-unknown-linux-musl");
    }

    build_dir = build_dir.join(env::var("PROFILE").unwrap());

    let output_file = build_dir.join(format!("{}.h", package_name)).display().to_string();

    cbindgen::generate(crate_dir)
        .expect("Unable to generate bindings")
        .write_to_file(&output_file);

    let rule_sets = read_tests("../../tests/rules");

    if rule_sets.is_empty() {
        return;
    }

    let templating = match Tera::new("tests/templates/**/*") {
        Ok(t) => t,
        Err(e) => panic!(e),
    };

    let rule_sets_list = RuleSetList { rule_sets };

    let context = Context::from_serialize(&rule_sets_list).expect("cannot serialize");
    let test_content = templating.render("main.rs.j2", &context).expect("cannot generate");
    let mut file = File::create("tests/redirectionio_router_test.rs").expect("cannot create file");

    file.write_all(test_content.as_bytes()).expect("cannot write");
}

fn read_tests(path: &str) -> HashMap<String, RuleSet> {
    let mut rule_sets = HashMap::new();

    match read_dir(path) {
        Err(_) => return rule_sets,
        Ok(directory) => {
            for file_result in directory {
                if let Ok(file) = file_result {
                    if let Ok(file_type) = file.file_type() {
                        if file_type.is_dir() {
                            rule_sets.extend(read_tests(file.path().to_str().unwrap()))
                        } else if file_type.is_file() {
                            match file.path().extension() {
                                None => (),
                                Some(ext) => {
                                    if ext == "yml" {
                                        let (key, rule_set) = build_test_file(file).expect("");

                                        rule_sets.insert(key, rule_set);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    rule_sets
}

fn build_test_file(file: DirEntry) -> std::io::Result<(String, RuleSet)> {
    let content = read_to_string(file.path())?;
    let mut rule_set: RuleSet = yaml_decode(content.as_str()).expect("error");

    for (id, rule) in &mut rule_set.rules {
        rule.agent_input.id = Some(id.clone());
        rule.agent_input.rank = Some(0);
    }

    let name = file
        .path()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        .replace(".yml", "")
        .replace("-", "_");

    Ok((name, rule_set))
}

/// Find the location of the `target/` directory. Note that this may be
/// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR`
/// variable.
fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}
