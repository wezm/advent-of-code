use std::{env, fs};

type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    let input_path = env::args_os()
        .skip(1)
        .next()
        .ok_or("missing input file path")?;
    let input = fs::read_to_string(input_path)?;
    let mut lines = input.lines();
    let times = lines.next().ok_or("missing times")?;
    let distances = lines.next().ok_or("missing distances")?;

    let times = parse("Time:", times)?;
    let distances = parse("Distance:", distances)?;
    if times.len() != distances.len() {
        return Err("time/distance length mismatch".into());
    }

    let mut results = vec![0; times.len()];
    for (i, (time, record_distance)) in times
        .iter()
        .copied()
        .zip(distances.iter().copied())
        .enumerate()
    {
        (1..time - 1).for_each(|hold| {
            let velocity = hold;
            let distance = (time - hold) * velocity;
            if distance > record_distance {
                results[i] += 1;
            }
        })
    }

    println!("Part 1: {}", results.iter().fold(1, |a, b| a * b));

    Ok(())
}

fn parse(prefix: &str, line: &str) -> Result<Vec<usize>, BoxError> {
    if !line.starts_with(prefix) {
        return Err(format!("line did not match expected prefix ({}): {}", prefix, line).into());
    }

    let numbers = line
        .split_ascii_whitespace()
        .skip(1)
        .filter(|word| !word.is_empty())
        .map(|word| word.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(numbers)
}
