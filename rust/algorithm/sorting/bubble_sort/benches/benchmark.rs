use criterion::{criterion_group, criterion_main, Criterion};
use rand::{ Rng, SeedableRng };

use bslib::bubble_sort_immutable;
use bslib::bubble_sort_mutable;

fn numbers() -> Vec<i32> {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(100);
    let mut numbers = vec![rng.gen()];
    for _ in 0..=100 {
        numbers.push(rng.gen());
    }

    numbers
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bubble Sort");

    let nums = numbers();
    group.bench_function("Immutable", |b| {
        b.iter(|| bubble_sort_immutable(nums.clone()))
    });
    group.bench_function("Mutable", |b| {
        b.iter(|| bubble_sort_mutable(&mut nums.clone()))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
