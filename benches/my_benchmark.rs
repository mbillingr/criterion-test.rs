#[macro_use]
extern crate criterion;

use criterion::Criterion;

use std::time::Duration;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(2)));
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::new(3, 0))
        .measurement_time(Duration::new(1, 0));
    targets = criterion_benchmark
}

criterion_main!(benches);

