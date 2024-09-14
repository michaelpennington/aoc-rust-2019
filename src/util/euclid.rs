use std::ops::Sub;

use num_traits::{Euclid, Num};

fn abs<T>(a: T) -> T
where
    T: Num + Sub<Output = T> + PartialOrd,
{
    if a < T::zero() {
        T::zero() - a
    } else {
        a
    }
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Euclid + Num + Copy + PartialOrd,
{
    let mut a = abs(a);
    let mut b = abs(b);
    let mut t;
    while b != T::zero() {
        t = b;
        b = a.rem_euclid(&b);
        a = t;
    }
    a
}
