use std::{fs, io};

use advent_of_code::input;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/day2.txt")?;
    let program = input::read_separated_line(',', &input)?;

    // To do this, before running the program, replace position 1 with the value 12 and replace
    // position 2 with the value 2. What value is left at position 0 after the program halts?
    let mut part1 = program.clone();
    run_program(&mut part1, 12, 2);

    println!("Part 1: {}", part1[0]);

    // Part 2
    // Determine what pair of inputs produces the output 19690720."
    // Each of the two input values will be between 0 and 99, inclusive
    let (noun, verb) = part2(19690720, &program);
    println!(
        "Part 2: noun={} verb={} result={}",
        noun,
        verb,
        100 * noun + verb
    );

    Ok(())
}

fn run_program(program: &mut [i32], noun: i32, verb: i32) {
    program[1] = noun;
    program[2] = verb;
    let mut addr = 0;

    loop {
        match program[addr] {
            1 => {
                program[program[addr + 3] as usize] =
                    program[program[addr + 1] as usize] + program[program[addr + 2] as usize]
            }
            2 => {
                program[program[addr + 3] as usize] =
                    program[program[addr + 1] as usize] * program[program[addr + 2] as usize]
            }
            99 => break,
            opcode => panic!("Invalid opcode {}", opcode),
        }

        addr += 4;
    }
}

fn part2(target: i32, program: &[i32]) -> (i32, i32) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut candidate = program.to_vec();
            run_program(&mut candidate, noun, verb);

            if candidate[0] == target {
                return (noun, verb);
            }
        }
    }

    panic!("Did not find solution");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = "1,0,0,0,99";
        let mut program = input::read_separated_line(',', &input).unwrap();

        run_program(&mut program, 0, 0);

        assert_eq!(program, &[2, 0, 0, 0, 99])
    }
    #[test]
    fn test_example4() {
        let input = "1,1,1,4,99,5,6,0,99";
        let mut program = input::read_separated_line(',', &input).unwrap();

        run_program(&mut program, 1, 1);

        assert_eq!(program, &[30, 1, 1, 4, 2, 5, 6, 0, 99])
    }
}
