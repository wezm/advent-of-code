use std::cell::RefCell;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::ops::{Index, IndexMut};
use std::rc::Rc;

type Address = i64;

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Add(Mode, Mode, Mode),
    Multiply(Mode, Mode, Mode),
    Input(Mode),
    Output(Mode),
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode, Mode),
    Equals(Mode, Mode, Mode),
    AdjustRelativeBase(Mode),
    Halt,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Mode {
    Immediate,
    Address,
    Relative,
}

#[derive(Debug)]
pub struct Pipe {
    queue: VecDeque<i64>,
    last: Option<i64>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ComputeResult {
    Halted,
    NeedsInput,
}

struct Memory(Vec<i64>);

pub struct Computer<I: Input, O: Output> {
    name: char,
    ip: i64,
    memory: Memory,
    input: I,
    output: O,
    relative_base: i64,
}

pub trait Input {
    fn read(&mut self) -> Option<i64>;

    fn push(&mut self, val: i64);
}

pub trait Output {
    fn write(&mut self, value: i64);

    fn last_value(&self) -> i64;

    fn get(&self) -> &[i64];

    fn clear(&mut self);
}

impl Input for Vec<i64> {
    fn read(&mut self) -> Option<i64> {
        self.pop()
    }

    fn push(&mut self, val: i64) {
        self.insert(0, val)
    }
}

impl Output for Vec<i64> {
    fn write(&mut self, value: i64) {
        self.push(value)
    }

    fn last_value(&self) -> i64 {
        *self.last().unwrap()
    }

