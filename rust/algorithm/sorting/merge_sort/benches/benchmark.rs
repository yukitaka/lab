use criterion::{criterion_group, criterion_main, Criterion};
use rand::{Rng, SeedableRng};

use mslib::merge_sort;

fn numbers() -> Vec<i32> {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(100);
    let mut numbers = vec![rng.gen()];
    for _ in 0..=100 {
        numbers.push(rng.gen());
    }

    numbers
}

fn bench(c: &mut Criterion) {
    let nums = numbers();
    c.bench_function("Merge sort", |b| b.iter(|| merge_sort(&mut nums.clone())));
}

criterion_group!(benches, bench);
criterion_main!(benches);
