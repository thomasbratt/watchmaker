use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use watchmaker::{make_random, Solver, WSGenetic, WSGenome};

#[inline]
fn weasel(n: usize) {
    let mut solver: Solver<WSGenome> = Solver::new(
        Box::new(WSGenetic::new(make_random())),
        0.01,
        n,
        1_000,
        Duration::from_secs(5),
        0.0,
        None,
    );

    let _results = solver.solve();
    // eprintln!("{:?}", results)
}

fn weasel_1_000(c: &mut Criterion) {
    c.bench_function("weasel 1_000", |b| b.iter(|| weasel(black_box(1_000))));
}

fn weasel_10_000(c: &mut Criterion) {
    c.bench_function("weasel 10_000", |b| b.iter(|| weasel(black_box(10_000))));
}

criterion_group!(benches, weasel_1_000, weasel_10_000);
criterion_main!(benches);
