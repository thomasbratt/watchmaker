use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use watchmaker::{make_random, solve, WSGenetic};

#[inline]
fn weasel(population_size: usize, cross_over_candidates: usize) {
    let _result = solve(
        Box::new(WSGenetic::new(make_random())),
        None,
        0.0,
        cross_over_candidates,
        1_000,
        0.01,
        population_size,
        Duration::from_secs(15),
    );
    // eprintln!("{:?}", _result)
}

fn weasel_population_size_1_000(c: &mut Criterion) {
    c.bench_function("weasel 1_000", |b| {
        b.iter(|| weasel(black_box(1_000), black_box(3)))
    });
}

fn weasel_population_size_10_000(c: &mut Criterion) {
    c.bench_function("weasel 10_000", |b| {
        b.iter(|| weasel(black_box(10_000), black_box(3)))
    });
}

fn weasel_candidates_8(c: &mut Criterion) {
    c.bench_function("weasel_candidates_8", |b| {
        b.iter(|| weasel(black_box(1_000), black_box(8)))
    });
}

fn weasel_candidates_32(c: &mut Criterion) {
    c.bench_function("weasel_candidates_32", |b| {
        b.iter(|| weasel(black_box(1_000), black_box(32)))
    });
}

criterion_group!(
    benches,
    weasel_population_size_1_000,
    weasel_population_size_10_000,
    weasel_candidates_8,
    weasel_candidates_32,
);
criterion_main!(benches);
