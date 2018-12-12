#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::{HashMap, HashSet};
use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    fn distance_to(&self, other: &Coordinate) -> u32 {
        ((other.x as i32 - self.x as i32).abs() + (other.y as i32 - self.y as i32).abs()) as u32
    }
}

#[derive(Debug, Clone)]
enum Claim {
    Unclaimed,
    Claimed { index: usize, distance: u32 },
    Tied { distance: u32 },
}

struct Grid {
    grid: Vec<Claim>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![Claim::Unclaimed; width * height],
            width,
            height,
        }
    }

    fn claim(&mut self, index: usize, coord: &Coordinate) {
        // loop through the grid, calculate the distance to each point, if it's unclaimed or the
        // distance is shorter then claim it.
        for (current_index, claim) in self.grid.iter_mut().enumerate() {
            let current_coord = index_to_coord(current_index, self.width);
            let distance = coord.distance_to(&current_coord);

            match claim {
                Claim::Unclaimed => *claim = Claim::Claimed { index, distance },
                Claim::Claimed {
                    distance: claimed_distance,
                    ..
                } if *claimed_distance > distance => *claim = Claim::Claimed { index, distance },
                Claim::Claimed {
                    distance: claimed_distance,
                    ..
                } if *claimed_distance == distance => *claim = Claim::Tied { distance },
                Claim::Tied {
                    distance: tied_distance,
                } if distance < *tied_distance => *claim = Claim::Claimed { index, distance },
                _ => {}
            }
        }
    }

    fn largest_claimed_area(&self) -> u32 {
        let areas = self.grid.iter().fold(HashMap::new(), |mut areas, claim| {
            if let Claim::Claimed { index, .. } = claim {
                let area = areas.entry(index).or_insert(0u32);
                *area += 1
            }

            areas
        });

        // Exclude indexes that touch the edges as they will be infinite
        let mut exclude = HashSet::new();
        for x in 0..self.width {
            if let Claim::Claimed { index, .. } = self.grid[x] {
                exclude.insert(index);
            }
            if let Claim::Claimed { index, .. } = self.grid[self.width * (self.height - 1) + x] {
                exclude.insert(index);
            }
        }
        for y in 1..self.height - 1 {
            if let Claim::Claimed { index, .. } = self.grid[y * self.width] {
                exclude.insert(index);
            }
            if let Claim::Claimed { index, .. } = self.grid[y * self.width + self.width - 1] {
                exclude.insert(index);
            }
        }

        *areas
            .iter()
            .filter_map(|(index, area)| {
                if exclude.contains(index) {
                    None
                } else {
                    Some(area)
                }
            })
            .max()
            .unwrap()
    }
}

fn main() {
    let input = fs::read_to_string("input/day6.txt").expect("input");

    let coordinates = input
        .lines()
        .map(parse_coord)
        .collect::<Option<Vec<_>>>()
        .expect("error parsing input");

    part1(&coordinates);
    part2(&coordinates);
}

fn part1(coordinates: &[Coordinate]) {
    let max = furthest_coord(coordinates);
    let mut grid = Grid::new(max.x as usize, max.y as usize);

    // Claim closest locations for each coord
    for (index, coord) in coordinates.iter().enumerate() {
        grid.claim(index, &coord);
    }

    // Find the one with the most claimed area
    let area = grid.largest_claimed_area();
    println!("Part 1 = {}", area);
}

fn part2(coordinates: &[Coordinate]) {
    // For each location calculate the distance to each coordinate. If that's less than 10,000 the
    // it's part of "the region", count it.
    let max = furthest_coord(coordinates);
    let mut count_in_region = 0;

    for y in 0..max.y {
        for x in 0..max.x {
            let current_location = Coordinate { x, y };
            // Sum distance to all the coordinates
            let sum = coordinates
                .iter()
                .map(|coord| current_location.distance_to(coord))
                .sum::<u32>();

            if sum < 10000 {
                count_in_region += 1;
            }
        }
    }

    println!("Part 2 = {}", count_in_region);
}

fn furthest_coord(coordinates: &[Coordinate]) -> Coordinate {
    // There are no negative coords, so assume origin at 0,0
    let mut max = Coordinate { x: 0, y: 0 };
    for Coordinate { x, y } in coordinates {
        if *x > max.x {
            max.x = *x;
        }

        if *y > max.y {
            max.y = *y;
        }
    }

    max
}

fn parse_coord(line: &str) -> Option<Coordinate> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"\A(\d+), (\d+)\z"#).unwrap();
    }
    let captures = RE.captures(line)?;

    Some(Coordinate {
        x: captures[1].parse().ok()?,
        y: captures[2].parse().ok()?,
    })
}

fn index_to_coord(index: usize, width: usize) -> Coordinate {
    let x = (index % width) as u32;
    let y = (index / width) as u32;
    Coordinate { x, y }
}
