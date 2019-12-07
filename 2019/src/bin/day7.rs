use std::{fs, io};

use advent_of_code::{computer, input};

const AMPLIFIERS: usize = 5;

fn main() -> io::Result<()> {
    let source = fs::read_to_string("input/day7.txt")?;
    let data = input::read_separated_line(',', &source)?;

    let max = phase_settings()
        .iter()
        .map(|settings| {
            let mut output = 0;
            for phase_setting in settings.iter() {
                let input = vec![output, *phase_setting];
                let mut program = data.clone();
                let mut computer = computer::Computer::new(&mut program, input);
                computer.run(None, None);
                output = computer.output()[0];
            }
            output
        })
        .max()
        .unwrap();
    println!("Part 1: {}", max);

    Ok(())
}

fn phase_settings() -> Vec<[i32; AMPLIFIERS]> {
    let mut permutations = Vec::new();
    let mut indexes = [0i32; AMPLIFIERS];
    let mut elements = [0i32; AMPLIFIERS];
    for i in 0i32..=4 {
        elements[i as usize] = i;
    }

    permutations.push(elements);

    let mut i = 0;
    while i < AMPLIFIERS {
        if indexes[i] < i as i32 {
            elements.swap(if i % 2 == 0 { 0 } else { indexes[i] as usize }, i);
            permutations.push(elements);
            indexes[i] += 1;
            i = 0;
        } else {
            indexes[i] = 0;
            i += 1;
        }
    }

    permutations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_setting_generation() {
        let settings = phase_settings();
        assert_eq!(
            &settings[0..6],
            &[
                [0, 1, 2, 3, 4],
                [1, 0, 2, 3, 4],
                [2, 0, 1, 3, 4],
                [0, 2, 1, 3, 4],
                [1, 2, 0, 3, 4],
                [2, 1, 0, 3, 4]
            ]
        );
    }
}
