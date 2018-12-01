use std::io::{self, BufRead, BufReader};
use std::fs::{self, File};
use std::collections::HashSet;

const INPUT: &str = "input/2018/day1.txt";

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = File::open(INPUT).expect("input");
    let input = BufReader::new(input);

    let frequency = input.lines().map(|line| {
        line.unwrap().parse::<i32>().expect("unable to parse line")
    }).sum::<i32>();
    println!("{}", frequency);
}

fn part2() {
    let input = fs::read_to_string(INPUT).expect("input");

    let mut frequency = 0;
    let mut seen = HashSet::new();
    for line in input.lines().cycle() {
        let drift = line.parse::<i32>().expect("unable to parse line");
        frequency += drift;
        if !seen.insert(frequency) {
            break;
        }
    }

    println!("{}", frequency);
}
