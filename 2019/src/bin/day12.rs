use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;
use std::{fs, io, ops};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Moon {
    position: Point3D,
    velocity: Point3D,
}

fn main() -> io::Result<()> {
    let mut moons = parse_positions(&fs::read_to_string("input/day12.txt")?)?;

    for _i in 0..1000 {
        step(&mut moons);
    }
    let system: Vec<_> = moons.into_iter().map(|moon| moon.unwrap()).collect();
    println!("Part 1: {}", total_energy(&system));

    let moons = parse_positions(&fs::read_to_string("input/day12.txt")?)?;
    let cycles = cycles(moons);

    dbg!(cycles);

    Ok(())
}

fn cycles(mut moons: Vec<Option<Moon>>) -> Vec<usize> {
    let mut cycles = Vec::with_capacity(moons.len());
    for i in 0..moons.len() {
        let mut seen = HashSet::new();
        let mut count = 0;
        loop {
            if count % 1000000 == 0 {
                println!("{}", count);
            }
            if seen.contains(&moons[i]) {
                dbg!(&moons[1]);
                break;
            }
            seen.insert(moons[i].clone());
            step(&mut moons);
            count += 1;
        }
        cycles.push(count);
    }
    cycles
}

fn parse_positions(s: &str) -> io::Result<Vec<Option<Moon>>> {
    let positions: Vec<Point3D> = s
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "parse error"))?;

    Ok(positions
        .into_iter()
        .map(|position| {
            Some(Moon {
                position,
                velocity: Point3D { x: 0, y: 0, z: 0 },
            })
        })
        .collect())
}

fn step(system: &mut Vec<Option<Moon>>) {
    // Within each time step, first update the velocity of every moon by applying gravity.
    // Then, once all moons' velocities have been updated, update the position of every moon
    // by applying velocity.

    // To apply gravity, consider every pair of moons. On each axis (x, y, and z), the velocity of
    // each moon changes by exactly +1 or -1 to pull the moons together. For example, if Ganymede
    // has an x position of 3, and Callisto has a x position of 5, then Ganymede's x velocity
    // changes by +1 (because 5 > 3) and Callisto's x velocity changes by -1 (because 3 < 5).
    // However, if the positions on a given axis are the same, the velocity on that axis does not
    // change for that pair of moons.

    // Hackery....
    for pair in (0..system.len()).combinations(2) {
        let mut moon1 = system[pair[0]].take().unwrap();
        let mut moon2 = system[pair[1]].take().unwrap();
        moon1.apply_gravity(&mut moon2);
        system[pair[0]].replace(moon1);
        system[pair[1]].replace(moon2);
    }

    // Once all gravity has been applied, apply velocity: simply add the velocity of each moon to
    // its own position.
    for opt_moon in system {
        if let Some(moon) = opt_moon {
            moon.apply_velocity()
        }
    }
}

fn total_energy(system: &[Moon]) -> i32 {
    system.iter().map(|moon| moon.total_energy()).sum()
}

impl Moon {
    fn apply_gravity(&mut self, other: &mut Moon) {
        if self.position.x == other.position.x {
        } else if self.position.x < other.position.x {
            self.velocity.x += 1;
            other.velocity.x -= 1;
        } else {
            self.velocity.x -= 1;
            other.velocity.x += 1;
        }

        if self.position.y == other.position.y {
        } else if self.position.y < other.position.y {
            self.velocity.y += 1;
            other.velocity.y -= 1;
        } else {
            self.velocity.y -= 1;
            other.velocity.y += 1;
        }

        if self.position.z == other.position.z {
        } else if self.position.z < other.position.z {
            self.velocity.z += 1;
            other.velocity.z -= 1;
        } else {
            self.velocity.z -= 1;
            other.velocity.z += 1;
        }
    }

    fn apply_velocity(&mut self) {
        self.position += self.velocity;
    }

    // The total energy for a single moon is its potential energy multiplied by its kinetic energy.
    // A moon's potential energy is the sum of the absolute values of its x, y, and z position
    // coordinates. A moon's kinetic energy is the sum of the absolute values of its velocity
    // coordinates.
    fn potential_energy(&self) -> i32 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }

    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

impl FromStr for Point3D {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^<x=([\d-]+), y=([\d-]+), z=([\d-]+)>$").unwrap();
        let caps = re.captures(s.trim()).unwrap();

        Ok(Point3D {
            x: caps[1].parse()?,
            y: caps[2].parse()?,
            z: caps[3].parse()?,
        })
    }
}

impl ops::AddAssign for Point3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
";
        let mut moons = parse_positions(input).unwrap();
        step(&mut moons);
        dbg!(moons);
    }

    #[test]
    fn test_part2() {
        let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
";
        let moons = parse_positions(input).unwrap();
        dbg!(cycles(moons));
    }
}
