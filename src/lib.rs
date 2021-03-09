use packed_simd::*;
use rayon::prelude::*;

pub struct CounterResults {
    pub a: u64,
    pub c: u64,
    pub g: u64,
    pub t: u64,
}

impl CounterResults {
    fn new() -> Self {
        Self {
            a: 0,
            c: 0,
            g: 0,
            t: 0,
        }
    }

    fn add(self, other: Self) -> Self {
        Self {
            a: self.a + other.a,
            c: self.c + other.c,
            g: self.g + other.g,
            t: self.t + other.t,
        }
    }
}

#[inline]
fn check_results(s: &[u8], results: CounterResults) -> Option<CounterResults> {
    if results.a + results.c + results.g + results.t != s.len() as u64 {
        None
    } else {
        Some(results)
    }
}

fn count_simple(s: &[u8]) -> CounterResults {
    let mut results = CounterResults::new();
    for v in s {
        let v = *v as char;
        results.a += (v == 'A') as u64;
        results.c += (v == 'C') as u64;
        results.g += (v == 'G') as u64;
        results.t += (v == 'T') as u64;
    }
    results
}

fn count_li_simple(s: &[u8]) -> (i64, i64, i64, i64) {
    let mut sum1: i64 = 0;
    let mut sum2: i64 = 0;
    let mut sum3: i64 = 0;
    let sum4 = s.len() as i64;

    for &v in s {
        let x = v & 7;
        sum1 += x as i64;
        sum2 += (x >> 1) as i64;
        sum3 += (x >> 2) as i64;
    }

    (sum1, sum2, sum3, sum4)
}

/// Vectorized version of count_li_simple
fn count_li_simd(s: &[u8]) -> (i64, i64, i64, i64) {
    let mut sum1: i64 = 0;
    let mut sum2: i64 = 0;
    let mut sum3: i64 = 0;
    let sum4 = s.len() as i64;

    for chunk in s.chunks_exact(32) {
        let chunk = u8x32::from_slice_unaligned(chunk);
        let x = chunk & 7;
        sum1 += x.wrapping_sum() as i64;
        sum2 += (x >> 1).wrapping_sum() as i64;
        sum3 += (x >> 2).wrapping_sum() as i64;
    }

    (sum1, sum2, sum3, sum4)
}

fn count_li(s: &[u8]) -> CounterResults {
    let n = 32;
    let chunk_size = (1 << 10) * n;

    let (sum1, sum2, sum3, sum4) = s
        .par_chunks(chunk_size)
        .map(|chunks| {
            if chunks.len() == chunk_size {
                count_li_simd(chunks)
            } else {
                count_li_simple(chunks)
            }
        })
        .reduce(
            || (0, 0, 0, 0),
            |l, r| (l.0 + r.0, l.1 + r.1, l.2 + r.2, l.3 + r.3),
        );
    let a = sum1 * 1 + sum2 * -3 + sum3 * 2 + sum4 * 0;
    let c = sum1 * -1 + sum2 * 3 + sum3 * -3 + sum4 * 1;
    let g = sum1 * 1 + sum2 * -2 + sum3 * 1 + sum4 * -1;
    let t = sum1 * -1 + sum2 * 2 + sum3 * 0 + sum4 * 1;

    CounterResults {
        a: a as u64,
        c: c as u64,
        g: g as u64,
        t: t as u64,
    }
}

pub fn count_opt_li(s: &[u8]) -> Option<CounterResults> {
    check_results(s, count_li(s))
}

pub fn count(s: &[u8]) -> Option<CounterResults> {
    check_results(s, count_simple(s))
}

#[inline]
fn count_letter(chunk: &u8x32, letter: &u8x32) -> u64 {
    let a_eq = (*letter).eq(*chunk);
    (-(u8x32::from_cast(a_eq).wrapping_sum() as i8)) as u64
}

pub fn count_opt(s: &[u8]) -> Option<CounterResults> {
    if s.len() < 10_000 {
        return count(s);
    }
    let n = 32;
    let av = u8x32::splat('A' as u8);
    let cv = u8x32::splat('C' as u8);
    let gv = u8x32::splat('G' as u8);
    let tv = u8x32::splat('T' as u8);
    let chunk_size = (1 << 10) * n;

    let results = s
        .par_chunks(chunk_size)
        .map(|chunk| {
            if chunk.len() == chunk_size {
                let mut results = CounterResults::new();
                for chunk in chunk.chunks_exact(n) {
                    let chunk = u8x32::from_slice_unaligned(chunk);
                    results.a += count_letter(&chunk, &av);
                    results.c += count_letter(&chunk, &cv);
                    results.g += count_letter(&chunk, &gv);
                    results.t += count_letter(&chunk, &tv);
                }
                results
            } else {
                count_simple(chunk)
            }
        })
        .reduce(|| CounterResults::new(), CounterResults::add);
    check_results(s, results)
}

#[cfg(test)]
mod tests {
    use rand::seq::IteratorRandom;
    use rand::Rng;

    fn compare_on(genome: &String) {
        let r1 = crate::count_opt(genome.as_bytes()).unwrap();
        let r2 = crate::count(genome.as_bytes()).unwrap();
        let r3 = crate::count_li(genome.as_bytes());
        assert_eq!(r1.a, r2.a);
        assert_eq!(r1.c, r2.c);
        assert_eq!(r1.g, r2.g);
        assert_eq!(r1.t, r2.t);
        assert_eq!(r1.a, r3.a);
        assert_eq!(r1.c, r3.c);
        assert_eq!(r1.g, r3.g);
        assert_eq!(r1.t, r3.t);
    }

    #[test]
    fn correctness() {
        let mut rng = rand::thread_rng();
        for _ in 1..100 {
            let mut genome = String::new();
            for _ in 0..rng.gen_range(0..100_000) {
                genome.push('A');
            }
            compare_on(&genome);
            let mut genome = String::new();
            for _ in 0..rng.gen_range(0..100_000) {
                genome.push('C');
            }
            compare_on(&genome);
            let mut genome = String::new();
            for _ in 0..rng.gen_range(0..100_000) {
                genome.push('G');
            }
            compare_on(&genome);
            let mut genome = String::new();
            for _ in 0..rng.gen_range(0..100_000) {
                genome.push('T');
            }
            compare_on(&genome);
            let mut genome = String::new();
            for _ in 0..rng.gen_range(0..100_000) {
                genome.push("ACGT".chars().choose(&mut rng).unwrap());
            }
            compare_on(&genome);
        }
    }
}
