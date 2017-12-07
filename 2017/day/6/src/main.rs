use std::fs::File;
use std::io::{Read};
use std::str::FromStr;
use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();
    let mut file = File::open("input").expect("unable to open input file");
    file.read_to_string(&mut buffer).expect("error reading input");

    let mut banks = buffer.as_str()
        .split_whitespace()
        .map(|digit| i32::from_str(&digit).unwrap())
        .collect::<Vec<_>>();

    println!("{}", redistribution_cycles(&mut banks));
}

fn redistribute(banks: &mut Vec<i32>) {
    let max = *banks.iter().max().unwrap();
    let bank_to_redistribute = banks.iter().position(|blocks| *blocks == max).unwrap();
    let mut blocks = banks[bank_to_redistribute];
    let mut current_bank = bank_to_redistribute;
    banks[bank_to_redistribute] = 0;

    while blocks > 0 {
        current_bank = (current_bank + 1) % banks.len();
        banks[current_bank] += 1;
        blocks -= 1;
    }
}

fn redistribution_cycles(banks: &mut Vec<i32>) -> usize {
    let mut seen_states = HashSet::new();
    seen_states.insert(banks.clone());

    loop {
        redistribute(banks);
        if seen_states.insert(banks.clone()) == false {
            break;
        }
    }

    seen_states.len()
}

#[test]
fn test_example() {
    let mut banks = vec![0, 2, 7, 0];
    assert_eq!(redistribution_cycles(&mut banks), 5);
}
