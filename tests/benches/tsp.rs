use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use watchmaker::{
    search, ConcurrencySettings, SearchSettingsBuilder, TourniquetSelector, TspGenetic,
};

#[inline]
fn tsp(population_size: usize, cross_over_candidates: usize, concurrency: ConcurrencySettings) {
    let _result = search(
        Box::new(TspGenetic::default()),
        Box::new(TourniquetSelector::new(cross_over_candidates).unwrap()),
        None,
        &SearchSettingsBuilder::default()
            .concurrency(concurrency)
            .population_size(population_size)
            .mutation_probability(0.01)
            .time_limit(Duration::from_secs(15))
            .epoch_limit(100)
            .build()
            .unwrap(),
    );
}

fn tsp_population_size_1_024_single_threaded(c: &mut Criterion) {
    c.bench_function("tsp 1_024 single threaded", |b| {
        b.iter(|| {
            tsp(
                black_box(1_024),
                black_box(10),
                black_box(ConcurrencySettings::Single),
            )
        })
    });
}

fn tsp_population_size_32_768_single_threaded(c: &mut Criterion) {
    c.bench_function("tsp 32_768 single threaded", |b| {
        b.iter(|| {
            tsp(
                black_box(32_768),
                black_box(15),
                black_box(ConcurrencySettings::Single),
            )
        })
    });
}

fn tsp_population_size_1_024_multi_threaded(c: &mut Criterion) {
    c.bench_function("tsp 1_024 multi threaded", |b| {
        b.iter(|| {
            tsp(
                black_box(1_024),
                black_box(10),
                black_box(ConcurrencySettings::Multi),
            )
        })
    });
}

fn tsp_population_size_32_768_multi_threaded(c: &mut Criterion) {
    c.bench_function("tsp 32_768 multi threaded", |b| {
        b.iter(|| {
            tsp(
                black_box(32_768),
                black_box(15),
                black_box(ConcurrencySettings::Multi),
            )
        })
    });
}

criterion_group!(
    benches,
    tsp_population_size_1_024_single_threaded,
    tsp_population_size_1_024_multi_threaded,
    tsp_population_size_32_768_single_threaded,
    tsp_population_size_32_768_multi_threaded,
);
criterion_main!(benches);
