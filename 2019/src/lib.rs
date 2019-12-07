pub mod computer;
pub mod input;
pub mod point;

pub fn number_to_digits(mut number: u32) -> [u8; 6] {
    let mut digits = [0; 6];
    digits.iter_mut().rev().for_each(|digit| {
        *digit = (number % 10) as u8;
        number /= 10;
    });

    digits
}

pub fn quinary(upto: u32) -> Vec<[u8; 5]> {
    let limit = number_to_digits(upto);
    let mut result = Vec::new();

    let mut counter = [0; 5];
    while &counter != &limit[1..] {
        result.push(counter);
        counter[0] += 1;
        for i in 0..5 {
            if counter[i] == 5 {
                counter[i] = 0;
                counter[i + 1] += 1;
            } else {
                break;
            }
        }
    }
    result.push(counter);

    result
}
