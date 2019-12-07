use advent_of_code::number_to_digits;
use std::ops::RangeInclusive;

const INPUT: RangeInclusive<u32> = 356261..=846303;

// However, they do remember a few key facts about the password:

// * It is a six-digit number.
// * The value is within the range given in your puzzle input.
// * Two adjacent digits are the same (like 22 in 122345).
// * Going from left to right, the digits never decrease; they only ever increase or stay the same
//   (like 111123 or 135679).

fn main() {
    println!("Part 1: {}", INPUT.filter(meets_criteria_part1).count());
    println!("Part 2: {}", INPUT.filter(meets_criteria_part2).count());
}

fn meets_criteria_part1(number: &u32) -> bool {
    let digits = number_to_digits(*number);

    // adjacent digits and increasing digits
    digits.windows(2).any(|pair| pair[0] == pair[1])
        && digits.windows(2).all(|pair| pair[1] >= pair[0])
}

fn meets_criteria_part2(number: &u32) -> bool {
    let digits = number_to_digits(*number);

    // Group by digits
    let mut chunks: Vec<Vec<_>> = vec![];
    let mut prev = None;
    for &digit in &digits {
        if Some(digit) == prev {
            chunks.last_mut().unwrap().push(digit)
        } else {
            chunks.push(vec![digit])
        }

        prev = Some(digit)
    }

    // check if any double digits are present
    chunks.iter().any(|chunk| chunk.len() == 2) && digits.windows(2).all(|pair| pair[1] >= pair[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meets_criteria_part1() {
        assert!(meets_criteria_part1(&111123));
        assert!(meets_criteria_part1(&135669));
        assert!(!meets_criteria_part1(&223450));
        assert!(!meets_criteria_part1(&123789));
    }

    #[test]
    fn test_meets_criteria_part2() {
        assert!(meets_criteria_part2(&112233));
        assert!(!meets_criteria_part2(&123444));
        assert!(meets_criteria_part2(&111122));
    }
}
