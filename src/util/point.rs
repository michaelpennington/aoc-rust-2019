use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub},
    str::FromStr,
};

use anyhow::anyhow;
use num_traits::{Euclid, Num, NumAssign};

use super::euclid::gcd;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pt<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    pub fn turn(&mut self, turn: Turn) {
        match (*self, turn) {
            (Dir::N, Turn::L) | (Dir::S, Turn::R) => *self = Dir::W,
            (Dir::N, Turn::R) | (Dir::S, Turn::L) => *self = Dir::E,
            (Dir::E, Turn::L) | (Dir::W, Turn::R) => *self = Dir::N,
            (Dir::E, Turn::R) | (Dir::W, Turn::L) => *self = Dir::S,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir2 {
    U,
    D,
    L,
    R,
}

impl<T> Display for Pt<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl FromStr for Dir2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::U),
            "D" => Ok(Self::D),
            "L" => Ok(Self::L),
            "R" => Ok(Self::R),
            _ => Err(anyhow!("{s} is not a valid Dir2")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Turn {
    L,
    R,
}

impl<T> Add<Pt<T>> for Pt<T>
where
    T: Add<Output = T>,
{
    type Output = Pt<T>;

    fn add(self, rhs: Pt<T>) -> Self::Output {
        Pt {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl<T> AddAssign<Dir> for Pt<T>
where
    T: Num + NumAssign,
{
    fn add_assign(&mut self, rhs: Dir) {
        match rhs {
            Dir::N => self.y -= T::one(),
            Dir::S => self.y += T::one(),
            Dir::E => self.x += T::one(),
            Dir::W => self.x -= T::one(),
        }
    }
}

impl<T> Sub<Pt<T>> for Pt<T>
where
    T: Sub<Output = T>,
{
    type Output = Pt<T>;

    fn sub(self, rhs: Pt<T>) -> Self::Output {
        Pt {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub const ORIGINI32: Pt<i32> = Pt { x: 0, y: 0 };

impl<T> Pt<T>
where
    T: Copy + Num + Euclid + Ord,
{
    pub fn normalize(self) -> Self {
        let gcd = gcd(self.x, self.y);
        Self {
            x: self.x / gcd,
            y: self.y / gcd,
        }
    }
}

impl Pt<i32> {
    pub fn manhattan_distance(&self, other: &Pt<i32>) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl<T> AddAssign<Pt<T>> for Pt<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Pt<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> From<(T, T)> for Pt<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T> From<Pt<T>> for (T, T) {
    fn from(value: Pt<T>) -> Self {
        (value.x, value.y)
    }
}

impl<T> FromStr for Pt<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(',')
            .ok_or(anyhow!("{s} must include a comma to be parsed as Pt"))?;
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pt3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> From<Pt3<T>> for (T, T, T) {
    fn from(value: Pt3<T>) -> Self {
        (value.x, value.y, value.z)
    }
}

impl<T> From<(T, T, T)> for Pt3<T> {
    fn from(value: (T, T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl<T> Add for Pt3<T>
where
    T: Add<Output = T>,
{
    type Output = Pt3<T>;

    fn add(self, rhs: Pt3<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Pt3<T>
where
    T: Sub<Output = T>,
{
    type Output = Pt3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Pt3<isize> {
    pub fn abs_norm(&self, other: &Pt3<isize>) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

impl<T> FromStr for Pt3<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arr: [&str; 3] = s
            .trim()
            .trim_start_matches(['(', '<'])
            .trim_end_matches([')', '>'])
            .split(',')
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Ok(Self {
            x: arr[0].parse()?,
            y: arr[1].parse()?,
            z: arr[2].parse()?,
        })
    }
}

impl<T> Div<T> for Pt3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Pt3<T>;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> Mul<T> for Pt3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Pt3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.y * rhs,
        }
    }
}

impl<T> MulAssign<T> for Pt3<T>
where
    T: MulAssign<T> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T> Display for Pt3<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}
