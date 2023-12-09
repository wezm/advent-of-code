use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    let input_path = env::args_os()
        .skip(1)
        .next()
        .ok_or("missing input file path")?;
    let file = BufReader::new(File::open(input_path)?);
    let mut part1 = 0;
    let mut part2 = 0;
    for line in file.lines() {
        let line = line?;
        let history = parse_line(&line)?;
        let value = calculate_next_value(&history);
        let prev_value = calculate_prev_value(&history);
        part1 += value;
        part2 += prev_value;
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

fn parse_line(line: &str) -> Result<Vec<i32>, ParseIntError> {
    line.split_ascii_whitespace()
        .map(|string| string.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
}

fn calculate_next_value(history: &[i32]) -> i32 {
    let x = window_differences(history);

    // See if they're all the same, if so then the next value is the same
    let first = x[0];
    let last = history.last().unwrap();
    last + if x.iter().all(|val| *val == first) {
        first
    } else {
        calculate_next_value(&x)
    }
}

fn calculate_prev_value(history: &[i32]) -> i32 {
    let x = window_differences(history);

    // See if they're all the same, if so then the next value is the same
    let same = x[0];
    let first = history.first().unwrap();
    first
        - if x.iter().all(|val| *val == same) {
            same
        } else {
            calculate_prev_value(&x)
        }
}

fn window_differences(values: &[i32]) -> Vec<i32> {
    values
        .windows(2)
        .map(|window| {
            let &[a, b] = window else {
                unreachable!("window is not a pair")
            };
            b - a
        })
        .collect()
}
