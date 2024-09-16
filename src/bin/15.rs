use std::collections::HashMap;

use advent_of_code::{intcode::Program, util::graph::Graph};
use rand::{rngs::ThreadRng, Rng};

advent_of_code::solution!(15);

struct Map {
    tiles: HashMap<(i32, i32), bool>,
}

impl Graph for Map {
    type Node = (i32, i32);

    fn neighbors(&self, node: Self::Node) -> impl Iterator<Item = (Self::Node, usize)> {
        (node.0 - 1..=node.0 + 1)
            .flat_map(move |x| (node.1 - 1..=node.1 + 1).map(move |y| (x, y)))
            .filter(move |p| (p.0 != node.0 && p.1 == node.1) || (p.0 == node.0 && p.1 != node.1))
            .filter(|p| {
                self.tiles
                    .get(p)
                    .copied()
                    .is_some_and(std::convert::identity)
            })
            .map(|p| (p, 1))
    }

    fn h(from: Self::Node, to: Self::Node) -> usize {
        (from.0.abs_diff(to.0) + from.1.abs_diff(to.1)) as usize
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut computer: Program<i32> = input.parse().unwrap();
    let (mut x, mut y) = (0, 0);
    let mut tiles = HashMap::new();
    let mut rng = ThreadRng::default();
    tiles.insert((x, y), true);
    let mut already_tried = Vec::with_capacity(4);
    let mut oxygen = (0, 0);
    for _ in 0..1000000 {
        let dir = {
            loop {
                let candidate = rng.gen_range(1..=4);
                if !already_tried.contains(&candidate) {
                    break candidate;
                }
            }
        };
        computer.input(std::iter::once(dir));
        let status = computer.next().unwrap();
        let new_pos = match dir {
            1 => (x, y - 1),
            2 => (x, y + 1),
            3 => (x - 1, y),
            4 => (x + 1, y),
            _ => unreachable!(),
        };
        match status {
            0 => {
                tiles.insert(new_pos, false);
                already_tried.push(dir);
            }
            1 => {
                (x, y) = new_pos;
                tiles.insert(new_pos, true);
                already_tried.clear();
            }
            2 => {
                (x, y) = new_pos;
                tiles.insert(new_pos, true);
                oxygen = new_pos;
            }
            _ => unreachable!(),
        }
    }
    let map = Map { tiles };
    map.a_star_distance((0, 0), oxygen)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut computer: Program<i32> = input.parse().unwrap();
    let (mut x, mut y) = (0, 0);
    let mut tiles = HashMap::new();
    let mut rng = ThreadRng::default();
    tiles.insert((x, y), true);
    let mut already_tried = Vec::with_capacity(4);
    let mut oxygen = (0, 0);
    for _ in 0..1000000 {
        let dir = {
            loop {
                let candidate = rng.gen_range(1..=4);
                if !already_tried.contains(&candidate) {
                    break candidate;
                }
            }
        };
        computer.input(std::iter::once(dir));
        let status = computer.next().unwrap();
        let new_pos = match dir {
            1 => (x, y - 1),
            2 => (x, y + 1),
            3 => (x - 1, y),
            4 => (x + 1, y),
            _ => unreachable!(),
        };
        match status {
            0 => {
                tiles.insert(new_pos, false);
                already_tried.push(dir);
            }
            1 => {
                (x, y) = new_pos;
                tiles.insert(new_pos, true);
                already_tried.clear();
            }
            2 => {
                (x, y) = new_pos;
                tiles.insert(new_pos, true);
                oxygen = new_pos;
            }
            _ => unreachable!(),
        }
    }
    let map = Map { tiles };
    map.tiles
        .iter()
        .filter(|(_, e)| **e)
        .map(|(v, _)| map.a_star_distance(oxygen, *v).unwrap())
        .max()
}
