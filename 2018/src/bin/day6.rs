#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Coordinate(i32, i32);

fn main() {
    let input = fs::read_to_string("input/day6.txt").expect("input");

    let coordinates = input
        .lines()
        .map(parse_coord)
        .collect::<Option<Vec<_>>>()
        .expect("error parsing input");

    part1(&coordinates);
}

fn part1(coordinates: &[Coordinate]) {
    let max = furthest_coord(coordinates);
    let grid = vec![None, max.0 * max.1];

    // Now fill the grid with the coords...

}

fn furthest_coord(coordinates: &[Coordinate]) -> Coordinate {
    // There are no negative coords, so assume origin at 0,0
    let mut max = Coordinate(0, 0);
    for Coordinate(x, y) in coordinates {
        if *x > max.0 {
            max.0= *x;
        }

        if *y > max.1 {
            max.1 = *y;
        }
    }

    max
}

fn parse_coord(line: &str) -> Option<Coordinate> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"\A(\d+), (\d+)\z"#).unwrap();
    }
    let captures = RE.captures(line)?;

    Some(Coordinate(captures[1].parse().ok()?, captures[2].parse().ok()?))
}
