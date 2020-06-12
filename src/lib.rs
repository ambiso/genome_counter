use packed_simd::*;
use std::collections::HashMap;

pub struct CounterResults {
    a: u64,
    c: u64,
    g: u64,
    t: u64,
}

impl CounterResults {
    pub fn new() -> Self {
        Self {
            a: 0,
            c: 0,
            g: 0,
            t: 0,
        }
    }
}

pub fn count_acgt(s: &[u8]) -> Option<HashMap<char, u64>> {
    let av = u8x32::splat('A' as u8);
    let cv = u8x32::splat('C' as u8);
    let gv = u8x32::splat('G' as u8);
    let tv = u8x32::splat('T' as u8);

    #[inline]
    fn count_letter(chunk: &u8x32, letter: &u8x32) -> u64 {
        let a_eq = (*letter).eq(*chunk);
        let count_a = u8x32::from_cast(a_eq).wrapping_sum();
        (-(count_a as i8)) as u64
    }

    let mut results = CounterResults::new();
    s.chunks_exact(32)
        .map(u8x32::from_slice_unaligned)
        .for_each(|chunk| {
            results.a += count_letter(&chunk, &av);
            results.c += count_letter(&chunk, &cv);
            results.g += count_letter(&chunk, &gv);
            results.t += count_letter(&chunk, &tv);
        });
    if results.a + results.c + results.g + results.t != s.len() as u64 {
        None
    } else {
        let mut map = HashMap::with_capacity(4);
        map.insert('A', results.a);
        map.insert('C', results.c);
        map.insert('G', results.g);
        map.insert('T', results.t);
        Some(map)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let map = crate::count_acgt("AAAACCCCGGGGTTTTAAAAAAAACCTTTTTT".as_bytes()).unwrap();
        assert_eq!(*map.get(&'A').unwrap(), 12);
        assert_eq!(*map.get(&'C').unwrap(), 6);
        assert_eq!(*map.get(&'G').unwrap(), 4);
        assert_eq!(*map.get(&'T').unwrap(), 10);
    }
}
