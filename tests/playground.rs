#[macro_use]
extern crate log;
#[path = "../src/router/mod.rs"]
mod router;
use serde::{Serialize, Deserialize};
use router::rule::Rule;
use std::fs::read_to_string;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiAgentRuleResponse {
    #[serde(rename = "hydra:member")]
    pub rules: Vec<Rule>,
}

fn create_rules(filename: String) -> String {
    let content = read_to_string(filename).expect("Cannot open file");
    let api: ApiAgentRuleResponse = serde_json::from_str(content.as_str()).expect("Cannot deserialize");

    serde_json::to_string(&api.rules).expect("Cannot serialize")
}

fn main() {
    let rules = create_rules( "../bench-files/large-rules-100k.json".to_string());

    println!("loaded rules");

    let router = router::MainRouter::new_from_data(rules, 0).expect("builded");

    println!("router builded");

    let trace = router.trace("https://www.culture.leclerc/pageRecherche?q=mon%20petit%20bebe%20koala".to_string()).expect("traced");

    println!("{:#?}", trace)
}
