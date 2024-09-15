use std::{collections::HashSet, str::FromStr};

use advent_of_code::util::{euclid::gcd, point::Pt3};

advent_of_code::solution!(12);

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct System {
    moons: [Moon; 4],
}

impl System {
    fn gravities(&self) -> Vec<Pt3<i32>> {
        self.moons
            .iter()
            .map(|target| {
                self.moons
                    .iter()
                    .map(|m| Pt3 {
                        x: (m.position.x - target.position.x).signum(),
                        y: (m.position.y - target.position.y).signum(),
                        z: (m.position.z - target.position.z).signum(),
                    })
                    .sum()
            })
            .collect()
    }

    fn update(&mut self) {
        let gravities = self.gravities();
        for (moon, g) in self.moons.iter_mut().zip(&gravities) {
            moon.velocity += *g;
            moon.position += moon.velocity;
        }
    }

    fn x_axis(&self) -> [i32; 8] {
        [
            self.moons[0].position.x,
            self.moons[0].velocity.x,
            self.moons[1].position.x,
            self.moons[1].velocity.x,
            self.moons[2].position.x,
            self.moons[2].velocity.x,
            self.moons[3].position.x,
            self.moons[3].velocity.x,
        ]
    }

    fn y_axis(&self) -> [i32; 8] {
        [
            self.moons[0].position.y,
            self.moons[0].velocity.y,
            self.moons[1].position.y,
            self.moons[1].velocity.y,
            self.moons[2].position.y,
            self.moons[2].velocity.y,
            self.moons[3].position.y,
            self.moons[3].velocity.y,
        ]
    }

    fn z_axis(&self) -> [i32; 8] {
        [
            self.moons[0].position.z,
            self.moons[0].velocity.z,
            self.moons[1].position.z,
            self.moons[1].velocity.z,
            self.moons[2].position.z,
            self.moons[2].velocity.z,
            self.moons[3].position.z,
            self.moons[3].velocity.z,
        ]
    }

    fn energy(&self) -> u32 {
        self.moons
            .iter()
            .map(|m| {
                (m.position.x.unsigned_abs()
                    + m.position.y.unsigned_abs()
                    + m.position.z.unsigned_abs())
                    * (m.velocity.x.unsigned_abs()
                        + m.velocity.y.unsigned_abs()
                        + m.velocity.z.unsigned_abs())
            })
            .sum()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Moon {
    position: Pt3<i32>,
    velocity: Pt3<i32>,
}

impl Moon {
    fn with_position(position: Pt3<i32>) -> Self {
        Self {
            position,
            velocity: Pt3::default(),
        }
    }
}

impl FromStr for System {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut moons = [Moon::with_position(Pt3 { x: 0, y: 0, z: 0 }); 4];
        for (line, moon) in s.lines().zip(&mut moons) {
            let mut pts = line
                .trim_start_matches('<')
                .trim_end_matches('>')
                .split(", ");
            let x = pts.next().unwrap().trim_start_matches("x=").parse()?;
            let y = pts.next().unwrap().trim_start_matches("y=").parse()?;
            let z = pts.next().unwrap().trim_start_matches("z=").parse()?;
            *moon = Moon::with_position((x, y, z).into());
        }
        Ok(Self { moons })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut system = input.parse::<System>().unwrap();
    for _ in 0..1000 {
        system.update();
    }
    Some(system.energy())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut system = input.parse::<System>().unwrap();
    let mut xs = HashSet::with_capacity(200000);
    let mut ys = HashSet::with_capacity(200000);
    let mut zs = HashSet::with_capacity(200000);
    let mut x_axis_repeat = None;
    let mut y_axis_repeat = None;
    let mut z_axis_repeat = None;
    for i in 0u64.. {
        if x_axis_repeat.is_none() && !xs.insert(system.x_axis()) {
            x_axis_repeat = Some(i);
        }
        if y_axis_repeat.is_none() && !ys.insert(system.y_axis()) {
            y_axis_repeat = Some(i);
        }
        if z_axis_repeat.is_none() && !zs.insert(system.z_axis()) {
            z_axis_repeat = Some(i);
        }
        if let (Some(x), Some(y), Some(z)) = (x_axis_repeat, y_axis_repeat, z_axis_repeat) {
            let gcd_xy = gcd(x, y);
            let lcm_xy = (x * y) / gcd_xy;
            let gcd_xyz = gcd(lcm_xy, z);
            return Some((z * lcm_xy) / gcd_xyz);
        }
        system.update();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14645));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4686774924));
    }
}
