use auto_temp::Temp;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("auto temp", |b| b.iter(|| bench()));
  c.bench_function("auto temp multi", |b| b.iter(|| bench_multi()));
}

fn bench() {
  let temp = Temp::new(None, None).unwrap();

  temp.create();
}

fn bench_multi() {
  let temp = Temp::new(None, None).unwrap();

  temp.create_multiple(100);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
