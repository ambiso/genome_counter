use counter::count_opt;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::seq::IteratorRandom;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("count4", |b| b.iter(|| count_opt("ACGT".as_bytes())));
    let mut rng = rand::thread_rng();
    let mut genome = String::new();
    for _ in 0..100_000 {
        genome.push("ACGT".chars().choose(&mut rng).unwrap());
    }
    println!("length: {}", genome.len());
    c.bench_function("count_100k_rand", |b| {
        b.iter(|| count_opt(genome.as_bytes()))
    });
    let mut genome = String::new();
    for _ in 0..100_000_000 {
        genome.push("ACGT".chars().choose(&mut rng).unwrap());
    }
    println!("length: {}", genome.len());
    c.bench_function("count_100m_rand", |b| {
        b.iter(|| count_opt(genome.as_bytes()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
