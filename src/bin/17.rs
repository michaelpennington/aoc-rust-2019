use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use advent_of_code::{
    intcode::Program,
    util::point::{Dir, Pt, Turn},
};
use strum::IntoEnumIterator;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<usize> {
    let computer = input.parse::<Program<i32>>().unwrap();
    let mut map = String::new();
    for c in computer {
        let c = char::from_u32(c as u32).unwrap();
        map.push(c);
    }
    let map = map
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut sum = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '#'
                && map
                    .get(y)
                    .is_some_and(|l| l.get(x + 1).is_some_and(|&c| c == '#'))
                && map
                    .get(y)
                    .is_some_and(|l| l.get(x.wrapping_sub(1)).is_some_and(|&c| c == '#'))
                && map
                    .get(y + 1)
                    .is_some_and(|l| l.get(x).is_some_and(|&c| c == '#'))
                && map
                    .get(y.wrapping_sub(1))
                    .is_some_and(|l| l.get(x).is_some_and(|&c| c == '#'))
            {
                sum += x * y;
            }
        }
    }
    Some(sum)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct _Map(Vec<Vec<char>>);

impl Index<Pt<usize>> for _Map {
    type Output = char;

    fn index(&self, index: Pt<usize>) -> &Self::Output {
        &self.0[index.y][index.x]
    }
}

impl IndexMut<Pt<usize>> for _Map {
    fn index_mut(&mut self, index: Pt<usize>) -> &mut Self::Output {
        &mut self.0[index.y][index.x]
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct _Path {
    turns: Vec<Turn>,
    lens: Vec<usize>,
}

impl Display for _Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.turns.iter().zip(&self.lens).peekable();
        while let Some((turn, len)) = iter.next() {
            if iter.peek().is_none() {
                write!(f, "{},{}", turn, len)?;
            } else {
                write!(f, "{turn},{len},")?;
            }
        }
        Ok(())
    }
}

impl _Map {
    fn _get(&self, index: Pt<usize>) -> Option<&char> {
        self.0.get(index.y).and_then(|l| l.get(index.x))
    }

    fn _get_mut(&mut self, index: Pt<usize>) -> Option<&mut char> {
        self.0.get_mut(index.y).and_then(|l| l.get_mut(index.x))
    }

    fn _get_path(&self, start: Pt<usize>, dir: Dir) -> _Path {
        let mut turns = Vec::new();
        let mut lens = Vec::new();
        let mut loc = start;
        let mut len = 0;
        let mut dir = dir;
        loop {
            if self._get(loc + dir).is_some_and(|c| *c == '#') {
                loc += dir;
                len += 1;
            } else {
                if len != 0 {
                    lens.push(len);
                    len = 0;
                }
                let mut cont = false;
                for turn in Turn::iter() {
                    if self._get(loc + (dir + turn)).is_some_and(|c| *c == '#') {
                        turns.push(turn);
                        dir.turn(turn);
                        cont = true;
                    }
                }
                if !cont {
                    break;
                }
            }
        }
        _Path { turns, lens }
    }
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut computer = input.parse::<Program<i32>>().unwrap();
    computer.set(0, 2);
    computer.input(b"A,A,B,C,A,C,A,B,C,B\n".iter().map(|c| *c as i32));
    computer.input(b"R,12,L,8,R,6\n".iter().map(|c| *c as i32));
    computer.input(b"R,12,L,6,R,6,R,8,R,6\n".iter().map(|c| *c as i32));
    computer.input(b"L,8,R,8,R,6,R,12\n".iter().map(|c| *c as i32));
    computer.input(b"n\n".iter().map(|c| *c as i32));
    let mut last = 0;
    for c in computer {
        last = c;
    }
    Some(last)
}