    fn get(&self) -> &[i64] {
        self
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl Input for Rc<RefCell<Pipe>> {
    fn read(&mut self) -> Option<i64> {
        dbg!(self.borrow_mut().queue.pop_front())
    }

    fn push(&mut self, val: i64) {
        self.borrow_mut().queue.push_back(val)
    }
}

impl Output for Rc<RefCell<Pipe>> {
    fn write(&mut self, value: i64) {
        let mut pipe = self.borrow_mut();
        pipe.last = Some(value);
        pipe.queue.push_back(value);
    }

    fn last_value(&self) -> i64 {
        self.borrow().last.unwrap()
    }

    fn get(&self) -> &[i64] {
        unimplemented!()
    }

    fn clear(&mut self) {
        unimplemented!()
    }
}

fn decode(mut instruction: i64) -> Instruction {
    let opcode = divmod(&mut instruction, 100);

    match opcode {
        1 => Instruction::Add(
            Mode::from(divmod(&mut instruction, 10)),
            Mode::from(divmod(&mut instruction, 10)),
            Mode::from(divmod(&mut instruction, 10)),
        ),
        2 => Instruction::Multiply(
            Mode::from(divmod(&mut instruction, 10)),
            Mode::from(divmod(&mut instruction, 10)),
            Mode::from(divmod(&mut instruction, 10)),
        ),
        3 => Instruction::Input(Mode::from(divmod(&mut instruction, 10))),
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
            Mode::from(divmod(&mut instruction, 10)),
        ),
        8 => Instruction::Equals(
            Mode::from(divmod(&mut instruction, 10)),
            Mode::from(divmod(&mut instruction, 10)),
            Mode::from(divmod(&mut instruction, 10)),
        ),
        9 => Instruction::AdjustRelativeBase(Mode::from(divmod(&mut instruction, 10))),
        99 => Instruction::Halt,
        _ => panic!("Invalid opcode: {}", opcode),
    }
}

impl<I, O> Computer<I, O>
where
    I: Input,
    O: Output,
{
    pub fn new(name: char, memory: Vec<i64>, input: I, output: O) -> Self {
        Computer {
            name,
            ip: 0,
            memory: Memory(memory),
            input,
            output,
            relative_base: 0,
        }
    }

    pub fn name(&self) -> char {
        self.name
    }

    fn read(&self, value: i64, mode: Mode) -> i64 {
        match mode {
            Mode::Immediate => self.memory[value as usize],
            Mode::Address => self.memory[self.memory[value as usize] as usize],
            // The address a relative mode parameter refers to is itself plus the current relative base.
            Mode::Relative => {
                self.memory
                    [usize::try_from(self.relative_base + self.memory[value as usize]).unwrap()]
            }
        }
    }

    fn write(&mut self, address: Address, value: i64, mode: Mode) {
        match mode {
            Mode::Immediate => panic!("attempt to write with immediate mode"),
            Mode::Address => self.memory[address as usize] = value,
            Mode::Relative => {
                self.memory[usize::try_from(self.relative_base + address).unwrap()] = value
            }
        }
    }

    pub fn run(&mut self, noun: Option<i64>, verb: Option<i64>) -> ComputeResult {
        println!("{}: resume ip = {}", self.name, self.ip);
        if let Some(noun) = noun {
            self.write(1, noun, Mode::Address);
        }
        if let Some(verb) = verb {
            self.write(2, verb, Mode::Address);
        }

        loop {
            match decode(self.read(self.ip, Mode::Immediate)) {
                Instruction::Add(mode1, mode2, write_mode) => {
                    let result = self.read(self.ip + 1, mode1) + self.read(self.ip + 2, mode2);
                    self.write(self.read(self.ip + 3, Mode::Immediate), result, write_mode);
                    self.ip += 4;
                }
                Instruction::Multiply(mode1, mode2, write_mode) => {
                    let result = self.read(self.ip + 1, mode1) * self.read(self.ip + 2, mode2);
                    self.write(self.read(self.ip + 3, Mode::Immediate), result, write_mode);
                    self.ip += 4;
                }
                Instruction::Input(write_mode) => match self.input.read() {
                    Some(value) => {
                        self.write(self.read(self.ip + 1, Mode::Immediate), value, write_mode);
                        self.ip += 2;
                    }
                    None => {
                        // println!("{}: pause ip = {}", self.name, self.ip);
                        return ComputeResult::NeedsInput;
                    }
                },
                Instruction::Output(mode) => {
                    let value = self.read(self.ip + 1, mode);
                    println!("{}: output {}", self.name, value);
                    self.output.write(value);
                    self.ip += 2;
                }
                Instruction::JumpIfTrue(mode1, mode2) => {
                    if self.read(self.ip + 1, mode1) != 0 {
                        self.ip = self.read(self.ip + 2, mode2);
                    } else {
                        self.ip += 3;
                    }
                }
                Instruction::JumpIfFalse(mode1, mode2) => {
                    if self.read(self.ip + 1, mode1) == 0 {
                        self.ip = self.read(self.ip + 2, mode2);
                    } else {
                        self.ip += 3;
                    }
                }
                Instruction::LessThan(mode1, mode2, write_mode) => {
                    if self.read(self.ip + 1, mode1) < self.read(self.ip + 2, mode2) {
                        self.write(self.read(self.ip + 3, Mode::Immediate), 1, write_mode);
                    } else {
                        self.write(self.read(self.ip + 3, Mode::Immediate), 0, write_mode);
                    }
                    self.ip += 4;
                }
                Instruction::Equals(mode1, mode2, write_mode) => {
                    if self.read(self.ip + 1, mode1) == self.read(self.ip + 2, mode2) {
                        self.write(self.read(self.ip + 3, Mode::Immediate), 1, write_mode);
                    } else {
                        self.write(self.read(self.ip + 3, Mode::Immediate), 0, write_mode);
                    }
                    self.ip += 4;
                }
                Instruction::AdjustRelativeBase(mode) => {
                    let base = self.read(self.ip + 1, mode);
                    self.relative_base += base;
                    self.relative_base;
                    self.ip += 2;
                }
                Instruction::Halt => return ComputeResult::Halted,
            }
        }
    }

    pub fn last_output(&self) -> i64 {
        self.output.last_value()
    }

    pub fn input(&mut self, val: i64) {
        self.input.push(val)
    }

    pub fn output(&self) -> &[i64] {
        self.output.get()
    }

    pub fn clear_output(&mut self) {
        self.output.clear()
    }
}

impl Pipe {
    pub fn new(queue: VecDeque<i64>) -> Self {
        Pipe { queue, last: None }
    }

    pub fn push_front(&mut self, value: i64) {
        self.queue.push_front(value);
    }
}

fn divmod(value: &mut i64, divisor: i64) -> i64 {
    let res = *value % divisor;
    *value /= divisor;
    res
}

impl From<i64> for Mode {
    fn from(mode: i64) -> Self {
        match mode {
            0 => Mode::Address,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => unreachable!(),
        }
    }
}

impl Index<usize> for Memory {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.0.len() {
            // Out of range reads, read 0
            return &0;
        }

        self.0.index(index)
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.0.len() {
            // Out of range write, need to expand to allow
            let mut extra = vec![0; (index + 1) - self.0.len()];
            self.0.append(&mut extra);
        }

        self.0.index_mut(index)
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
            Instruction::Multiply(Mode::Address, Mode::Immediate, Mode::Address)
        )
    }

    #[test]
    fn test_day2() {
        let input = fs::read_to_string("input/day2.txt").unwrap();
        let data = input::read_separated_line(',', &input).unwrap();
        let mut program = Computer::new('2', data, vec![], vec![]);

        // Check that day2 still works wirh run through this implementation
        program.run(Some(12), Some(2));
        assert_eq!(program.read(0, Mode::Immediate), 4138658);
    }
}
