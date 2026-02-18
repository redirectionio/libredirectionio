use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use redirectionio::{RouterConfig, api::VariableValue, http::PathAndQueryWithSkipped, marker::StaticOrDynamic};

fn static_or_dynamic_replace(c: &mut Criterion) {
    let mut group = c.benchmark_group("StaticOrDynamic::replace");

    for &count in &[2, 5, 10, 20] {
        let variables: Vec<(String, VariableValue)> = (0..count)
            .map(|i| (format!("var{i}"), VariableValue::Value(format!("value{i}"))))
            .collect();

        let template: String = (0..count).map(|i| format!("/path/@var{i}/segment")).collect();

        group.bench_with_input(BenchmarkId::from_parameter(count), &(template, variables), |b, (tpl, vars)| {
            b.iter(|| StaticOrDynamic::replace(tpl.clone(), black_box(vars), false));
        });
    }

    group.finish();
}

fn path_and_query_with_skipped_from_config(c: &mut Criterion) {
    let mut group = c.benchmark_group("PathAndQueryWithSkipped::from_config");

    let urls: &[(&str, &str)] = &[
        ("/simple-path", "simple"),
        ("/path?a=1&b=2&c=3", "3_params"),
        ("/path?a=1&b=2&c=3&d=4&e=5&f=6&g=7&h=8&i=9&j=10", "10_params"),
        (
            "/path?a=1&b=2&c=3&d=4&e=5&f=6&g=7&h=8&i=9&j=10&utm_source=google&utm_medium=cpc&utm_campaign=brand&utm_term=test&utm_content=ad1",
            "10_params_5_marketing",
        ),
        (
            "/path?key=hello+world&special=%3Cscript%3E&long_value=Lorem+ipsum+dolor+sit+amet+consectetur+adipiscing+elit",
            "special_chars",
        ),
    ];

    let config = RouterConfig::default();

    for &(url, label) in urls {
        group.bench_with_input(BenchmarkId::from_parameter(label), &url, |b, &url| {
            b.iter(|| PathAndQueryWithSkipped::from_config(&config, url));
        });
    }

    group.finish();
}

criterion_group!(benches, static_or_dynamic_replace, path_and_query_with_skipped_from_config,);
criterion_main!(benches);
