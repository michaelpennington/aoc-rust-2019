use std::{collections::VecDeque, fmt::Display, iter::repeat_n, str::FromStr};

use anyhow::anyhow;

advent_of_code::solution!(16);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DigitList {
    digits: Vec<u8>,
}

impl Display for DigitList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in &self.digits {
            write!(f, "{digit}")?;
        }
        Ok(())
    }
}

impl DigitList {
    fn fft(&self) -> Self {
        let mut digits = Vec::with_capacity(self.digits.len());
        digits.extend((0..self.digits.len()).map(|n| self.construct_nth(n)));
        Self { digits }
    }

    fn construct_nth(&self, n: usize) -> u8 {
        let mut total: i32 = 0;
        for (mul, &input) in repeat_n(0, n + 1)
            .chain(repeat_n(1, n + 1))
            .chain(repeat_n(0, n + 1))
            .chain(repeat_n(-1, n + 1))
            .cycle()
            .skip(1)
            .zip(&self.digits)
        {
            total += (input as i32) * mul;
        }
        total = total.abs();
        (total % 10) as u8
    }

    fn fast_fft(&self) -> Self {
        let mut digits = VecDeque::with_capacity(self.digits.len());
        let mut acc = 0;
        for i in 1..=self.digits.len() {
            acc = (acc + self.digits[self.digits.len() - i]) % 10;
            digits.push_front(acc);
        }
        Self {
            digits: digits.into(),
        }
    }
}

impl From<&[u8]> for DigitList {
    fn from(value: &[u8]) -> Self {
        Self {
            digits: value.to_vec(),
        }
    }
}

impl FromStr for DigitList {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::with_capacity(s.len());
        for c in s.chars() {
            digits.push(
                c.to_digit(10)
                    .ok_or_else(|| anyhow!("{c} is not a valid digit"))? as u8,
            )
        }
        Ok(Self { digits })
    }
}

pub fn part_one(input: &str) -> Option<DigitList> {
    let mut list = input.trim().parse::<DigitList>().unwrap();
    for _ in 0..100 {
        list = list.fft();
    }
    list.digits.truncate(8);
    Some(list)
}

pub fn part_two(input: &str) -> Option<DigitList> {
    let list = input.trim().parse::<DigitList>().unwrap();
    let offset = list.digits.iter().take(7).copied().collect::<Vec<_>>();
    let offset = {
        let mut off = 0;
        let mut mul = 1_000_000;
        for num in offset {
            off += mul * (num as usize);
            mul /= 10;
        }
        off
    };
    let mut digits = list.digits.repeat(10000);
    digits.drain(..offset);
    let mut list = DigitList { digits };
    for _ in 0..100 {
        list = list.fast_fft();
    }
    list.digits.truncate(8);
    Some(list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some((&[2, 4, 1, 7, 6, 1, 7, 6][..]).into()));
    }

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some((&[7, 3, 7, 4, 5, 4, 1, 8][..]).into()));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some((&[5, 2, 4, 3, 2, 1, 3, 3][..]).into()));
    }
}
