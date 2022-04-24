use watchmaker::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};


#[inline]
fn weasel(n: usize) {

}

fn weasel_10(c: &mut Criterion) {
    c.bench_function("weasel 10", |b| b.iter(|| weasel(black_box(10))));
}

fn weasel_100(c: &mut Criterion) {
    c.bench_function("weasel 100", |b| b.iter(|| weasel(black_box(100))));
}

criterion_group!(benches, weasel_10, weasel_100);
criterion_main!(benches);
