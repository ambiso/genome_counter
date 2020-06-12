#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

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
            _ => unreachable!()
        }
    }
}

pub fn count_acgt(s: &[u8]) -> CounterResults {
    let mut results = CounterResults::new();
    for v in s {
        results.count(*v);
    }
    results
}
