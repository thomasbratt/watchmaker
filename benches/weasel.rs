use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use watchmaker::{make_random, solve, WSGenetic};

#[inline]
fn weasel(n: usize) {
    let _result = solve(
        Box::new(WSGenetic::new(make_random())),
        None,
        0.01,
        n,
        0.0,
        1_000,
        Duration::from_secs(5),
    );
    // eprintln!("{:?}", _results)
}

fn weasel_1_000(c: &mut Criterion) {
    c.bench_function("weasel 1_000", |b| b.iter(|| weasel(black_box(1_000))));
}

fn weasel_10_000(c: &mut Criterion) {
    c.bench_function("weasel 10_000", |b| b.iter(|| weasel(black_box(10_000))));
}

criterion_group!(benches, weasel_1_000, weasel_10_000);
criterion_main!(benches);
