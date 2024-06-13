#[macro_use]
extern crate criterion;

use criterion::{BatchSize, BenchmarkId, Criterion};
use flate2::read::GzDecoder;
use redirectionio::api::{Rule, RulesMessage};
use redirectionio::router::Router;
use redirectionio::RouterConfig;
use std::fs::File;

fn create_rules(filename: String) -> RulesMessage {
    let content_gzip = File::open(filename.clone()).expect("Cannot open file");
    let rules: RulesMessage = serde_json::from_reader(GzDecoder::new(content_gzip)).expect("Cannot deserialize");

    rules
}

fn build_router_bench(c: &mut Criterion) {
    let files = vec![
        "../bench-files/heavy-memory-rules.json.gz".to_string(),
        "../bench-files/large-rules-1k.json.gz".to_string(),
        "../bench-files/large-rules-10k.json.gz".to_string(),
        "../bench-files/large-rules-50k.json.gz".to_string(),
        "../bench-files/large-rules-150k.json.gz".to_string(),
        "../bench-files/large-rules-200k.json.gz".to_string(),
        "../bench-files/large-rules-210k.json.gz".to_string(),
    ];

    let mut group = c.benchmark_group("router_builder");

    for filename in files {
        group.sample_size(10);
        group.bench_with_input(BenchmarkId::from_parameter(filename.clone()), &filename, |b, f| {
            b.iter_batched(
                || create_rules(f.to_string()),
                |rules| {
                    let config = RouterConfig::default();
                    let mut router = Router::<Rule>::from_config(config.clone());
                    let rules_len = rules.rules.len();

                    for rule in rules.rules {
                        router.insert(rule);
                    }

                    router.cache(None);

                    assert_eq!(router.len(), rules_len);
                },
                BatchSize::NumIterations(1),
            );
        });
    }

    group.finish();
}

criterion_group!(benches, build_router_bench);
criterion_main!(benches);
