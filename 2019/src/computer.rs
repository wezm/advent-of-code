type Address = i32;

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Add(Mode, Mode),
    Multiply(Mode, Mode),
    Input,
    Output(Mode),
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode),
    Equals(Mode, Mode),
    Halt,
}

#[derive(Debug, Eq, PartialEq)]
enum Mode {
    Immediate,
    Address,
}

pub struct Computer<I: Input, O: Output> {
    memory: Vec<i32>,
    input: I,
    output: O,
}

pub trait Input {
    fn read(&mut self) -> Option<i32>;
}

pub trait Output {
    fn write(&mut self, value: i32);

    fn last_value(&self) -> i32;
}

impl Input for Vec<i32> {
    fn read(&mut self) -> Option<i32> {
        self.pop()
    }
}

impl Output for Vec<i32> {
    fn write(&mut self, value: i32) {
        self.push(value)
    }

    fn last_value(&self) -> i32 {
        *self.last().unwrap()
    }
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
        5 => Instruction::JumpIfTrue(
            Mode::from(divmod(&mut instruction, 10)),
            Mode::from(divmod(&mut instruction, 10)),
        ),
        6 => Instruction::JumpIfFalse(
            Mode::from(divmod(&mut instruction, 10)),
            Mode::from(divmod(&mut instruction, 10)),
        ),
        7 => Instruction::LessThan(
            Mode::from(divmod(&mut instruction, 10)),
            Mode::from(divmod(&mut instruction, 10)),
        ),
        8 => Instruction::Equals(
            Mode::from(divmod(&mut instruction, 10)),
            Mode::from(divmod(&mut instruction, 10)),
        ),
        99 => Instruction::Halt,
        _ => panic!("Invalid opcode: {}", opcode),
    }
}

impl<I, O> Computer<I, O> where I: Input, O: Output {
    pub fn new(memory: Vec<i32>, input: I, output: O) -> Self {
        Computer {
            memory,
            input,
            output,
        }
    }

    fn read(&self, value: i32, mode: Mode) -> i32 {
        match mode {
            Mode::Immediate => self.memory[value as usize],
            Mode::Address => self.memory[self.memory[value as usize] as usize],
        }
    }

    fn write(&mut self, address: Address, value: i32) {
        self.memory[address as usize] = value;
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
                    let value = self.input.read().expect("no more input");
                    self.write(self.read(ip + 1, Mode::Immediate), value);
                    ip += 2;
                }
                Instruction::Output(mode) => {
                    self.output.write(self.read(ip + 1, mode));
                    ip += 2;
                }
                Instruction::JumpIfTrue(mode1, mode2) => {
                    if self.read(ip + 1, mode1) != 0 {
                        ip = self.read(ip + 2, mode2);
                    } else {
                        ip += 3;
                    }
                }
                Instruction::JumpIfFalse(mode1, mode2) => {
                    if self.read(ip + 1, mode1) == 0 {
                        ip = self.read(ip + 2, mode2);
                    } else {
                        ip += 3;
                    }
                }
                Instruction::LessThan(mode1, mode2) => {
                    if self.read(ip + 1, mode1) < self.read(ip + 2, mode2) {
                        self.write(self.read(ip + 3, Mode::Immediate), 1);
                    } else {
                        self.write(self.read(ip + 3, Mode::Immediate), 0);
                    }
                    ip += 4;
                }
                Instruction::Equals(mode1, mode2) => {
                    if self.read(ip + 1, mode1) == self.read(ip + 2, mode2) {
                        self.write(self.read(ip + 3, Mode::Immediate), 1);
                    } else {
                        self.write(self.read(ip + 3, Mode::Immediate), 0);
                    }
                    ip += 4;
                }
                Instruction::Halt => break,
            }
        }
    }

    pub fn output(&self) -> i32 {
        self.output.last_value()
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
        let mut program = Computer::new(&mut data, vec![]);

        // Check that day2 still works wirh run through this implementation
        program.run(Some(12), Some(2));
        assert_eq!(program.read(0, Mode::Immediate), 4138658);
    }
}
