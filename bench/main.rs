use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use pie::parser::parse;
use std::{format, fs};

fn bench_parsing(c: &mut Criterion) {
    let filenames = ["applications", "atoms", "declarations", "lambdas"];

    let sources = filenames
        .map(|name| format!("examples/{}.pie", name))
        .map(fs::read_to_string)
        .into_iter()
        .collect::<Result<Vec<String>, std::io::Error>>()
        .unwrap();

    let mut group = c.benchmark_group("parsing");

    for (i, source) in sources.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::from_parameter(filenames[i]),
            source,
            |b, source| b.iter(|| parse(source)),
        );
    }
    group.finish();
}

criterion_group!(parsing, bench_parsing);
criterion_main!(parsing);
