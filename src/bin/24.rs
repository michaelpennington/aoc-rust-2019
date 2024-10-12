use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use anyhow::ensure;
use bitflags::bitflags;

advent_of_code::solution!(24);

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    struct Space: u32 {
        const S00 = 1 << 0;
        const S10 = 1 << 1;
        const S20 = 1 << 2;
        const S30 = 1 << 3;
        const S40 = 1 << 4;
        const S01 = 1 << 5;
        const S11 = 1 << 6;
        const S21 = 1 << 7;
        const S31 = 1 << 8;
        const S41 = 1 << 9;
        const S02 = 1 << 10;
        const S12 = 1 << 11;
        const S22 = 1 << 12;
        const S32 = 1 << 13;
        const S42 = 1 << 14;
        const S03 = 1 << 15;
        const S13 = 1 << 16;
        const S23 = 1 << 17;
        const S33 = 1 << 18;
        const S43 = 1 << 19;
        const S04 = 1 << 20;
        const S14 = 1 << 21;
        const S24 = 1 << 22;
        const S34 = 1 << 23;
        const S44 = 1 << 24;
    }
}

impl Space {
    fn neighbors_set(&self, x: u32, y: u32) -> u32 {
        self.neighbors(x, y).intersection(*self).bits().count_ones()
    }

    fn neighbors(&self, x: u32, y: u32) -> Self {
        assert!(x < 5 && y < 5);
        match (x, y) {
            (0, 0) => Self::S10 | Self::S01,
            (0, 1) => Self::S11 | Self::S02 | Self::S00,
            (0, 2) => Self::S12 | Self::S03 | Self::S01,
            (0, 3) => Self::S13 | Self::S04 | Self::S02,
            (0, 4) => Self::S14 | Self::S03,
            (1, 0) => Self::S00 | Self::S11 | Self::S20,
            (1, 1) => Self::S01 | Self::S10 | Self::S21 | Self::S12,
            (1, 2) => Self::S02 | Self::S22 | Self::S11 | Self::S13,
            (1, 3) => Self::S03 | Self::S23 | Self::S12 | Self::S14,
            (1, 4) => Self::S04 | Self::S24 | Self::S13,
            (2, 0) => Self::S10 | Self::S30 | Self::S21,
            (2, 1) => Self::S20 | Self::S22 | Self::S11 | Self::S31,
            (2, 2) => Self::S12 | Self::S32 | Self::S21 | Self::S23,
            (2, 3) => Self::S13 | Self::S33 | Self::S22 | Self::S24,
            (2, 4) => Self::S14 | Self::S34 | Self::S23,
            (3, 0) => Self::S20 | Self::S40 | Self::S31,
            (3, 1) => Self::S21 | Self::S41 | Self::S30 | Self::S32,
            (3, 2) => Self::S22 | Self::S42 | Self::S31 | Self::S33,
            (3, 3) => Self::S23 | Self::S43 | Self::S32 | Self::S34,
            (3, 4) => Self::S24 | Self::S44 | Self::S33,
            (4, 0) => Self::S30 | Self::S41,
            (4, 1) => Self::S31 | Self::S40 | Self::S42,
            (4, 2) => Self::S32 | Self::S41 | Self::S43,
            (4, 3) => Self::S33 | Self::S42 | Self::S44,
            (4, 4) => Self::S34 | Self::S43,
            _ => unreachable!(),
        }
    }

    fn mutate(&self) -> Self {
        let mut new_space = *self;
        for y in 0..5 {
            for x in 0..5 {
                let bit = Self::from_bits_truncate(1 << (y * 5 + x));
                let bit_set = self.contains(bit);
                match (bit_set, self.neighbors_set(x, y)) {
                    (false, 0 | 3 | 4) | (true, 1) => {}
                    (true, 0 | 2 | 3 | 4) => {
                        new_space.set(bit, false);
                    }
                    (false, 1 | 2) => {
                        new_space.set(bit, true);
                    }
                    _ => unreachable!(),
                }
            }
        }
        new_space
    }

