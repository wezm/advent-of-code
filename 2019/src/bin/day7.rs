use std::{fs, io};

use advent_of_code::computer::{ComputeResult, Computer, Output, Pipe};
use advent_of_code::input;
use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

const AMPLIFIERS: usize = 5;

fn main() -> io::Result<()> {
    let source = fs::read_to_string("input/day7.txt")?;
    let data = input::read_separated_line(',', &source)?;

    part1(data.clone());
    part2(data);

    Ok(())
}

fn part1(data: Vec<i64>) {
    let mut elements = [0i64; AMPLIFIERS];
    for i in 0i64..=4 {
        elements[i as usize] = i;
    }
    let max = phase_settings(elements)
        .into_iter()
        .map(|settings| {
            let mut output = 0;
            let mut name = 'A';
            for phase_setting in settings.iter() {
                let input = vec![output, *phase_setting];
                let mut computer = Computer::new(name, data.clone(), input, vec![]);
                name = char::try_from(name as u32 + 1).unwrap();
                computer.run(None, None);
                output = computer.output();
            }
            output
        })
        .max()
        .unwrap();
    println!("Part 1: {}", max);
}

fn part2(data: Vec<i64>) {
    let mut elements = [0i64; AMPLIFIERS];
    for i in 5i64..=9 {
        elements[i as usize - 5] = i;
    }

    let max = phase_settings(elements)
        .into_iter()
        .map(|settings| run_part2_with_settings(data.clone(), settings))
        .max()
        .unwrap();

    println!("Part 2: {}", max);
}

fn run_part2_with_settings(data: Vec<i64>, settings: [i64; AMPLIFIERS]) -> i64 {
    // Construct the amplifiers and the pipes between them
    // The first amplifier gets io_pipe as input
    // The last amplifier gets io_pipe as output
    // otherwise the get a pipe between them
    let io_pipe = Rc::new(RefCell::new(Pipe::new(vec![0].into_iter().collect())));
    let mut last_pipe = Rc::clone(&io_pipe);
    let mut amplifiers = Vec::with_capacity(AMPLIFIERS);
    for i in 0..AMPLIFIERS {
        let next_pipe = if i == AMPLIFIERS - 1 {
            // This is the last amplifier, its output goes to the input of the first
            Rc::clone(&io_pipe)
        } else {
            Rc::new(RefCell::new(Pipe::new(vec![].into_iter().collect())))
        };
        // Add the phase setting (it's the first thing that's read)
        last_pipe.borrow_mut().push_front(settings[i]);
        let amp = Computer::new(
            char::try_from('A' as u32 + i as u32).unwrap(),
            data.clone(),
            last_pipe,
            Rc::clone(&next_pipe),
        );
        amplifiers.push(amp);
        last_pipe = next_pipe;
    }

    // Now run them
    let mut i = 0;
    let mut halted = Vec::with_capacity(AMPLIFIERS);
    loop {
        match amplifiers[i].run(None, None) {
            ComputeResult::Halted => {
                let amp = amplifiers.remove(i);
                println!("{} halted", amp.name());
                halted.push(amp);
                if amplifiers.is_empty() {
                    break;
                }
                // Don't increment i as we removed an element
                i = i % amplifiers.len();
            }
            ComputeResult::NeedsInput => {
                println!("{} needs input", amplifiers[i].name());
                // continue to next one
                i = (i + 1) % amplifiers.len();
            }
        }
    }

    // Need the last output sent from the last amplifier
    dbg!(io_pipe.borrow());
    io_pipe.last_value()
}

fn phase_settings(mut elements: [i64; AMPLIFIERS]) -> Vec<[i64; AMPLIFIERS]> {
    let mut permutations = Vec::new();
    let mut indexes = [0i64; AMPLIFIERS];

    permutations.push(elements);

    let mut i = 0;
    while i < AMPLIFIERS {
        if indexes[i] < i as i64 {
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
        let settings = phase_settings([0, 1, 2, 3, 4]);
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

    #[test]
    fn test_part2_example1() {
        let input = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(run_part2_with_settings(input, [9, 8, 7, 6, 5]), 139629729);
    }

    #[test]
    fn test_part2_example2() {
        let input = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(run_part2_with_settings(input, [9, 7, 8, 5, 6]), 18216);
    }

    #[test]
    fn test_part2_with_part1_example() {
        let input = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(run_part2_with_settings(input, [4, 3, 2, 1, 0]), 43210);
    }
}
