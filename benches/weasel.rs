use criterion::{black_box, criterion_group, criterion_main, Criterion};
use watchmaker::{make_random, search, SettingsBuilder, WSGenetic};

#[inline]
fn weasel(population_size: usize, cross_over_candidates: usize) {
    let _result = search(
        Box::new(WSGenetic::new(make_random())),
        None,
        make_random(),
        &SettingsBuilder::default()
            .cross_over_candidates(cross_over_candidates)
            .population_size(population_size)
            .build()
            .unwrap(),
    );
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
