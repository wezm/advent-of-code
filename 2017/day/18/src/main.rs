use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::collections::HashMap;

type Register = char;

#[derive(Debug)]
enum Operand {
    Register(Register),
    Immediate(isize),
}

impl Operand {
    fn new(register_or_value: &str) -> Self {
        if let Ok(value) = isize::from_str(register_or_value) {
            Operand::Immediate(value)
        }
        else {
            Operand::Register(register_or_value.chars().next().unwrap())
        }
    }

    fn value(&self, machine: &Machine) -> isize {
        match *self {
            // TODO: Sdd a get to Machine
            Operand::Register(ref reg) => machine.registers.get(reg).map(|val| *val).unwrap_or(0),
            Operand::Immediate(val) => val,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Snd(Operand),
    Set(Operand, Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Mod(Operand, Operand),
    Rcv(Operand),
    Jgz(Operand, Operand),
}

impl Instruction {
    fn new(line: &str) -> Self {
        use Instruction::*;

        let asm = line.split_whitespace().collect::<Vec<&str>>();
        let instruction = asm[0];

        match instruction {
            "snd" => Snd(Operand::new(asm[1])),
            "set" => Set(Operand::new(asm[1]), Operand::new(asm[2])),
            "add" => Add(Operand::new(asm[1]), Operand::new(asm[2])),
            "mul" => Mul(Operand::new(asm[1]), Operand::new(asm[2])),
            "mod" => Mod(Operand::new(asm[1]), Operand::new(asm[2])),
            "rcv" => Rcv(Operand::new(asm[1])),
            "jgz" => Jgz(Operand::new(asm[1]), Operand::new(asm[2])),
            _ => panic!("bad instruction")
        }
    }
}

struct Program(Vec<Instruction>);

impl Program {
    fn load(path: &str) -> Program {
        let file = File::open(path).expect("unable to open input file");
        let reader = BufReader::new(file);

        let program = reader.lines()
            .map(|line| Instruction::new(line.as_ref().unwrap()))
            .collect::<Vec<Instruction>>();
        Program(program)
    }
}

// TODO: Machine executes programs and instructions
#[derive(Debug)]
struct Machine {
    registers: HashMap<char, isize>,
    pc: isize,
    last_played: isize,
    last_received: isize,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            registers: HashMap::new(),
            pc: 0,
            last_played: 0,
            last_received: 0,
        }
    }

    pub fn run(&mut self, program: &Program) {
        loop {
            if self.pc < 0 || self.pc >= program.0.len() as isize {
                break;
            }

            let instruction = &program.0[self.pc as usize];
            self.execute(instruction);

            if self.last_received != 0 {
                break;
            }
        }
    }

    fn get(&self, register: char) -> isize {
        self.registers.get(&register).map(|val| *val).unwrap_or(0)
    }

    fn set(&mut self, register: char, value: isize) {
        self.registers.insert(register, value);
    }

    fn execute(&mut self, instruction: &Instruction) {
        use Instruction::*;

        // TODO; Increment pc

        match *instruction {
            Snd(ref op) => {
                self.last_played = op.value(self);
            }
            Set(Operand::Register(reg), ref val) => {
                let value = val.value(self);
                self.set(reg, value);
            }
            Add(Operand::Register(reg), ref val) => {
                let reg_value = self.get(reg);
                let value = val.value(self);
                self.set(reg, reg_value + value);
            }
            Mul(Operand::Register(reg), ref val) => {
                let reg_value = self.get(reg);
                let value = val.value(self);
                self.set(reg, reg_value * value);
            }
            Mod(Operand::Register(reg), ref val) => {
                let reg_value = self.get(reg);
                let value = val.value(self);
                self.set(reg, reg_value % value);
            }
            Rcv(ref op) => {
                let value = op.value(self);
                if value != 0 {
                    self.last_received = self.last_played;
                }
            }
            Jgz(_, _) => { /* handled below */ }
            _ => panic!("bad argument")
        };

        // Update PC
        match *instruction {
            Jgz(ref x, ref offset) => {
                let x_value = x.value(self);
                let offset_value = offset.value(self);

                if x_value != 0 {
                    self.pc += offset_value;
                }
                else {
                    self.pc += 1;
                }
            }
            _ => self.pc += 1
        };
    }
}

fn main() {
    let program = Program::load("input");
    let mut machine = Machine::new();

    machine.run(&program);

    println!("{:?}", machine);
}
