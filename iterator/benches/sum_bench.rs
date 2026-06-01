use criterion::{Criterion, criterion_group, criterion_main};
use rand;

fn sum_for(x: &[f64]) -> f64 {
    let mut result = 0.0;
    for i in 0..x.len() {
        result += x[i];
    }
    result
}

fn sum_iter(x: &[f64]) -> f64 {
    x.iter().sum()
}

const LEN: usize = 1024 * 1024;

fn bench(c: &mut Criterion) {
    let samples: Vec<f64> = (0..LEN).map(|_| rand::random::<_>()).collect();
    c.bench_function("for_loop", |b| b.iter(|| sum_for(&samples)));
    c.bench_function("iterator", |b| b.iter(|| sum_iter(&samples)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
