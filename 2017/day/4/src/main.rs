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

    println!("{}", valid);
}

fn passphrase_is_valid(passphrase: &str) -> bool {
    let mut seen = HashSet::new();
    !passphrase.split_whitespace().any(|word| seen.insert(word) == false)
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
