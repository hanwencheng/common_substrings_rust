use criterion::{criterion_group, criterion_main, Criterion};
use common_substrings_rust::get_substrings;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("get substring", |b| b.iter(|| get_substrings(vec![
        "java", 
        "offe", 
        "coffescript", 
        "typescript", 
        "typed", 
        "javacoffie",
        "fessss",
        "fe",
    ], 2, 3)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);