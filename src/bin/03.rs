use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use advent_of_code::util::point::{Dir2, Pt, ORIGINI32};

advent_of_code::solution!(3);

#[derive(Debug)]
struct Path {
    map: HashSet<Pt<i32>>,
}

#[derive(Debug)]
struct PathWithSteps {
    map: HashMap<Pt<i32>, u32>,
}

impl Path {
    fn intersections<'a>(&'a self, other: &'a Path) -> impl Iterator<Item = &'a Pt<i32>> + 'a {
        self.map
            .iter()
            .filter(move |&p| *p != Pt { x: 0, y: 0 } && other.map.contains(p))
    }
}

impl FromStr for Path {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cur_loc = Pt::default();
        let mut map = HashSet::with_capacity(s.split(',').count());
        map.insert(cur_loc);
        for dir in s.split(',') {
            let inc = match dir[..1].parse()? {
                Dir2::U => Pt { x: 0, y: -1 },
                Dir2::D => Pt { x: 0, y: 1 },
                Dir2::L => Pt { x: -1, y: 0 },
                Dir2::R => Pt { x: 1, y: 0 },
            };
            for _ in 0..dir[1..].parse::<u32>()? {
                cur_loc += inc;
                map.insert(cur_loc);
            }
        }
        Ok(Self { map })
    }
}

impl FromStr for PathWithSteps {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cur_loc = Pt::default();
        let mut map = HashMap::with_capacity(s.split(',').count());
        let mut i = 0;
        for dir in s.split(',') {
            let inc = match dir[..1].parse()? {
                Dir2::U => Pt { x: 0, y: -1 },
                Dir2::D => Pt { x: 0, y: 1 },
                Dir2::L => Pt { x: -1, y: 0 },
                Dir2::R => Pt { x: 1, y: 0 },
            };
            for _ in 0..dir[1..].parse::<u32>()? {
                i += 1;
                cur_loc += inc;
                map.entry(cur_loc).or_insert(i);
            }
        }
        Ok(Self { map })
    }
}

impl PathWithSteps {
    fn intersections<'a>(&'a self, other: &'a PathWithSteps) -> impl Iterator<Item = u32> + 'a {
        self.map
            .iter()
            .filter_map(move |(p1, s1)| other.map.get(p1).map(|s2| s1 + s2))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let map1 = lines.next().unwrap().parse::<Path>().unwrap();
    let map2 = lines.next().unwrap().parse::<Path>().unwrap();
    map1.intersections(&map2)
        .map(|pt| pt.manhattan_distance(&ORIGINI32))
        .min()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let map1 = lines.next().unwrap().parse::<PathWithSteps>().unwrap();
    let map2 = lines.next().unwrap().parse::<PathWithSteps>().unwrap();
    map1.intersections(&map2).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(159));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(135));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_part_two_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(610));
    }

    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(410));
    }
}
