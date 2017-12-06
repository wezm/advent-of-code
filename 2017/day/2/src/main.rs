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
}

fn checksum(spreadsheet: &Spreadsheet) -> i32 {
    // Ideally we'd only iterate over each row once, hopefully the compiler optimises it away
    spreadsheet.iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
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
