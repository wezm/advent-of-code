use std::fs;

struct Polymer {
    units: Vec<char>,
}

fn reacts(chr1: char, chr2: char) -> bool {
    // The is_ascii_lowercase probably isn't strictly necessary but ¯\_(ツ)_/¯
    (chr1.is_ascii_uppercase() && chr2 == chr1.to_ascii_lowercase())
        || (chr1.is_ascii_lowercase() && chr2 == chr1.to_ascii_uppercase())
}

impl Polymer {
    fn new() -> Self {
        Polymer { units: Vec::new() }
    }

    fn last(&self) -> Option<char> {
        self.units.last().map(|c| *c)
    }

    fn push(&mut self, chr: char) {
        // if the char being pushed is the same as the top of stack then pop it, otherwise push
        if self.last().map(|top| reacts(top, chr)).unwrap_or(false) {
            self.units.pop();
        } else {
            self.units.push(chr);
        }
    }

    fn into_string(self) -> String {
        self.units.iter().collect()
    }

    fn len(&self) -> usize {
        self.units.len()
    }
}

fn main() {
    let input = fs::read_to_string("input/day5.txt").expect("input");

    part1(input.trim());
    part2(input.trim());
}

fn part1(input: &str) {
    let result = reduce(input);

    println!("{}", result.len());
}

fn part2(input: &str) {
    let shortest_length = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|letter| {
            // Remove letter/LETTER and LETTER/letter
            let input = input
                .replace(letter, "")
                .replace(letter.to_ascii_uppercase(), "");
            reduce(&input)
        })
        .min_by(|polymer_a, polymer_b| polymer_a.len().cmp(&polymer_b.len()));

    println!("Part 2: {:?}", shortest_length.map(|polymer| polymer.len()));
}

fn reduce(input: &str) -> Polymer {
    input.chars().fold(Polymer::new(), |mut polymer, chr| {
        polymer.push(chr);
        polymer
    })
}

#[test]
fn test_reacts() {
    assert!(reacts('a', 'A'));
    assert!(reacts('A', 'a'));
    assert!(!reacts('A', 'A'));
    assert!(!reacts('a', 'a'));
    assert!(!reacts('a', 'B'));
}

#[test]
fn test_part1() {
    let input = "dabAcCaCBAcCcaDA";
    let result = reduce(input);

    assert_eq!(result.len(), 10);
    assert_eq!(result.into_string(), "dabCBAcaDA".to_string());
}

#[test]
fn test_part1_snippet() {
    let input = "wUuXxrRbeEaAuUMmJUuXxvoFfMmOTjJmMtVvmMVWpdDPwGgAalVv";
    let result = reduce(input);

    assert_eq!(result.into_string(), "wbJl".to_string());
}
