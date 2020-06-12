use std::collections::HashMap;
use std::mem::transmute;

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

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub fn count_acgt(s: &[u8]) -> Option<HashMap<char, u64>> {
    let results = unsafe {
        let mut counter = _mm256_setzero_si256();
        let acgt = _mm256_set_epi64x('A' as i64, 'C' as i64, 'G' as i64, 'T' as i64);
        for v in s {
            let v = _mm256_set1_epi64x(*v as i64);
            let eq = _mm256_cmpeq_epi64(acgt, v);
            counter = _mm256_add_epi64(counter, eq);
        }
        let (t, g, c, a): (i64, i64, i64, i64) = transmute(counter);
        CounterResults {
            a: -a as u64,
            c: -c as u64,
            g: -g as u64,
            t: -t as u64,
        }
    };

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
        let map = crate::count_acgt("ACCCGGGGTT".as_bytes()).unwrap();
        assert_eq!(*map.get(&'A').unwrap(), 1);
        assert_eq!(*map.get(&'C').unwrap(), 3);
        assert_eq!(*map.get(&'G').unwrap(), 4);
        assert_eq!(*map.get(&'T').unwrap(), 2);
    }
}
