use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn main(input: &Path) {
    let file = BufReader::new(File::open(input).expect("unable to open input"));

    let mut group1 = Vec::new();
    let mut group2 = Vec::new();

    for line in file.lines() {
        let line = line.expect("error reading input line");
        let mut split = line.split_ascii_whitespace();
        let mut next = || split.next().unwrap().parse::<i32>().unwrap();
        group1.push(next());
        group2.push(next());
    }

    group1.sort();
    group2.sort();

    let part1: i32 = group1
        .iter()
        .copied()
        .zip(group2.iter().copied())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Part 1: {part1}");
}
