use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let file = File::open("input").expect("unable to open input file");
    let reader = BufReader::new(file);

    let valid = reader.lines()
        .map(|line| passphrase_is_valid(&line.unwrap()))
        .filter(|valid| *valid == true)
        .count();

    println!("Part 1: {}", valid);

    let file = File::open("input").expect("unable to open input file");
    let reader = BufReader::new(file);

    let valid = reader.lines()
        .map(|line| passphrase_is_valid_part2(&line.unwrap()))
        .filter(|valid| *valid == true)
        .count();

    println!("Part 2: {}", valid);
}

fn passphrase_is_valid(passphrase: &str) -> bool {
    let mut seen = HashSet::new();
    !passphrase.split_whitespace().any(|word| seen.insert(word) == false)
}

fn passphrase_is_valid_part2(passphrase: &str) -> bool {
    let mut seen = HashSet::new();
    !passphrase.split_whitespace().any(|word| {
        let mut sorted_chars = word.chars().collect::<Vec<_>>();
        sorted_chars.sort();
        seen.insert(sorted_chars) == false
    })
}

// aa bb cc dd ee is valid.
#[test]
fn test_example1() {
    assert_eq!(passphrase_is_valid("aa bb cc dd ee"), true)
}

// aa bb cc dd aa is not valid - the word aa appears more than once.
#[test]
fn test_example2() {
    assert_eq!(passphrase_is_valid("aa bb cc dd aa"), false)
}

// aa bb cc dd aaa is valid - aa and aaa count as different words.
#[test]
fn test_example3() {
    assert_eq!(passphrase_is_valid("aa bb cc dd aaa"), true)
}

// abcde fghij is a valid passphrase.
#[test]
fn test_example1_part2() {
    assert_eq!(passphrase_is_valid_part2("abcde fghij "), true)
}

// abcde xyz ecdab is not valid - the letters from the third word can be rearranged to form the first word.
#[test]
fn test_example2_part2() {
    assert_eq!(passphrase_is_valid_part2("abcde xyz ecdab"), false)
}

// a ab abc abd abf abj is a valid passphrase, because all letters need to be used when forming another word.
#[test]
fn test_example3_part2() {
    assert_eq!(passphrase_is_valid_part2("a ab abc abd abf abj"), true)
}

// iiii oiii ooii oooi oooo is valid.
#[test]
fn test_example4_part2() {
    assert_eq!(passphrase_is_valid_part2("iiii oiii ooii oooi oooo"), true)
}

// oiii ioii iioi iiio is not valid - any of these words can be rearranged to form any other word.
#[test]
fn test_example5_part2() {
    assert_eq!(passphrase_is_valid_part2("oiii ioii iioi iiio"), false)
}
