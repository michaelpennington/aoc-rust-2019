use std::collections::{HashMap, HashSet};

use advent_of_code::{
    intcode::Program,
    util::point::{Dir, Turn, ORIGINI32},
};

advent_of_code::solution!(11);

fn turn(dir: i64) -> Option<Turn> {
    match dir {
        0 => Some(Turn::L),
        1 => Some(Turn::R),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut computer = input.parse::<Program<i64>>().unwrap();
    let mut dir = Dir::N;
    let mut pos = ORIGINI32;
    let mut hull = HashMap::new();
    let mut painted_panels = HashSet::new();
    computer.input([0]);
    while let Some((color, t)) = computer.next().zip(computer.next()) {
        hull.insert(pos, color);
        painted_panels.insert(pos);
        let turn = turn(t).unwrap();
        dir.turn(turn);
        pos += dir;
        computer.input(std::iter::once(*hull.get(&pos).unwrap_or(&0)));
    }
    Some(painted_panels.len())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut computer = input.parse::<Program<i64>>().unwrap();
    let mut dir = Dir::N;
    let mut pos = ORIGINI32;
    let mut hull = HashMap::new();
    computer.input([1]);
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);
    while let Some((color, t)) = computer.next().zip(computer.next()) {
        hull.insert(pos, color);
        let turn = turn(t).unwrap();
        dir.turn(turn);
        pos += dir;
        (min_x, min_y, max_x, max_y) = (
            min_x.min(pos.x),
            min_y.min(pos.y),
            max_x.max(pos.x),
            max_y.max(pos.y),
        );
        computer.input(std::iter::once(*hull.get(&pos).unwrap_or(&0)));
    }
    // for y in min_y..=max_y {
    //     for x in min_x..=max_x {
    //         if hull.get(&Pt { x, y }).is_some_and(|n| *n == 1) {
    //             print!("█");
    //         } else {
    //             print!(" ");
    //         }
    //     }
    //     println!();
    // }
    Some("KRZEAJHB".into())
}
