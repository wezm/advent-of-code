use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    let input_path = env::args_os()
        .skip(1)
        .next()
        .ok_or("missing input file path")?;
    let file = BufReader::new(File::open(input_path)?);

    let mut state = Vec::new();
    let mut combinations = Vec::new();
    let mut part1 = 0;
    for line in file.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        let (pattern, info) = line.split_once(' ').ok_or("unable to split line")?;

        let info = info
            .split(',')
            .map(|n| n.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;

        // generate all combinations of wildcards
        combinations.clear();
        state.clear();
        generate(pattern.as_bytes(), &mut state, &mut combinations, &info);
        // dbg!(&combinations);
        part1 += combinations.len();
    }

    println!("Part 1: {part1}");

    Ok(())
}

fn matches(s: &str, pattern: &str) -> bool {
    s.chars()
        .zip(pattern.chars())
        .all(|(a, b)| a == b || b == '?')
}

fn generate(
    pattern: &[u8],
    state: &mut Vec<u8>,
    combinations: &mut Vec<String>,
    contiguous: &[u32],
) {
    if pattern.is_empty() {
        let candidate = String::from_utf8(state.clone()).unwrap();
        let candidate_info = count_contiguous(&candidate);
        if candidate_info == contiguous {
            combinations.push(candidate);
        }
        return;
    }

    match pattern[0] {
        b'.' | b'#' => {
            state.push(pattern[0]);
            generate(&pattern[1..], state, combinations, contiguous);
            state.pop().unwrap();
        }
        b'?' => {
            state.push(b'.');
            generate(&pattern[1..], state, combinations, contiguous);
            state.pop().unwrap();
            state.push(b'#');
            generate(&pattern[1..], state, combinations, contiguous);
            state.pop().unwrap();
        }
        _ => unreachable!("unexpected char"),
    }
}

fn count_contiguous(s: &str) -> Vec<u32> {
    let mut counts = Vec::new();
    let mut in_contiguous = false;
    let mut count: u32 = 0;
    for c in s.chars() {
        match c {
            '#' if in_contiguous => count += 1,
            '#' => {
                in_contiguous = true;
                count = 1;
            }
            '.' if in_contiguous => {
                counts.push(count);
                in_contiguous = false;
            }
            _ => {}
        }
    }
    if in_contiguous {
        counts.push(count);
    }
    counts
}
