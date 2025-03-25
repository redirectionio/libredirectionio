#[macro_use]
extern crate criterion;
use criterion::{BenchmarkId, Criterion};
use flate2::read::GzDecoder;
use redirectionio::RouterConfig;
use redirectionio::api::{Rule, RuleChangeSet, RulesMessage, TestExamplesOutput, TestExamplesProjectInput};
use redirectionio::router::Router;
use std::fs::File;

mod perf;

fn test_examples_bench(c: &mut Criterion) {
    let files = vec!["../bench-files/large-rules-210k.json.gz".to_string()];

    let mut group = c.benchmark_group("no_match");
    group.sample_size(10);

    for filename in files {
        let config = RouterConfig::default();
        let content_gzip = File::open(filename.clone()).expect("Cannot open file");
        let rules: RulesMessage = serde_json::from_reader(GzDecoder::new(content_gzip)).expect("Cannot deserialize");
        let mut router = Router::<Rule>::from_config(config.clone());

        for rule in rules.rules {
            router.insert(rule)
        }

        router.cache(None);

        let arc_router = std::sync::Arc::new(router);

        group.bench_with_input(BenchmarkId::from_parameter(filename.clone()), &filename, |b, _f| {
            b.iter(|| {
                TestExamplesOutput::from_project(
                    TestExamplesProjectInput {
                        change_set: RuleChangeSet::default(),
                        max_hops: 5,
                        project_domains: vec![],
                    },
                    arc_router.clone(),
                );
            });
        });
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(99));
    targets = test_examples_bench
}
criterion_main!(benches);
