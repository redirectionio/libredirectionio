#[macro_use]
extern crate criterion;
use criterion::{Criterion, BenchmarkId, BatchSize};
use serde::{Serialize, Deserialize};
use redirectionio::{Rule, MainRouter};
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

fn build_router_bench(c: &mut Criterion) {
    let files = vec![
        "../bench-files/large-rules-1k.json".to_string(),
        "../bench-files/large-rules-10k.json".to_string(),
        "../bench-files/large-rules-50k.json".to_string(),
        "../bench-files/large-rules-150k.json".to_string(),
        "../bench-files/large-rules-200k.json".to_string(),
    ];

    let mut group = c.benchmark_group("router_builder");

    for filename in files {
        group.sample_size(10);
        group.bench_with_input(BenchmarkId::from_parameter(filename.clone()), &filename, |b, f| {
            b.iter_batched(
                || create_rules(f.to_string()),
                |rules| MainRouter::new_from_data(rules, 0),
                BatchSize::NumIterations(1),
            );
        });
    }

    group.finish();
}

criterion_group!(benches, build_router_bench);
criterion_main!(benches);
