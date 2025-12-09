pub(crate) struct Prng(u64);

impl Prng {
    pub(crate) const fn create(seed: u64) -> Self {
        Self(seed)
    }

    const fn next_u64(mut x: u64) -> u64 {
        const FACTOR: u64 = 0x2545F4914F6CDD1D;

        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        x.wrapping_mul(FACTOR)
    }

    pub(crate) const fn next(&mut self) -> u64 {
        self.0 = Self::next_u64(self.0);
        self.0
    }
}
