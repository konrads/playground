//! Run: `cargo bench --bench smallest_missing` (HTML report in target/criterion/report/index.html)

use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use excercises::corro::in_memory_dynamodb::{solution, solution2};

// worst case: shuffled 1..=n, so the answer is n + 1
fn shuffled(n: i32) -> Vec<i32> {
    let mut xs: Vec<i32> = (1..=n).collect();
    let mut seed: u64 = 42;
    for i in (1..xs.len()).rev() {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        xs.swap(i, (seed >> 33) as usize % (i + 1));
    }
    xs
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("smallest_missing");
    // solution2 is O(n^2), so keep the sample count low
    group.sample_size(10);
    for n in [1_000, 10_000, 100_000] {
        let xs = shuffled(n);
        for (name, f) in [
            ("solution", solution as fn(&mut Vec<i32>) -> i32),
            ("solution2", solution2),
        ] {
            group.bench_with_input(BenchmarkId::new(name, n), &xs, |b, xs| {
                b.iter_batched(|| xs.clone(), |mut xs| f(&mut xs), BatchSize::LargeInput)
            });
        }
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
