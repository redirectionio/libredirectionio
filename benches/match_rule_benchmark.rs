#[macro_use]
extern crate criterion;
#[macro_use]
extern crate log;
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

fn no_match_bench(c: &mut Criterion) {
    let files = vec![
        "../bench-files/large-rules-1k.json".to_string(),
        "../bench-files/large-rules-10k.json".to_string(),
        "../bench-files/large-rules-50k.json".to_string(),
        "../bench-files/large-rules-150k.json".to_string(),
        "../bench-files/large-rules-200k.json".to_string(),
    ];

    let mut group = c.benchmark_group("no_match");
    group.sample_size(10);

    for filename in files {
        let rules = create_rules(filename.clone());
        let router = MainRouter::new_from_data(rules, 0).expect("Cannot create router");

        group.bench_with_input(BenchmarkId::from_parameter(filename.clone()), &filename, |b, _f| {
            b.iter(|| {
                router.match_rule("/no-match".to_string());
            });
        });
    }

    group.finish();
}

fn no_match_cache_bench(c: &mut Criterion) {
    let files = vec![
//        "../bench-files/large-rules-1k.json".to_string(),
        "../bench-files/large-rules-10k.json".to_string(),
//        "../bench-files/large-rules-50k.json".to_string(),
//        "../bench-files/large-rules-150k.json".to_string(),
//        "../bench-files/large-rules-200k.json".to_string(),
    ];

    let mut group = c.benchmark_group("no_match_cache");
    group.sample_size(10);

    for filename in files {
        let rules = create_rules(filename.clone());
        let router = MainRouter::new_from_data(rules, 1000).expect("Cannot create router");

        group.bench_with_input(BenchmarkId::from_parameter(filename.clone()), &filename, |b, _f| {
            b.iter(|| {
                router.match_rule("/no-match".to_string());
            });
        });
    }

    group.finish();
}

criterion_group!(benches, no_match_cache_bench);
criterion_main!(benches);
