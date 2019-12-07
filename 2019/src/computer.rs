use std::io;
use std::io::Write;
use std::str::FromStr;

type Address = i32;

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Add(Mode, Mode),
    Multiply(Mode, Mode),
    Input,
    Output(Mode),
    Halt,
}

#[derive(Debug, Eq, PartialEq)]
enum Mode {
    Immediate,
    Address,
}

pub struct Memory<'a> {
    mem: &'a mut [i32],
}

fn decode(mut instruction: i32) -> Instruction {
    let opcode = divmod(&mut instruction, 100);

    match opcode {
        1 => {
            // let mode3 = divmod(&mut instruction, 10);
            // Parameters that an instruction writes to will never be in immediate mode.
            Instruction::Add(
                Mode::from(divmod(&mut instruction, 10)),
                Mode::from(divmod(&mut instruction, 10)),
            )
        }
        2 => Instruction::Multiply(
            Mode::from(divmod(&mut instruction, 10)),
            Mode::from(divmod(&mut instruction, 10)),
        ),
        3 => Instruction::Input,
        4 => Instruction::Output(Mode::from(divmod(&mut instruction, 10))),
        99 => Instruction::Halt,
        _ => panic!("Invalid opcode: {}", opcode),
    }
}

impl<'a> Memory<'a> {
    pub fn new(mem: &'a mut [i32]) -> Self {
        Memory { mem }
    }

    fn read(&self, value: i32, mode: Mode) -> i32 {
        match mode {
            Mode::Immediate => self.mem[value as usize],
            Mode::Address => self.mem[self.mem[value as usize] as usize],
        }
    }

    fn write(&mut self, address: Address, value: i32) {
        self.mem[address as usize] = value;
    }

    pub fn run(&mut self, noun: Option<i32>, verb: Option<i32>) {
        if let Some(noun) = noun {
            self.write(1, noun);
        }
        if let Some(verb) = verb {
            self.write(2, verb);
        }
        let mut ip = 0; // instruction pointer

        loop {
            match decode(self.read(ip, Mode::Immediate)) {
                Instruction::Add(mode1, mode2) => {
                    let result = self.read(ip + 1, mode1) + self.read(ip + 2, mode2);
                    self.write(self.read(ip + 3, Mode::Immediate), result);
                    ip += 4;
                }
                Instruction::Multiply(mode1, mode2) => {
                    let result = self.read(ip + 1, mode1) * self.read(ip + 2, mode2);
                    self.write(self.read(ip + 3, Mode::Immediate), result);
                    ip += 4;
                }
                Instruction::Input => {
                    let mut input = String::new();
                    print!("Input integer: ");
                    io::stdout().flush().unwrap();
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            let integer = i32::from_str(input.trim()).expect("invalid integer");
                            self.write(self.read(ip + 1, Mode::Immediate), integer);
                        }
                        Err(error) => panic!("error: {}", error),
                    }
                    ip += 2;
                }
                Instruction::Output(mode) => {
                    let value = self.read(ip + 1, mode);
                    println!("{}", value);
                    ip += 2;
                }
                Instruction::Halt => break,
            }
        }
    }
}

fn divmod(value: &mut i32, divisor: i32) -> i32 {
    let res = *value % divisor;
    *value /= divisor;
    res
}

impl From<i32> for Mode {
    fn from(mode: i32) -> Self {
        match mode {
            0 => Mode::Address,
            1 => Mode::Immediate,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;
    use std::fs;

    #[test]
    fn test_decode() {
        assert_eq!(
            decode(1002),
            Instruction::Multiply(Mode::Address, Mode::Immediate)
        )
    }

    #[test]
    fn test_day2() {
        let input = fs::read_to_string("input/day2.txt").unwrap();
        let mut data = input::read_separated_line(',', &input).unwrap();
        let mut program = Memory::new(&mut data);

        // Check that day2 still works wirh run through this implementation
        program.run(Some(12), Some(2));
        assert_eq!(program.read(0, Mode::Immediate), 4138658);
    }
}
