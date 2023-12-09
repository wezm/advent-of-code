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
    let mut sum = 0;
    for line in file.lines() {
        let line = line?;
        let history = parse_line(&line)?;
        let value = calculate_next_value(&history);
        sum += value;
    }

    println!("Part 1: {sum}");

    Ok(())
}

fn parse_line(line: &str) -> Result<Vec<i32>, ParseIntError> {
    line.split_ascii_whitespace()
        .map(|string| string.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
}

fn calculate_next_value(history: &[i32]) -> i32 {
    let x: Vec<_> = history
        .windows(2)
        .map(|window| {
            let &[a, b] = window else {
                unreachable!("window is not a pair")
            };
            b - a
        })
        .collect();

    // See if they're all the same, if so then the next value is the same
    let first = x[0];
    let last = history.last().unwrap();
    last + if x.iter().all(|val| *val == first) {
        first
    } else {
        calculate_next_value(&x)
    }
}
