use criterion::{criterion_group, criterion_main, Criterion};
use genome_counter::{count, count_opt, CounterResults};
use rand::seq::IteratorRandom;

fn bench_size(c: &mut Criterion, n: usize, tag: &str, f: impl Fn(&[u8]) -> Option<CounterResults>) {
    let mut rng = rand::thread_rng();
    let mut genome = String::new();
    for _ in 0..n {
        genome.push("ACGT".chars().choose(&mut rng).unwrap());
    }
    c.bench_function(format!("count_rand_{}_{}", n, tag).as_str(), |b| {
        b.iter(|| f(genome.as_bytes()))
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_size(c, 100_000_000, "opt", &count_opt);
    bench_size(c, 100_000_000, "simple", &count);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
