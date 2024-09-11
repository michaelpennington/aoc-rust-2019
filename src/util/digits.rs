use std::ops::{Div, DivAssign, Mul, MulAssign, RemAssign};

pub struct DigitsIter<T> {
    n: T,
    divisor: T,
}

impl<T> DigitsIter<T>
where
    T: TryFrom<usize> + MulAssign + Mul<Output = T> + PartialOrd + Clone + Copy,
    <T as TryFrom<usize>>::Error: std::fmt::Debug,
{
    pub fn new(n: T) -> Self {
        let ten = 10.try_into().unwrap();
        let mut divisor = 1.try_into().unwrap();
        while n >= divisor * 10.try_into().unwrap() {
            divisor *= ten;
        }

        Self { n, divisor }
    }
}

impl<T> Iterator for DigitsIter<T>
where
    T: TryFrom<usize> + RemAssign + DivAssign + Div<Output = T> + PartialEq + Clone + Copy,
    <T as TryFrom<usize>>::Error: std::fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0.try_into().unwrap() {
            None
        } else {
            let v = Some(self.n / self.divisor);
            self.n %= self.divisor;
            self.divisor /= 10.try_into().unwrap();
            v
        }
    }
}
