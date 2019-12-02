use std::{fs, io};

use advent_of_code::input;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/day2.txt")?;
    let mut program = input::read_separated_line(',', &input)?;

    // To do this, before running the program, replace position 1 with the value 12 and replace
    // position 2 with the value 2. What value is left at position 0 after the program halts?
    program[1] = 12;
    program[2] = 2;
    run_program(&mut program);

    println!("Part 1: {}", program[0]);

    Ok(())
}

fn run_program(program: &mut [i32]) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = "1,0,0,0,99";
        let mut program = input::read_separated_line(',', &input).unwrap();

        run_program(&mut program);

        assert_eq!(program, &[2, 0, 0, 0, 99])
    }
    #[test]
    fn test_example4() {
        let input = "1,1,1,4,99,5,6,0,99";
        let mut program = input::read_separated_line(',', &input).unwrap();

        run_program(&mut program);

        assert_eq!(program, &[30, 1, 1, 4, 2, 5, 6, 0, 99])
    }
}
