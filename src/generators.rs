pub trait Generator: Iterator<Item = u64> {}

/// Extend any Iterator<Item=u64> to be a Generator
impl<T: Iterator<Item = u64>> Generator for T {}

pub struct XorShift64 {
    state: u64,
    remaining: u128,
}

impl XorShift64 {
    pub fn new(seed: u64, count: u128) -> Self {
        XorShift64 {
            state: seed,
            remaining: count,
        }
    }
}

impl Iterator for XorShift64 {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        self.state = self.state.wrapping_mul(0x2545F4914F6CDD1Du64);

        self.remaining -= 1;
        Some(self.state)
    }
}
