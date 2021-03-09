use criterion::{criterion_group, criterion_main, Criterion};
use genome_counter::{count, count_opt, count_opt_li, CounterResults};
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
    // c.bench_function("count4", |b| b.iter(|| count_opt("ACGT".as_bytes())));

    bench_size(c, 1_000_000, "li", &count_opt_li);
    bench_size(c, 1_000_000, "opt", &count_opt);
    bench_size(c, 1_000_000, "simple", &count);

    // let mut i = 1;
    // while i < 100_000_000 {
    //     bench_size(c, i, "opt", &count_opt);
    //     bench_size(c, i, "simple", &count);
    //     i *= 1_000;
    // }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
