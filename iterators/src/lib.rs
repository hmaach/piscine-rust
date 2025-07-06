#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 {
            return None;
        }

        let current = self.v;

        if self.v == 1 {
            self.v = 0;
        } else if self.v % 2 == 0 {
            self.v /= 2;
        } else {
            self.v = self.v * 3 + 1;
        }

        Some(current)
    }
}

impl Collatz {
    pub fn new(v: u64) -> Self {
        Self { v }
    }
}

pub fn collatz(n: u64) -> usize {
    if n == 0 {
        return 0;
    }

    Collatz::new(n).count() - 1
}
