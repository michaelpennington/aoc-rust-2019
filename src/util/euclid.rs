use std::ops::{ShrAssign, Sub};

use num_traits::{Euclid, Num, Signed};

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

pub fn mod_inverse<T>(a: T, n: T) -> T
where
    T: Num + Copy + std::fmt::Display + PartialOrd + Signed + Euclid,
{
    let mut t = T::zero();
    let mut new_t = T::one();
    let mut r = n;
    let mut new_r = a;
    while new_r != T::zero() {
        let quotient = r.div_euclid(&new_r);
        (t, new_t) = (new_t, t - quotient * new_t);
        (r, new_r) = (new_r, r - quotient * new_r);
    }
    if r > T::one() {
        panic!("{a} is not invertible mod {n}");
    }
    if t < T::zero() {
        t = t + n;
    }

    t
}

pub fn mod_pow<T>(mut base: T, mut exp: T, modulus: T) -> T
where
    T: Copy + Num + PartialOrd + ShrAssign,
{
    let two = T::one() + T::one();
    if modulus == T::one() {
        return T::zero();
    }
    let mut result = T::one();
    base = base % modulus;
    while exp > T::zero() {
        if exp % two == T::one() {
            result = result * base % modulus;
        }
        exp >>= T::one();
        base = base * base % modulus;
    }
    result
}
