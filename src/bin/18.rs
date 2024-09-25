use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    str::FromStr,
};

use advent_of_code::util::point::{Dir, Pt};
use anyhow::{anyhow, bail};
use strum::IntoEnumIterator;

advent_of_code::solution!(18);

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    struct Keys: u32 {
        const A = 1 << 0;
        const B = 1 << 1;
        const C = 1 << 2;
        const D = 1 << 3;
        const E = 1 << 4;
        const F = 1 << 5;
        const G = 1 << 6;
        const H = 1 << 7;
        const I = 1 << 8;
        const J = 1 << 9;
        const K = 1 << 10;
        const L = 1 << 11;
        const M = 1 << 12;
        const N = 1 << 13;
        const O = 1 << 14;
        const P = 1 << 15;
        const Q = 1 << 16;
        const R = 1 << 17;
        const S = 1 << 18;
        const T = 1 << 19;
        const U = 1 << 20;
        const V = 1 << 21;
        const W = 1 << 22;
        const X = 1 << 23;
        const Y = 1 << 24;
        const Z = 1 << 25;
    }
}

impl TryFrom<char> for Keys {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let mut buf = [0; 4];
        if value.is_ascii_alphabetic() {
            let value = value.to_ascii_uppercase();
            bitflags::parser::from_str_strict(value.encode_utf8(&mut buf))
                .map_err(|e| anyhow!("Invalid key or gate {e}"))
        } else {
            Err(anyhow!(
                "`{value}` must be an alphabetic char to be a gate or key"
            ))
        }
    }
}

#[derive(Debug)]
struct Map {
    key_to_key: HashMap<char, HashMap<char, (u32, Keys)>>,
    min_d: u32,
    num_keys: u32,
}

impl Map {
    fn from_input(input: Input) -> Self {
        let mut key_to_key: HashMap<_, HashMap<_, _>> = HashMap::new();
        let mut min_d = u32::MAX;
        let num_keys = input
            .keys
            .iter()
            .filter(|(k, _)| k.is_ascii_alphabetic())
            .count() as u32;
        for (i, (key, from)) in input.keys.iter().enumerate() {
            for (other, to) in &input.keys[i + 1..] {
                if let Some((len, gates)) = input.a_star(*from, *to) {
                    key_to_key
                        .entry(*key)
                        .or_default()
                        .insert(*other, (len, gates));
                    key_to_key
                        .entry(*other)
                        .or_default()
                        .insert(*key, (len, gates));
                    min_d = min_d.min(len);
                }
            }
        }
        Self {
            key_to_key,
            min_d,
            num_keys,
        }
    }

    fn neighbors(&self, c: char, seen: Keys) -> impl Iterator<Item = (char, u32)> + use<'_> {
        self.key_to_key[&c]
            .iter()
            .filter_map(move |(&to, (l, gates))| {
                (!seen.contains(to.try_into().unwrap_or(Keys::empty())) && seen.contains(*gates))
                    .then_some((to, *l))
            })
    }

    fn neighbors_part_two(
        &self,
        cs: [char; 4],
        seen: Keys,
    ) -> impl Iterator<Item = (char, usize, u32)> + use<'_> {
        cs.into_iter().enumerate().flat_map(move |(i, c)| {
            self.key_to_key[&c]
                .iter()
                .filter_map(move |(&to, (l, gates))| {
                    (!seen.contains(to.try_into().unwrap_or(Keys::empty()))
                        && seen.contains(*gates))
                    .then_some((to, i, *l))
                })
        })
    }

    fn find_path(&self) -> Option<u32> {
        let mut open_set = BinaryHeap::new();
        let h = |k: (char, Keys)| self.min_d * (self.num_keys - k.1.bits().count_ones());
        open_set.push(Node2 {
            c: '@',
            keys: Keys::empty(),
            len: self.min_d,
        });
        let mut g_score = HashMap::new();
        g_score.insert(('@', Keys::empty()), 0);
        while let Some(current) = open_set.pop() {
            if current.keys.bits().count_ones() == self.num_keys {
                return Some(current.len);
            }
            for (next_key, distance) in self.neighbors(current.c, current.keys) {
                let tentative_gscore = g_score[&(current.c, current.keys)] + distance;
                let new_keys = current.keys | next_key.try_into().unwrap();
                if tentative_gscore
                    < g_score
                        .get(&(next_key, new_keys))
                        .copied()
                        .unwrap_or(u32::MAX)
                {
                    g_score.insert((next_key, new_keys), tentative_gscore);
                    open_set.push(Node2 {
                        c: next_key,
                        keys: new_keys,
                        len: tentative_gscore + h((next_key, new_keys)),
                    });
                }
            }
        }
        None
    }

    fn find_path_part_two(&self) -> Option<u32> {
        let mut open_set = BinaryHeap::new();
        let h = |k: Keys| self.min_d * (self.num_keys - k.bits().count_ones());
        open_set.push(Node3 {
            cs: ['1', '2', '3', '4'],
            keys: Keys::empty(),
            len: self.min_d,
        });
        let mut g_score = HashMap::new();
        g_score.insert((['1', '2', '3', '4'], Keys::empty()), 0);
        while let Some(current) = open_set.pop() {
            if current.keys.bits().count_ones() == self.num_keys {
                return Some(current.len);
            }
            for (next_key, index, distance) in self.neighbors_part_two(current.cs, current.keys) {
                let tentative_gscore = g_score[&(current.cs, current.keys)] + distance;
                let new_keys = current.keys | next_key.try_into().unwrap();
                let mut new_place = current.cs;
                new_place[index] = next_key;
                if tentative_gscore
                    < g_score
                        .get(&(new_place, new_keys))
                        .copied()
                        .unwrap_or(u32::MAX)
                {
                    g_score.insert((new_place, new_keys), tentative_gscore);
                    open_set.push(Node3 {
                        cs: new_place,
                        keys: new_keys,
                        len: tentative_gscore + h(new_keys),
                    });
                }
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node2 {
    c: char,
    keys: Keys,
    len: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node3 {
    cs: [char; 4],
    keys: Keys,
    len: u32,
}

impl PartialOrd for Node3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node3 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.len.cmp(&other.len).reverse()
    }
}

impl PartialOrd for Node2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.len.cmp(&other.len).reverse()
    }
}