    fn neighbors_recursive(&self, x: u32, y: u32) -> (Space, Space, Space) {
        assert!(x < 5 && y < 5 && (x != 2 || y != 2));
        match (x, y) {
            (0, 0) => (Self::S12 | Self::S21, Self::S10 | Self::S01, Self::empty()),
            (0, 1) => (Self::S12, Self::S00 | Self::S02 | Self::S11, Self::empty()),
            (0, 2) => (Self::S12, Self::S12 | Self::S01 | Self::S03, Self::empty()),
            (0, 3) => (Self::S12, Self::S13 | Self::S02 | Self::S04, Self::empty()),
            (0, 4) => (Self::S12 | Self::S23, Self::S14 | Self::S03, Self::empty()),
            (1, 0) => (Self::S21, Self::S00 | Self::S20 | Self::S11, Self::empty()),
            (1, 1) => (
                Self::empty(),
                Self::S01 | Self::S21 | Self::S10 | Self::S12,
                Self::empty(),
            ),
            (1, 2) => (
                Self::empty(),
                Self::S11 | Self::S13 | Self::S02,
                Self::S00 | Self::S01 | Self::S02 | Self::S03 | Self::S04,
            ),
            (1, 3) => (
                Self::empty(),
                Self::S03 | Self::S23 | Self::S12 | Self::S14,
                Self::empty(),
            ),
            (1, 4) => (Self::S23, Self::S04 | Self::S24 | Self::S13, Self::empty()),
            (2, 0) => (Self::S21, Self::S10 | Self::S30 | Self::S21, Self::empty()),
            (2, 1) => (
                Self::empty(),
                Self::S20 | Self::S11 | Self::S31,
                Self::S00 | Self::S10 | Self::S20 | Self::S30 | Self::S40,
            ),
            (2, 2) => panic!("Tried to calculate recursive neighbors of (2,2)"),
            (2, 3) => (
                Self::empty(),
                Self::S24 | Self::S13 | Self::S33,
                Self::S04 | Self::S14 | Self::S24 | Self::S34 | Self::S44,
            ),
            (2, 4) => (Self::S23, Self::S14 | Self::S34 | Self::S23, Self::empty()),
            (3, 0) => (Self::S21, Self::S20 | Self::S40 | Self::S31, Self::empty()),
            (3, 1) => (
                Self::empty(),
                Self::S30 | Self::S32 | Self::S21 | Self::S41,
                Self::empty(),
            ),
            (3, 2) => (
                Self::empty(),
                Self::S31 | Self::S33 | Self::S42,
                Self::S40 | Self::S41 | Self::S42 | Self::S43 | Self::S44,
            ),
            (3, 3) => (
                Self::empty(),
                Self::S32 | Self::S34 | Self::S23 | Self::S43,
                Self::empty(),
            ),
            (3, 4) => (Self::S23, Self::S24 | Self::S44 | Self::S33, Self::empty()),
            (4, 0) => (Self::S21 | Self::S32, Self::S30 | Self::S41, Self::empty()),
            (4, 1) => (Self::S32, Self::S40 | Self::S42 | Self::S31, Self::empty()),
            (4, 2) => (Self::S32, Self::S41 | Self::S43 | Self::S32, Self::empty()),
            (4, 3) => (Self::S32, Self::S42 | Self::S44 | Self::S33, Self::empty()),
            (4, 4) => (Self::S32 | Self::S23, Self::S34 | Self::S43, Self::empty()),
            _ => unreachable!(),
        }
    }

    fn count_neighbors_recursive(&self, above: &Space, below: &Space, x: u32, y: u32) -> u32 {
        let (above_n, self_n, below_n) = self.neighbors_recursive(x, y);
        above_n.intersection(*above).bits().count_ones()
            + self_n.intersection(*self).bits().count_ones()
            + below_n.intersection(*below).bits().count_ones()
    }
}

impl FromStr for Space {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut flags = Space::empty();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.char_indices() {
                ensure!(x < 5 && y < 5, "Index ({x}, {y}) out of bounds");
                if c == '#' {
                    flags |= Space::from_bits_truncate(1 << (y * 5 + x));
                }
            }
        }
        Ok(flags)
    }
}

struct RecursiveSpace {
    generation: usize,
    spaces: VecDeque<Space>,
}

impl FromStr for RecursiveSpace {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut flags = Space::empty();
        let mut spaces = VecDeque::with_capacity(200);
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.char_indices() {
                ensure!(x < 5 && y < 5, "Index ({x}, {y}) out of bounds");
                if c == '#' && (x != 2 || y != 2) {
                    flags |= Space::from_bits_truncate(1 << (y * 5 + x));
                }
            }
        }
        spaces.push_front(flags);
        Ok(Self {
            spaces,
            generation: 0,
        })
    }
}

const EMPTY_SPACE: Space = Space::empty();

impl RecursiveSpace {
    fn mutate(&mut self) {
        if self.generation % 2 == 0 {
            self.spaces.push_front(Space::empty());
            self.spaces.push_back(Space::empty());
        }
        let mut new_spaces = self.spaces.clone();
        for i in 0..self.spaces.len() {
            let above = i
                .checked_sub(1)
                .and_then(|i| self.spaces.get(i))
                .unwrap_or(&EMPTY_SPACE);
            let below = self.spaces.get(i + 1).unwrap_or(&EMPTY_SPACE);
            let current = self.spaces.get(i).unwrap();
            let new_space = new_spaces.get_mut(i).unwrap();
            for y in 0..5 {
                for x in 0..5 {
                    if x == 2 && y == 2 {
                        continue;
                    }
                    let bit = Space::from_bits_truncate(1 << (5 * y + x));
                    let bit_set = current.contains(bit);
                    match (
                        bit_set,
                        current.count_neighbors_recursive(above, below, x, y),
                    ) {
                        (false, 0 | 3 | 4 | 5 | 6 | 7 | 8) | (true, 1) => {}
                        (true, 0 | 2 | 3 | 4 | 5 | 6 | 7 | 8) => {
                            new_space.set(bit, false);
                        }
                        (false, 1 | 2) => {
                            new_space.set(bit, true);
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
        self.spaces = new_spaces;
        self.generation += 1;
    }

    fn count_bugs(&self) -> u32 {
        self.spaces.iter().map(|s| s.bits().count_ones()).sum()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut space = input.parse::<Space>().unwrap();
    let mut seen_spaces = HashSet::new();
    while seen_spaces.insert(space) {
        space = space.mutate();
    }
    Some(space.bits())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut recursive_space = input.parse::<RecursiveSpace>().unwrap();
    for _ in 0..200 {
        recursive_space.mutate();
    }
    Some(recursive_space.count_bugs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2129920));
    }
}
