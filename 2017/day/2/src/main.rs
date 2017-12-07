use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

type Spreadsheet = Vec<Vec<i32>>;

fn main() {
    let file = File::open("input").expect("unable to open input file");
    let reader = BufReader::new(file);

    let spreadsheet = reader.lines()
        .map(|line|
            line.unwrap()
                .as_str()
                .split_whitespace()
                .map(|digit| i32::from_str(digit).unwrap())
                .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    println!("{}", checksum(&spreadsheet));
    println!("{}", evenly_divisible(&spreadsheet));
}

fn checksum(spreadsheet: &Spreadsheet) -> i32 {
    // Ideally we'd only iterate over each row once, hopefully the compiler optimises it away
    spreadsheet.iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum()
}

fn evenly_divisible_row(row: &[i32]) -> Option<i32> {
    for i in 0..row.len() {
        for j in 0..row.len() {
            if i == j { continue };

            if row[i] % row[j] == 0 {
                return Some(row[i] / row[j])
            }
        }
    }

    None
}

fn evenly_divisible(spreadsheet: &Spreadsheet) -> i32 {
    spreadsheet.iter()
        .flat_map(|row| evenly_divisible_row(row))
        .sum()
}

#[test]
fn test_checksum() {
    let input = vec![
        vec![5, 1, 9, 5],
        vec![7, 5, 3],
        vec![2, 4, 6, 8],
    ];

    // In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.
    assert_eq!(checksum(&input), 18);
}

#[test]
fn test_evenly_divisible() {
    let input = vec![
        vec![5, 9, 2, 8],
        vec![9, 4, 7, 3],
        vec![3, 8, 6, 5],
    ];

    // In this example, the sum of the results would be 4 + 3 + 2 = 9.
    assert_eq!(evenly_divisible(&input), 9);
}
