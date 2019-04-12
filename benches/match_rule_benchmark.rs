#[macro_use]
extern crate criterion;
#[path = "../src/router/mod.rs"]
mod router;
use criterion::black_box;
use criterion::Criterion;

fn match_rule_bench_cache(c: &mut Criterion) {
    let main_router = router::create_test_router(true);
    c.bench_function("match_rule_bench_cache", move |b| {
        b.iter(|| {
            main_router.do_match("https://fr.ouibus.com/fr/montargis".to_string());
        });
    });
}

fn match_rule_bench_nocache(c: &mut Criterion) {
    let main_router = router::create_test_router(false);
    c.bench_function("match_rule_bench_nocache", move |b| {
        b.iter(|| {
            main_router.do_match("https://fr.ouibus.com/fr/montargis".to_string());
        });
    });
}

criterion_group!(benches, match_rule_bench_nocache, match_rule_bench_cache);
criterion_main!(benches);
