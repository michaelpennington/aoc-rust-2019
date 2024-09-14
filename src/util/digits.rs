use num_traits::{FromPrimitive, NumAssign, PrimInt};

pub struct DigitsIter<T> {
    n: T,
    divisor: T,
}

impl<T> DigitsIter<T>
where
    T: PrimInt + FromPrimitive + NumAssign,
{
    pub fn new(n: T) -> Self {
        let ten = T::from_u64(10).unwrap();
        let mut divisor = T::one();
        while n >= divisor * ten {
            divisor *= ten;
        }

        Self { n, divisor }
    }
}

impl<T> Iterator for DigitsIter<T>
where
    T: PrimInt + NumAssign + FromPrimitive,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == T::zero() {
            None
        } else {
            let v = Some(self.n / self.divisor);
            self.n %= self.divisor;
            self.divisor /= T::from_u64(10).unwrap();
            v
        }
    }
}
