use std::{collections::HashMap, str::FromStr};

use advent_of_code::util::{
    graph::Graph,
    point::{Dir, Pt},
};
use anyhow::bail;
use strum::IntoEnumIterator;

advent_of_code::solution!(20);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Space {
    Empty,
    Portal(Pt<usize>),
}

impl Space {
    fn is_portal(&self) -> Option<Pt<usize>> {
        match self {
            Space::Empty => None,
            Space::Portal(p) => Some(*p),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Map {
    spaces: HashMap<Pt<usize>, Space>,
    start: Pt<usize>,
    finish: Pt<usize>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SpaceV2 {
    Empty,
    PortalOuter(Pt<usize>),
    PortalInner(Pt<usize>),
}

impl SpaceV2 {
    fn is_portal_level(&self, level: usize) -> Option<(Pt<usize>, usize)> {
        match self {
            SpaceV2::Empty => None,
            SpaceV2::PortalOuter(_) if level == 0 => None,
            SpaceV2::PortalOuter(p) => Some((*p, level - 1)),
            SpaceV2::PortalInner(p) => Some((*p, level + 1)),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct MapV2 {
    spaces: HashMap<Pt<usize>, SpaceV2>,
    start: Pt<usize>,
    finish: Pt<usize>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Boundary {
    outer_nw: Pt<usize>,
    outer_se: Pt<usize>,
    inner_nw: Pt<usize>,
    inner_se: Pt<usize>,
}

impl Boundary {
    fn on_boundary(&self, pt: Pt<usize>) -> Option<BoundaryCondition> {
        use BoundaryCondition::*;
        if pt.x == self.outer_nw.x {
            Some(OuterWest)
        } else if pt.x == self.outer_se.x {
            Some(OuterEast)
        } else if pt.y == self.outer_nw.y {
            Some(OuterNorth)
        } else if pt.y == self.outer_se.y {
            Some(OuterSouth)
        } else if pt.x == self.inner_nw.x && (self.inner_nw.y..=self.inner_se.y).contains(&pt.y) {
            Some(InnerWest)
        } else if pt.x == self.inner_se.x && (self.inner_nw.y..=self.inner_se.y).contains(&pt.y) {
            Some(InnerEast)
        } else if pt.y == self.inner_nw.y && (self.inner_nw.x..=self.inner_se.x).contains(&pt.x) {
            Some(InnerNorth)
        } else if pt.y == self.inner_se.y && (self.inner_nw.x..=self.inner_se.x).contains(&pt.x) {
            Some(InnerSouth)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum BoundaryCondition {
    OuterNorth,
    OuterSouth,
    OuterEast,
    OuterWest,
    InnerNorth,
    InnerSouth,
    InnerEast,
    InnerWest,
}

impl FromStr for MapV2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BoundaryCondition::*;
        let map: Vec<Vec<_>> = s.lines().map(|l| l.chars().collect()).collect();
        let mut outer_nw = Pt::default();
        let mut outer_se = Pt::default();
        let mut inner_nw = Pt::default();
        let mut inner_se = Pt::default();
        for y in 1..map.len() - 1 {
            for x in 1..map[0].len() - 1 {
                if map[y][x] == '#' && map[y - 1][x] == ' ' && map[y][x - 1] == ' ' {
                    outer_nw = Pt { x, y };
                } else if map[y][x] == '#' && map[y + 1][x] == ' ' && map[y][x + 1] == ' ' {
                    outer_se = Pt { x, y };
                } else if map[y][x] == ' ' && map[y - 1][x] == '#' && map[y][x - 1] == '#' {
                    inner_nw = Pt { x: x - 1, y: y - 1 };
                } else if map[y][x] == ' ' && map[y + 1][x] == '#' && map[y][x + 1] == '#' {
                    inner_se = Pt { x: x + 1, y: y + 1 };
                }
            }
        }

        let boundary = Boundary {
            outer_nw,
            outer_se,
            inner_nw,
            inner_se,
        };
        let mut spaces = HashMap::new();
        let mut waiting: HashMap<(_, _), _> = HashMap::new();
        let mut start = Pt { x: 0, y: 0 };
        let mut finish = Pt { x: 0, y: 0 };
        for y in 2..map.len() - 2 {
            for x in 2..map[0].len() - 2 {
                match map[y][x] {
                    '.' => {
                        let real_pt = Pt { x: x - 2, y: y - 2 };
                        if let Some(bc) = boundary.on_boundary(Pt { x, y }) {
                            let ident = match bc {
                                InnerSouth | OuterNorth => (map[y - 2][x], map[y - 1][x]),
                                InnerNorth | OuterSouth => (map[y + 1][x], map[y + 2][x]),
                                InnerWest | OuterEast => (map[y][x + 1], map[y][x + 2]),
                                OuterWest | InnerEast => (map[y][x - 2], map[y][x - 1]),
                            };
                            if ident == ('A', 'A') {
                                start = real_pt;
                                spaces.insert(real_pt, SpaceV2::Empty);
                                continue;
                            } else if ident == ('Z', 'Z') {
                                finish = real_pt;
                                spaces.insert(real_pt, SpaceV2::Empty);
                                continue;
                            }
                            if let Some(pt) = waiting.remove(&ident) {
                                match bc {
                                    InnerSouth | InnerEast | InnerWest | InnerNorth => {
                                        let pt = match pt {
                                            SpaceV2::PortalOuter(pt) => pt,
                                            _ => unreachable!(),
                                        };
                                        spaces.insert(real_pt, SpaceV2::PortalInner(pt));
                                        spaces.insert(pt, SpaceV2::PortalOuter(real_pt));
                                    }
                                    OuterSouth | OuterEast | OuterWest | OuterNorth => {
                                        let pt = match pt {
                                            SpaceV2::PortalInner(pt) => pt,
                                            _ => unreachable!(),
                                        };
                                        spaces.insert(real_pt, SpaceV2::PortalOuter(pt));
                                        spaces.insert(pt, SpaceV2::PortalInner(real_pt));
                                    }
                                }
                            } else {
                                match bc {
                                    InnerSouth | InnerEast | InnerWest | InnerNorth => {
                                        waiting.insert(ident, SpaceV2::PortalInner(real_pt));
                                    }
                                    OuterSouth | OuterEast | OuterWest | OuterNorth => {
                                        waiting.insert(ident, SpaceV2::PortalOuter(real_pt));
                                    }
                                }
                            }
                        } else {
                            spaces.insert(real_pt, SpaceV2::Empty);
                        }
                    }
                    '#' => {}
                    c if c == ' ' || c.is_ascii_uppercase() => {}
                    c => bail!("Unexpected character {c} in map"),
                }
            }
        }
        Ok(Self {
            spaces,
            start,
            finish,
        })
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BoundaryCondition::*;
        let map: Vec<Vec<_>> = s.lines().map(|l| l.chars().collect()).collect();
        let mut outer_nw = Pt::default();
        let mut outer_se = Pt::default();
        let mut inner_nw = Pt::default();
        let mut inner_se = Pt::default();
        for y in 1..map.len() - 1 {
            for x in 1..map[0].len() - 1 {
                if map[y][x] == '#' && map[y - 1][x] == ' ' && map[y][x - 1] == ' ' {
                    outer_nw = Pt { x, y };
                } else if map[y][x] == '#' && map[y + 1][x] == ' ' && map[y][x + 1] == ' ' {
                    outer_se = Pt { x, y };
                } else if map[y][x] == ' ' && map[y - 1][x] == '#' && map[y][x - 1] == '#' {
                    inner_nw = Pt { x: x - 1, y: y - 1 };
                } else if map[y][x] == ' ' && map[y + 1][x] == '#' && map[y][x + 1] == '#' {
                    inner_se = Pt { x: x + 1, y: y + 1 };
                }
            }
        }

        let boundary = Boundary {
            outer_nw,
            outer_se,
            inner_nw,
            inner_se,
        };
        let mut spaces = HashMap::new();
        let mut waiting: HashMap<(_, _), _> = HashMap::new();
        let mut start = Pt { x: 0, y: 0 };
        let mut finish = Pt { x: 0, y: 0 };
        for y in 2..map.len() - 2 {
            for x in 2..map[0].len() - 2 {
                match map[y][x] {
                    '.' => {
                        let real_pt = Pt { x: x - 2, y: y - 2 };
                        if let Some(bc) = boundary.on_boundary(Pt { x, y }) {
                            let ident = match bc {
                                InnerSouth | OuterNorth => (map[y - 2][x], map[y - 1][x]),
                                InnerNorth | OuterSouth => (map[y + 1][x], map[y + 2][x]),
                                InnerWest | OuterEast => (map[y][x + 1], map[y][x + 2]),
                                OuterWest | InnerEast => (map[y][x - 2], map[y][x - 1]),
                            };
                            if ident == ('A', 'A') {
                                start = real_pt;
                                spaces.insert(real_pt, Space::Empty);
                                continue;
                            } else if ident == ('Z', 'Z') {
                                finish = real_pt;
                                spaces.insert(real_pt, Space::Empty);
                                continue;
                            }
                            if let Some(pt) = waiting.remove(&ident) {
                                spaces.insert(real_pt, Space::Portal(pt));
                                spaces.insert(pt, Space::Portal(real_pt));
                            } else {
                                waiting.insert(ident, real_pt);
                            }
                        } else {
                            spaces.insert(real_pt, Space::Empty);
                        }
                    }
                    '#' => {}
                    c if c == ' ' || c.is_ascii_uppercase() => {}
                    c => bail!("Unexpected character {c} in map"),
                }
            }
        }
        Ok(Self {
            spaces,
            start,
            finish,
        })
    }
}

impl Graph for Map {
    type Node = Pt<usize>;

    fn neighbors(&self, node: Self::Node) -> impl Iterator<Item = (Self::Node, usize)> {
        Dir::iter()
            .filter_map(move |d| node.checked_add_dir(d))
            .filter_map(|p| match self.spaces.get(&p) {
                Some(_) => Some((p, 1)),
                _ => None,
            })
            .chain(
                self.spaces
                    .get(&node)
                    .and_then(|sp| sp.is_portal().map(|pt| (pt, 1))),
            )
    }

    fn h(_from: Self::Node, _to: Self::Node) -> usize {
        1
    }
}

impl Graph for MapV2 {
    type Node = (Pt<usize>, usize);

    fn neighbors(&self, node: Self::Node) -> impl Iterator<Item = (Self::Node, usize)> {
        Dir::iter()
            .filter_map(move |d| node.0.checked_add_dir(d))
            .filter_map(move |p| match self.spaces.get(&p) {
                Some(_) => Some(((p, node.1), 1)),
                _ => None,
            })
            .chain(
                self.spaces
                    .get(&node.0)
                    .and_then(|sp| sp.is_portal_level(node.1).map(|pt| (pt, 1))),
            )
    }

    fn h(_from: Self::Node, _to: Self::Node) -> usize {
        1
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = input.parse::<Map>().unwrap();
    map.a_star_distance(map.start, map.finish).map(|u| u - 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = input.parse::<MapV2>().unwrap();
    map.a_star_distance((map.start, 0), (map.finish, 0))
        .map(|u| u - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(58));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26));
    }

    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(396));
    }
}
