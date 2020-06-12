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
    let mut results = CounterResults::new();
    for v in s {
        let v = *v as char;
        results.a += (v == 'A') as u64;
        results.c += (v == 'C') as u64;
        results.g += (v == 'G') as u64;
        results.t += (v == 'T') as u64;
    }
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
