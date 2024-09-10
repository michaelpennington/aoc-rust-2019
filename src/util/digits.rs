pub struct DigitsIter {
    n: u32,
    divisor: u32,
}

impl DigitsIter {
    pub fn new(n: u32) -> Self {
        let mut divisor = 1;
        while n >= divisor * 10 {
            divisor *= 10;
        }

        Self { n, divisor }
    }
}

impl Iterator for DigitsIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let v = Some(self.n / self.divisor);
            self.n %= self.divisor;
            self.divisor /= 10;
            v
        }
    }
}
