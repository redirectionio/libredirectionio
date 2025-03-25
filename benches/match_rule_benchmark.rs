#[macro_use]
extern crate criterion;
use criterion::{BenchmarkId, Criterion};
use flate2::read::GzDecoder;
use redirectionio::RouterConfig;
use redirectionio::action::{Action, UnitTrace};
use redirectionio::api::{Rule, RulesMessage};
use redirectionio::http::Request;
use redirectionio::router::Router;
use std::fs::File;

fn create_router(filename: String, config: &RouterConfig) -> Router<Rule> {
    let content_gzip = File::open(filename.clone()).expect("Cannot open file");
    let rules: RulesMessage = serde_json::from_reader(GzDecoder::new(content_gzip)).expect("Cannot deserialize");
    let mut router = Router::<Rule>::from_config(config.clone());

    for rule in rules.rules {
        router.insert(rule)
    }

    router
}

fn no_match_bench(c: &mut Criterion) {
    let files = vec![
        "../bench-files/large-rules-1k.json.gz".to_string(),
        "../bench-files/large-rules-10k.json.gz".to_string(),
        "../bench-files/large-rules-50k.json.gz".to_string(),
        "../bench-files/large-rules-150k.json.gz".to_string(),
        "../bench-files/large-rules-200k.json.gz".to_string(),
        "../bench-files/large-rules-210k.json.gz".to_string(),
    ];

    let mut group = c.benchmark_group("no_match");
    group.sample_size(10);

    for filename in files {
        let config = RouterConfig::default();
        let router = create_router(filename.clone(), &config);
        let request = Request::from_config(&config, "/no-match".to_string(), None, None, None, None, None);

        group.bench_with_input(BenchmarkId::from_parameter(filename.clone()), &filename, |b, _f| {
            b.iter(|| {
                router.match_request(&request);
            });
        });
    }

    group.finish();
}

fn no_match_cache_bench(c: &mut Criterion) {
    let files = vec![
        "../bench-files/large-rules-1k.json.gz".to_string(),
        "../bench-files/large-rules-10k.json.gz".to_string(),
        "../bench-files/large-rules-50k.json.gz".to_string(),
        "../bench-files/large-rules-150k.json.gz".to_string(),
        "../bench-files/large-rules-200k.json.gz".to_string(),
        "../bench-files/large-rules-210k.json.gz".to_string(),
    ];

    let mut group = c.benchmark_group("no_match_cache");
    group.sample_size(10);

    for filename in files {
        let config = RouterConfig::default();
        let mut router = create_router(filename.clone(), &config);
        let request = Request::from_config(&config, "/no-match".to_string(), None, None, None, None, None);

        router.cache(None);

        group.bench_with_input(BenchmarkId::from_parameter(filename.clone()), &filename, |b, _f| {
            b.iter(|| {
                router.match_request(&request);
            });
        });
    }

    group.finish();
}

fn match_rule_in_200k(c: &mut Criterion) {
    let config = RouterConfig::default();
    let mut router = create_router("../bench-files/large-rules-200k.json.gz".to_string(), &config);
    let request = Request::from_config(
        &config,
        "/sites/default/files/image-gallery/lowtideonuseppaimage000000edited_0.jpg".to_string(),
        Some("usharbors.com".to_string()),
        None,
        None,
        None,
        None,
    );

    router.cache(None);

    let mut group = c.benchmark_group("match_rule_in_200k");
    group.sample_size(10);

    group.bench_function("match_rule_in_200k", |b| {
        b.iter(|| {
            router.match_request(&request);
        });
    });

    group.finish();
}

fn build_action_rule_in_200k(c: &mut Criterion) {
    let config = RouterConfig::default();
    let mut router = create_router("../bench-files/large-rules-200k.json.gz".to_string(), &config);
    let request = Request::from_config(
        &config,
        "/sites/default/files/image-gallery/lowtideonuseppaimage000000edited_0.jpg".to_string(),
        Some("usharbors.com".to_string()),
        None,
        None,
        None,
        None,
    );

    router.cache(None);

    let mut group = c.benchmark_group("build_action_rule_in_200k");
    group.sample_size(10);

    group.bench_function("build_action_rule_in_200k", |b| {
        b.iter(|| {
            let rules = router.match_request(&request);
            let mut action = Action::from_routes_rule(rules.clone(), &request, None);

            let action_status_code = action.get_status_code(0, None);
            let (_, backend_status_code) = if action_status_code != 0 {
                (action_status_code, action_status_code)
            } else {
                // We call the backend and get a response code
                let final_status_code = action.get_status_code(200, None);
                (final_status_code, 200)
            };

            action.filter_headers(Vec::new(), backend_status_code, false, None);

            let body = "<!DOCTYPE html>
<html>
    <head>
    </head>
    <body>
    </body>
</html>";

            if let Some(mut body_filter) = action.create_filter_body(backend_status_code, &[]) {
                body_filter.filter(body.into(), None);
                body_filter.end(None);
            }
        });
    });

    group.finish();
}

fn impact(c: &mut Criterion) {
    let config = RouterConfig::default();
    let mut router = create_router("../bench-files/large-rules-200k.json.gz".to_string(), &config);
    let request = Request::from_config(
        &config,
        "/sites/default/files/image-gallery/lowtideonuseppaimage000000edited_0.jpg".to_string(),
        Some("usharbors.com".to_string()),
        None,
        None,
        None,
        None,
    );

    router.cache(None);

    let mut unit_trace = UnitTrace::default();

    let mut group = c.benchmark_group("impact");
    group.sample_size(10);

    group.bench_function("impact", |b| {
        b.iter(|| {
            let rules = router.match_request(&request);
            let mut action = Action::from_routes_rule(rules.clone(), &request, None);

            let action_status_code = action.get_status_code(0, Some(&mut unit_trace));
            let (_, backend_status_code) = if action_status_code != 0 {
                (action_status_code, action_status_code)
            } else {
                // We call the backend and get a response code
                let final_status_code = action.get_status_code(200, Some(&mut unit_trace));
                (final_status_code, 200)
            };

            action.filter_headers(Vec::new(), backend_status_code, false, Some(&mut unit_trace));

            let body = "<!DOCTYPE html>
<html>
    <head>
    </head>
    <body>
    </body>
</html>";

            if let Some(mut body_filter) = action.create_filter_body(backend_status_code, &[]) {
                body_filter.filter(body.into(), Some(&mut unit_trace));
                body_filter.end(Some(&mut unit_trace));
            }

            unit_trace.squash_with_target_unit_traces();
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    no_match_bench,
    no_match_cache_bench,
    match_rule_in_200k,
    build_action_rule_in_200k,
    impact,
);
criterion_main!(benches);
