use std::{env::current_dir, fs::read_to_string, path::Path};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use json_canon::to_string as canon_to_string;
use serde_json::{from_str, to_string as compact_to_string, Value};

fn from_elem(c: &mut Criterion) {
    let test_data_path = current_dir()
        .unwrap()
        .join(Path::new("./benches/basic.json"));
    let test_data_str = read_to_string(test_data_path).unwrap();
    let test_data: Value = from_str(&test_data_str).unwrap();
    let mut group = c.benchmark_group("from_elem");
    group.throughput(Throughput::Elements(1));
    group.bench_with_input(BenchmarkId::new("canon", &test_data), &test_data, |b, v| {
        b.iter(|| canon_to_string(&v).unwrap());
    });
    group.bench_with_input(
        BenchmarkId::new("compact", &test_data),
        &test_data,
        |b, v| {
            b.iter(|| compact_to_string(&v).unwrap());
        },
    );
    group.bench_with_input(
        BenchmarkId::new("ser_and_de", &test_data_str),
        &test_data_str,
        |b, s| {
            b.iter(|| {
                let v: Value = from_str(s).unwrap();
                compact_to_string(&v).unwrap();
            })
        },
    );
    group.finish();
}

criterion_group!(benches, from_elem);
criterion_main!(benches);
