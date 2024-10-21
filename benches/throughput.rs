use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use squares::Key;

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("squares::u32", |b| b.iter(|| {
    black_box(squares::u32(black_box(Key::unchecked(0xaf9ed4c87b8e4fa5)), black_box(20)));
  }));

  c.bench_function("squares::u64", |b| b.iter(|| {
    black_box(squares::u64(black_box(Key::unchecked(0xaf9ed4c87b8e4fa5)), black_box(20)))
  }));

  c.bench_function("squares::key", |b| b.iter(|| { black_box(squares::key(black_box(20))); }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);