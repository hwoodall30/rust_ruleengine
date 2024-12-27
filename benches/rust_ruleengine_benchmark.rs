use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_ruleengine::utils::read_json::read_json_file_streaming;
use rust_ruleengine::Condition;

const ITEMS_PATH: &str = "./tests/items.json";

fn criterion_benchmark(c: &mut Criterion) {
    let condition = match read_json_file_streaming::<Condition>("./tests/rules/complex.json") {
        Ok(rule) => rule,
        Err(err) => panic!("{}", err),
    };

    let items = match read_json_file_streaming::<Vec<serde_json::Value>>(ITEMS_PATH) {
        Ok(items) => black_box(items),
        Err(err) => panic!("{}", err),
    };

    let context =
        match read_json_file_streaming::<serde_json::Value>("./tests/contexts/simple.json") {
            Ok(context) => black_box(context),
            Err(err) => panic!("{}", err),
        };

    c.bench_function("rust_ruleengine_benchmark", |b| {
        b.iter(|| condition.filter(&items, &context))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
