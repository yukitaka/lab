use criterion::{criterion_group, criterion_main, Criterion};

use bslib::bubble_sort_immutable;
use bslib::bubble_sort_mutable;

fn bm1(c: &mut Criterion) {
    c.bench_function("Bubble Sort - Immutable", |b| {
        b.iter(|| bubble_sort_immutable(vec![3, 7, 4, 9, 1, 0]))
    });
}

fn bm2(c: &mut Criterion) {
    c.bench_function("Bubble Sort - Mutable", |b| {
        b.iter(|| {
            let mut numbers = vec![3, 7, 4, 9, 1, 0];
            bubble_sort_mutable(&mut numbers)
        })
    });
}

criterion_group!(benches, bm1, bm2);
criterion_main!(benches);
