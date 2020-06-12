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

    pub fn count(&mut self, v: u8) {
        match v as char {
            'A' => self.a += 1,
            'C' => self.c += 1,
            'G' => self.g += 1,
            'T' => self.t += 1,
            _ => panic!("Unknown letter")
        }
    }
}

pub fn count_acgt(s: &[u8]) -> HashMap<char, u64> {
    let mut results = CounterResults::new();
    for v in s {
        results.count(*v);
    }
    let mut map = HashMap::with_capacity(4);
    map.insert('A', results.a);
    map.insert('C', results.c);
    map.insert('G', results.g);
    map.insert('T', results.t);
    map
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let map = crate::count_acgt("ACCCGGGGTT".as_bytes());
        assert_eq!(*map.get(&'A').unwrap(), 1);
        assert_eq!(*map.get(&'C').unwrap(), 3);
        assert_eq!(*map.get(&'G').unwrap(), 4);
        assert_eq!(*map.get(&'T').unwrap(), 2);
    }
}