fn h(p1: Pt<usize>, p2: Pt<usize>) -> usize {
    p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y)
}

impl Input {
    fn into_part_two(mut self) -> Self {
        let (start, start_index) = self
            .keys
            .iter()
            .enumerate()
            .find(|(_, k)| k.0 == '@')
            .map(|(i, k)| (k.1, i))
            .unwrap();
        for d in Dir::iter() {
            self.walls.insert(start.checked_add_dir(d).unwrap());
        }
        self.walls.insert(start);
        self.keys.remove(start_index);
        self.keys.push(('1', start + Dir::N + Dir::E));
        self.keys.push(('2', start + Dir::N + Dir::W));
        self.keys.push(('3', start + Dir::S + Dir::E));
        self.keys.push(('4', start + Dir::S + Dir::W));
        self
    }

    fn neighbors(&self, pt: Pt<usize>) -> impl Iterator<Item = Pt<usize>> + use<'_> {
        Dir::iter()
            .filter_map(move |d| pt.checked_add_dir(d))
            .filter(|p| !self.walls.contains(p))
    }

    fn reconstruct_path(
        &self,
        to: Pt<usize>,
        came_from: HashMap<Pt<usize>, Pt<usize>>,
    ) -> (u32, Keys) {
        let mut current = to;
        let mut gates = Keys::empty();
        let mut dist = 0;
        while let Some(from) = came_from.get(&current) {
            dist += 1;
            if let Some(&gate) = self.gates.get(from) {
                gates |= gate.try_into().unwrap();
            }
            current = *from;
        }
        (dist, gates)
    }

    fn a_star(&self, from: Pt<usize>, to: Pt<usize>) -> Option<(u32, Keys)> {
        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let h = |p| h(p, to);
        open_set.push(Node {
            pt: from,
            score: h(from),
        });
        let mut g_score = HashMap::new();
        g_score.insert(from, 0);
        while let Some(Node { pt: current, .. }) = open_set.pop() {
            if current == to {
                return Some(self.reconstruct_path(to, came_from));
            }

            for neighbor in self.neighbors(current) {
                let tentative_g_score = g_score[&current] + 1;
                if tentative_g_score < g_score.get(&neighbor).copied().unwrap_or(usize::MAX) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    open_set.push(Node {
                        pt: neighbor,
                        score: tentative_g_score + h(neighbor),
                    })
                }
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    pt: Pt<usize>,
    score: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

#[derive(Debug)]
struct Input {
    keys: Vec<(char, Pt<usize>)>,
    gates: HashMap<Pt<usize>, char>,
    walls: HashSet<Pt<usize>>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.parse::<Input>().unwrap();
    let map = Map::from_input(input);
    map.find_path()
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.parse::<Input>().unwrap().into_part_two();
    let map = Map::from_input(input);
    map.find_path_part_two()
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut keys, mut gates, mut walls) = (Vec::new(), HashMap::new(), HashSet::new());
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.char_indices() {
                match c {
                    '#' => {
                        walls.insert(Pt { x, y });
                    }
                    c if c.is_ascii_lowercase() || c == '@' => {
                        keys.push((c, Pt { x, y }));
                    }
                    c if c.is_ascii_uppercase() => {
                        gates.insert(Pt { x, y }, c);
                    }
                    '.' => {}
                    _ => bail!("Unexpected char {c} in input, line {y}, col {x}"),
                }
            }
        }
        Ok(Self { keys, gates, walls })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(86));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(132));
    }

    #[test]
    fn test_part_one_three() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_one_four() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(81));
    }

    #[test]
    fn test_part_two_five() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_six() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_two_seven() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 7,
        ));
        assert_eq!(result, Some(32));
    }

    #[test]
    fn test_part_two_eight() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 8,
        ));
        assert_eq!(result, Some(72));
    }
}
