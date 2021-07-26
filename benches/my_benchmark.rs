use multiplied_integers::run_all;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

// fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci(n - 1) + fibonacci(n - 2),
//     }
// }

// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
// }

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("collect_options", |b| b.iter(|| run_all()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
