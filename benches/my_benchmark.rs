use criterion::{black_box, criterion_group, criterion_main, Criterion};
use counter::count_acgt;
use rand::seq::IteratorRandom;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("count4", |b| b.iter(|| count_acgt("ACGT".as_bytes())));
    let mut rng = rand::thread_rng();
    let mut genome = String::new();
    for i in 0..100_00_000 {
        genome.push("ACGT".chars().choose(&mut rng).unwrap());
    }
    println!("length: {}", genome.len());
    c.bench_function("count_rand", |b| b.iter(|| count_acgt(genome.as_bytes())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);