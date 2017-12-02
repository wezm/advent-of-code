use std::io::Read;
use std::fs::File;

fn main() {
    let mut input = Vec::new();
    let mut file = File::open("input").expect("unable to open input file");
    file.read_to_end(&mut input).expect("error reading input");

    println!("{}", captcha(&input));
    println!("{}", captcha2(&input));
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

fn captcha2(input: &[u8]) -> u64 {
    let digits = input.iter()
        .take_while(|&&chr| chr >= 0x30 && chr <= 0x39)
        .collect::<Vec<_>>();

    let mut sum = 0;
    for idx in 0..digits.len() {
        if digits[idx] == digits[(idx + (digits.len() / 2)) % digits.len()] {
            sum += *digits[idx] as u64 - 0x30;
        }
    }

    sum
}

// 1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit and the
// third digit (2) matches the fourth digit.
#[test]
fn test_captcha_example1() {
    assert_eq!(captcha(b"1122"), 3);
}

// 1111 produces 4 because each digit (all 1) matches the next.
#[test]
fn test_captcha_example2() {
    assert_eq!(captcha(b"1111"), 4);
}

// 1234 produces 0 because no digit matches the next.
#[test]
fn test_captcha_example3() {
    assert_eq!(captcha(b"1234"), 0);
}

// 91212129 produces 9 because the only digit that matches the next one is the last digit, 9.
#[test]
fn test_captcha_example4() {
    assert_eq!(captcha(b"91212129"), 9);
}

// 1212 produces 6: the list contains 4 items, and all four digits match the digit 2 items ahead.
#[test]
fn test_captcha2_example1() {
    assert_eq!(captcha2(b"1212"), 6);
}

// 1221 produces 0, because every comparison is between a 1 and a 2.
#[test]
fn test_captcha2_example2() {
    assert_eq!(captcha2(b"1221"), 0);
}

// 123425 produces 4, because both 2s match each other, but no other digit has a match.
#[test]
fn test_captcha2_example3() {
    assert_eq!(captcha2(b"123425"), 4);
}

// 123123 produces 12.
#[test]
fn test_captcha2_example4() {
    assert_eq!(captcha2(b"123123"), 12);
}

// 12131415 produces 4.
#[test]
fn test_captcha2_example5() {
    assert_eq!(captcha2(b"12131415"), 4);
}

