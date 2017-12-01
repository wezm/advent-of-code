use std::io::Read;
use std::fs::File;

fn main() {
    let mut input = Vec::new();
    let mut file = File::open("input").expect("unable to open input file");
    file.read_to_end(&mut input).expect("error reading input");

    println!("{}", captcha(&input));
}

fn captcha(input: &[u8]) -> u64 {
    let digits = input.iter()
        .take_while(|&&chr| chr >= 0x30 && chr <= 0x39)
        .collect::<Vec<_>>();

    let mut sum = 0;
    for idx in 0..digits.len() {
        if digits[idx] == digits[(idx + 1) % digits.len()] {
            sum += *digits[idx] as u64 - 0x30;
        }
    }

    sum
}

// 1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit and the
// third digit (2) matches the fourth digit.
#[test]
fn test_example1() {
    assert_eq!(captcha(b"1122"), 3);
}

// 1111 produces 4 because each digit (all 1) matches the next.
#[test]
fn test_example2() {
    assert_eq!(captcha(b"1111"), 4);
}

// 1234 produces 0 because no digit matches the next.
#[test]
fn test_example3() {
    assert_eq!(captcha(b"1234"), 0);
}

// 91212129 produces 9 because the only digit that matches the next one is the last digit, 9.
#[test]
fn test_example4() {
    assert_eq!(captcha(b"91212129"), 9);
}
