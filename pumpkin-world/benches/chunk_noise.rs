use criterion::{criterion_group, criterion_main, Criterion};
use pumpkin_world::bench_create_chunk_noise_overworld;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("overworld convert", |b| {
        b.iter(bench_create_chunk_noise_overworld)
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);