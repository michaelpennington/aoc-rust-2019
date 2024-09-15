use std::collections::HashMap;

use advent_of_code::intcode::Program;
use anyhow::anyhow;

advent_of_code::solution!(13);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TryFrom<i32> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Empty),
            1 => Ok(Self::Wall),
            2 => Ok(Self::Block),
            3 => Ok(Self::Paddle),
            4 => Ok(Self::Ball),
            _ => Err(anyhow!("{value} is not a valid tile id")),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut program = input.parse::<Program<i32>>().unwrap();
    let mut tiles = HashMap::new();
    while let (Some(x), Some(y), Some(tile)) = (program.next(), program.next(), program.next()) {
        tiles.insert((x, y), <Tile as TryFrom<_>>::try_from(tile).unwrap());
    }
    Some(tiles.values().filter(|v| **v == Tile::Block).count())
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut program = input.parse::<Program<i32>>().unwrap();
    let mut tiles: HashMap<(i32, i32), Tile> = HashMap::new();
    program.set(0, 2);
    program.input([0]);
    let mut score = 0;
    while let (Some(x), Some(y), Some(tile)) = (program.next(), program.next(), program.next()) {
        if (x, y) == (-1, 0) {
            score = tile;
        } else {
            let tile = <Tile as TryFrom<_>>::try_from(tile).unwrap();
            if tile == Tile::Ball {
                if let Some(paddle_x) = tiles
                    .iter()
                    .find_map(|(&p, &t)| (t == Tile::Paddle).then_some(p.0))
                {
                    program.input([(x - paddle_x).signum()]);
                }
            }
            tiles.insert((x, y), tile);
        }
    }
    Some(score)
}
