use auto_temp::Temp;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("auto temp", |b| b.iter(|| bench()));
}

fn bench() {
  let temp = Temp::new(None, Some(36.7), Some(35.0));

  let now_temp = temp.create();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
