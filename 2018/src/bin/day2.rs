use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/2018/day2.txt").expect("input");
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut twos = 0;
    let mut threes = 0;

    for line in input.lines() {
        let char_counts = count_chars(line);
        if char_counts.iter().any(|(_k, &v)| v == 2) {
            twos += 1;
        }
        if char_counts.iter().any(|(_k, &v)| v == 3) {
            threes += 1;
        }
    }

    println!("{}", twos * threes);
}

fn count_chars(line: &str) -> HashMap<char, u32> {
    let mut counts = HashMap::new();
    line.chars()
        .for_each(|chr| *counts.entry(chr).or_insert(0) += 1);
    counts
}

fn part2(input: &str) {
    for (i, line) in input.lines().enumerate() {
        for (j, other) in input.lines().enumerate() {
            if i == j {
                continue;
            }

            let number_differing =
                line.chars().zip(other.chars()).fold(
                    0,
                    |acc, (chr1, chr2)| if chr1 != chr2 { acc + 1 } else { acc },
                );

            if (number_differing == 1) {
                println!("{} and {} differ by 1 char", line, other);
                let common: String = line
                    .chars()
                    .zip(other.chars())
                    .filter_map(|(chr1, chr2)| if chr1 != chr2 { None } else { Some(chr1) })
                    .collect();
                println!("Common: {}", common);
                return;
            }
        }
    }
}
