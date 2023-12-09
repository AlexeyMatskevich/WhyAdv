use criterion::{black_box, Criterion, criterion_group, criterion_main};
use aoc::{first_try, second_try, third_try, fourth_try};

fn benchmark_function1(c: &mut Criterion) {
    c.bench_function("first_try", |b| b.iter(|| first_try()));
}

fn benchmark_function2(c: &mut Criterion) {
    c.bench_function("second_try", |b| b.iter(|| second_try()));
}

fn benchmark_function3(c: &mut Criterion) {
    c.bench_function("third_try", |b| b.iter(|| third_try()));
}

fn benchmark_function4(c: &mut Criterion) {
    c.bench_function("fourth_try", |b| b.iter(|| fourth_try()));
}

criterion_group!(benches, benchmark_function1, benchmark_function2, benchmark_function3, benchmark_function4);
// criterion_group!(benches, benchmark_function1);
criterion_main!(benches);