use criterion::{black_box, criterion_group, criterion_main, Criterion};
use collatz::collatz;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("collatz", |b| b.iter(|| collatz(black_box(10000000000000000000),1)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);