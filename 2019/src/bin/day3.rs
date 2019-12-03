use std::{fs, io};

use advent_of_code::point::Point;
use std::collections::HashSet;

trait Vector {
    fn go(&mut self, vector: Direction) -> Self;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/day3.txt")?;
    let paths: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.trim().split(',').collect())
        .map(|path| path_coordinates(&path))
        .collect();

    // Find points in common
    let wire1 = paths[0].iter().copied().collect::<HashSet<_>>();
    let wire2 = paths[1].iter().copied().collect::<HashSet<_>>();

    let result = wire1
        .intersection(&wire2)
        .map(Point::manhattan_distance)
        .min()
        .unwrap();
    println!("Part 1: {}", result);

    // Part 2, count steps (length of path)
    // For each common point find it's position in each wire paths, that is the steps
    let result = wire1
        .intersection(&wire2)
        .map(|point| {
            paths[0].iter().position(|other| point == other).unwrap()
                + paths[1].iter().position(|other| other == point).unwrap()
                + 2 // to account for 0 based index
        })
        .min()
        .unwrap();
    println!("Part 2: {}", result);

    Ok(())
}

fn path_coordinates(path: &Vec<&str>) -> Vec<Point> {
    let mut pos = Point(0, 0); // origin is  bottom left

    path.iter()
        .map(|&vector| {
            let dir = Direction::from(vector);
            (0..dir.magnitude())
                .map(|_| pos.go(dir))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

impl Vector for Point {
    fn go(&mut self, direction: Direction) -> Self {
        match direction {
            Direction::Up(_) => self.1 += 1,
            Direction::Down(_) => self.1 -= 1,
            Direction::Left(_) => self.0 -= 1,
            Direction::Right(_) => self.0 += 1,
        }

        *self
    }
}

impl Direction {
    pub fn magnitude(&self) -> i32 {
        match *self {
            Direction::Up(val) => val,
            Direction::Down(val) => val,
            Direction::Left(val) => val,
            Direction::Right(val) => val,
        }
    }
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        let dir = s.chars().next().unwrap();
        let magnitude = s[1..].parse().unwrap();
        match dir {
            'U' => Direction::Up(magnitude),
            'D' => Direction::Down(magnitude),
            'L' => Direction::Left(magnitude),
            'R' => Direction::Right(magnitude),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_from_str() {
        assert_eq!(Direction::from("U60"), Direction::Up(60));
    }
}
