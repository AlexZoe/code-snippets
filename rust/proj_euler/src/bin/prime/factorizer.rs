pub struct PrimeFactorizer {
    input: u64,
    next_candidate: u64,
}

impl PrimeFactorizer {
    pub fn new(input: u64) -> Self {
        PrimeFactorizer {
            input,
            next_candidate: 2,
        }
    }
}

impl Iterator for PrimeFactorizer {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next_candidate < ((self.input as f64).sqrt() as u64 + 1) {
            let mut found: u64 = 0;
            while self.input % self.next_candidate == 0 {
                found = self.next_candidate;
                self.input /= found;
            }

            self.next_candidate += if self.next_candidate == 2 { 1 } else { 2 };

            if found != 0 {
                return Some(found);
            }
        }

        let found = match self.input {
            0 | 1 => None,
            _ => Some(self.input),
        };
        self.input = 0;

        found
    }
}
