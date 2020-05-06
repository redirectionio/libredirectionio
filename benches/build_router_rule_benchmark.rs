#[macro_use]
extern crate criterion;
use criterion::{BatchSize, BenchmarkId, Criterion};
use redirectionio::api::{Rule, RulesMessage};
use redirectionio::router::Router;
use std::fs::read_to_string;

fn create_rules(filename: String) -> RulesMessage {
    let content = read_to_string(filename).expect("Cannot open file");
    let rules: RulesMessage = serde_json::from_str(content.as_str()).expect("Cannot deserialize");

    rules
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
                |rules| {
                    let mut router = Router::<Rule>::default();

                    for rule in rules.rules {
                        router.insert(rule.into_route());
                    }
                },
                BatchSize::NumIterations(1),
            );
        });
    }

    group.finish();
}

criterion_group!(benches, build_router_bench);
criterion_main!(benches);
