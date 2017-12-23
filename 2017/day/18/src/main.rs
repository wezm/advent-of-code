use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender, Receiver, RecvTimeoutError};
use std::thread;
use std::sync::Arc;
use std::time::Duration;

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
    pid: usize,
    registers: HashMap<char, isize>,
    pc: isize,
    send_count: usize,
}

impl Machine {
    pub fn new(pid: usize) -> Machine {
        let mut registers = HashMap::new();
        registers.insert('p', pid as isize);

        Machine {
            pid,
            registers,
            pc: 0,
            send_count: 0,
        }
    }

    pub fn run(&mut self, program: &Program, send: Sender<isize>, recv: Receiver<isize>) {
        loop {
            if self.pc < 0 || self.pc >= program.0.len() as isize {
                println!("pid {}: send_count = {}", self.pid, self.send_count);
                break;
            }

            let instruction = &program.0[self.pc as usize];
            // println!("pid {}: {:?}", self.pid, instruction);
            if let Err(_err) = self.execute(instruction, &send, &recv) {
                println!("pid {}: send_count = {}", self.pid, self.send_count);
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

    fn execute(&mut self, instruction: &Instruction, send: &Sender<isize>, recv: &Receiver<isize>) -> Result<(), RecvTimeoutError> {
        use Instruction::*;

        match *instruction {
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
            Snd(ref op) => {
                let value = op.value(self);
                send.send(value).expect("error sending");
                self.send_count += 1;
                // println!("pid {}: sent {}", self.pid, value);
            }
            Rcv(Operand::Register(reg)) => {
                let value = recv.recv_timeout(Duration::from_secs(5))?;
                // println!("pid {}: recv {}", self.pid, value);
                self.set(reg, value);
            }
            Jgz(_, _) => { /* handled below */ }
            _ => panic!("bad argument")
        };

        // Update PC
        match *instruction {
            Jgz(ref x, ref offset) => {
                let x_value = x.value(self);
                let offset_value = offset.value(self);

                if x_value > 0 {
                    self.pc += offset_value;
                }
                else {
                    self.pc += 1;
                }
            }
            _ => self.pc += 1
        };

        Ok(())
    }
}

fn run(program: Program) {
    let program = Arc::new(program);
    let mut machine0 = Machine::new(0);
    let mut machine1 = Machine::new(1);

    let (send0, recv1) = channel();
    let (send1, recv0) = channel();

    let program0 = Arc::clone(&program);
    let thread0 = thread::spawn(move || {
        machine0.run(&program0, send0, recv0);
    });

    let program1 = Arc::clone(&program);
    let thread1 = thread::spawn(move || {
        machine1.run(&program1, send1, recv1);
    });

    thread0.join().unwrap();
    thread1.join().unwrap();
}

fn main() {
    run(Program::load("input"));
}

#[test]
fn test_part_two() {
    let input = r"snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";

    let program = input.lines()
        .map(|line| Instruction::new(line.as_ref()))
        .collect::<Vec<Instruction>>();
    run(Program(program));
}
