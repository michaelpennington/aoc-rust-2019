use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use advent_of_code::util::point::{Pt, ORIGINI32};
use anyhow::anyhow;

advent_of_code::solution!(10);

struct Map {
    pts: HashSet<Pt<i32>>,
}

struct OrderedMap {
    loc: Pt<i32>,
    pts: Vec<VecDeque<Pt<i32>>>,
}

impl OrderedMap {
    fn construct(station: Pt<i32>, map: Map) -> Self {
        let mut stroids: Vec<_> = map
            .pts
            .into_iter()
            .filter(|&pt| pt != station)
            .map(|pt| pt - station)
            .collect();

        stroids.sort_by(|&pt1, &pt2| {
            let p1n = pt1.normalize();
            let p2n = pt2.normalize();
            (p2n.x * p1n.y).cmp(&(p2n.y * p1n.x)).then_with(|| {
                pt1.manhattan_distance(&ORIGINI32)
                    .cmp(&pt2.manhattan_distance(&ORIGINI32))
            })
        });
        let best = stroids.iter().position(|p| p.y < 0 && p.x == 0).unwrap();
        stroids.rotate_left(best);
        let mut pts: Vec<VecDeque<Pt<i32>>> = Vec::new();
        let mut cur_normalized = ORIGINI32;
        for pt in stroids {
            if pt.normalize() == cur_normalized {
                pts.last_mut().unwrap().push_back(pt);
            } else {
                cur_normalized = pt.normalize();
                pts.push(vec![pt].into());
            }
        }
        Self { loc: station, pts }
    }

    fn nth_stroid(&mut self, n: usize) -> Pt<i32> {
        let mut cur_loc = 0;
        let mut next = Pt { x: 0, y: 0 };
        for _ in 0..n {
            while self.pts[cur_loc].is_empty() {
                cur_loc = (cur_loc + 1) % self.pts.len();
            }
            next = self.pts[cur_loc].pop_front().unwrap();
            cur_loc = (cur_loc + 1) % self.pts.len();
        }
        next + self.loc
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pts = HashSet::new();
        for (y, l) in s.lines().enumerate() {
            for (x, c) in l.char_indices() {
                match c {
                    '#' => {
                        pts.insert(Pt {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    '.' => {}
                    _ => return Err(anyhow!("{c} is not a valid map char")),
                }
            }
        }
        Ok(Self { pts })
    }
}

impl Map {
    fn construct_los(&self) -> HashMap<Pt<i32>, HashSet<Pt<i32>>> {
        let mut out = HashMap::with_capacity(self.pts.len());
        for &pt1 in &self.pts {
            let mut pt_map = HashSet::new();
            for &pt2 in self.pts.iter().filter(|&p| *p != pt1) {
                pt_map.insert((pt1 - pt2).normalize());
            }
            out.insert(pt1, pt_map);
        }
        out
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = input.trim().parse::<Map>().unwrap();
    let los_map = map.construct_los();
    los_map.values().map(|v| v.len()).max()
}

pub fn part_two(input: &str) -> Option<i32> {
    let map = input.trim().parse::<Map>().unwrap();
    let los_map = map.construct_los();
    let station = *los_map
        .iter()
        .max_by_key(|(_, v)| v.len())
        .map(|(s, _)| s)
        .unwrap();
    let mut ordered = OrderedMap::construct(station, map);
    let target = ordered.nth_stroid(200);
    Some(target.x * 100 + target.y)
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
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_one_three() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_one_four() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(210));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(802));
    }
}
