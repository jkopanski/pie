use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use pie::parser::parse;
use std::{borrow::Cow, format, fs};

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
        group.bench_function(BenchmarkId::from_parameter(filenames[i]), |b| {
            b.iter_batched_ref(
                || -> Cow<str> { Cow::from(source) },
                parse,
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(parsing, bench_parsing);
criterion_main!(parsing);
