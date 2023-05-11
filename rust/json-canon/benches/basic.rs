use std::{env::current_dir, fs::read_to_string, path::Path};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use json_canon::to_string;
use serde_json::{from_str, Value};

fn bench_input(value: &Value) {
    to_string(value).unwrap();
}

fn from_elem(c: &mut Criterion) {
    let test_data_path = current_dir()
        .unwrap()
        .join(Path::new("./benches/basic.json"));
    let test_data_str = read_to_string(test_data_path).unwrap();
    let test_data: Value = from_str(&test_data_str).unwrap();
    let mut group = c.benchmark_group("from_elem");
    group.throughput(Throughput::Elements(1));
    group.bench_with_input(BenchmarkId::new("basic", &test_data), &test_data, |b, v| {
        b.iter(|| bench_input(&v));
    });
    group.finish();
}

criterion_group!(benches, from_elem);
criterion_main!(benches);
