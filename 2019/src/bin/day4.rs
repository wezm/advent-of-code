use std::ops::RangeInclusive;

const INPUT: RangeInclusive<u32> = 356261..=846303;

// However, they do remember a few key facts about the password:

// * It is a six-digit number.
// * The value is within the range given in your puzzle input.
// * Two adjacent digits are the same (like 22 in 122345).
// * Going from left to right, the digits never decrease; they only ever increase or stay the same
//   (like 111123 or 135679).

fn main() {
    let count = INPUT.filter(meets_criteria).count();
    println!("Part 1: {}", count);
}

fn meets_criteria(number: &u32) -> bool {
    let mut number = *number;
    let mut digits = [0; 6];
    digits.iter_mut().rev().for_each(|digit| {
        *digit = number % 10;
        number /= 10;
    });

    // adjacent digits and increasing digits
    digits.windows(2).any(|pair| pair[0] == pair[1])
        && digits.windows(2).all(|pair| pair[1] >= pair[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meets_criteria() {
        assert!(meets_criteria(&111123));
        assert!(meets_criteria(&135669));
        assert!(!meets_criteria(&223450));
        assert!(!meets_criteria(&123789));
    }
}
