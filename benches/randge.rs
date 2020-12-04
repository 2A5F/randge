use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::thread_rng;
use randge::{randge_barrel, randge_linear, randge_tree};

pub fn randge(c: &mut Criterion) {
    let mut group = c.benchmark_group("randge");
    group.bench_function("linear", |b| {
        b.iter(|| randge_linear(black_box(-10000..10000), black_box(10000), thread_rng()).collect::<Vec<_>>())
    });
    group.bench_function("tree", |b| {
        b.iter(|| randge_tree(black_box(-10000..10000), black_box(10000), thread_rng()).collect::<Vec<_>>())
    });
    group.bench_function("barrel", |b| {
        b.iter(|| randge_barrel(black_box(-10000..10000), black_box(10000), thread_rng()).collect::<Vec<_>>())
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = randge
}
criterion_main!(benches);
