use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::str::FromStr;

pub struct RegisterMachine {
    registers: HashMap<String, i64>,
    max_seen: i64
}

impl RegisterMachine {
    pub fn new() -> Self {
        RegisterMachine {
            registers: HashMap::new(),
            max_seen: 0
        }
    }

    pub fn max(&self) -> i64 {
        *self.registers.values().max().unwrap()
    }

    pub fn registers(&self) -> &HashMap<String, i64> {
        &self.registers
    }

    pub fn max_seen(&self) -> i64 {
        self.max_seen
    }

    fn eval_cond(lhs: i64, cond: &str, rhs: i64) -> bool {
        match cond {
            "==" => lhs == rhs,
            "!=" => lhs != rhs,
            ">"  => lhs > rhs,
            ">=" => lhs >= rhs,
            "<"  => lhs < rhs,
            "<=" => lhs <= rhs,
            _ => panic!("unknown cond op, '{}'", cond)
        }
    }

    fn eval_line(&mut self, line: &str) {
        let tokens = line.split_whitespace().collect::<Vec<_>>();

        // b inc 5 if a > 1
        let register = tokens[0];
        let op = tokens[1];
        let op_operand = i64::from_str(tokens[2]).expect("op operand is not a number");
        let cond_reg = tokens[4];
        let cond_op = tokens[5];
        let cond_operand = i64::from_str(tokens[6]).expect("cond operand is not a number");

        if RegisterMachine::eval_cond(self.registers.get(cond_reg).map(|val| *val).unwrap_or(0), cond_op, cond_operand) {
            if !self.registers.contains_key(register) {
                self.registers.insert(register.to_owned(), 0);
            }

            let register_val = self.registers.get_mut(register).unwrap();
            match op {
                "inc" => *register_val += op_operand,
                "dec" => *register_val -= op_operand,
                _ => panic!("unknown operation, '{}'", op)
            }
            self.max_seen = self.max_seen.max(*register_val)
        }
    }

    fn eval(&mut self, input: &str) {
        for line in input.lines() {
            self.eval_line(line);
        }
    }
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("input").expect("unable to open input file");
    file.read_to_string(&mut input).expect("error reading input");

    let mut machine = RegisterMachine::new();
    machine.eval(&input);
    println!("Part 1: {}", machine.max());
    println!("Part 2: {}", machine.max_seen());

}

#[test]
fn test_eval() {
    let input = "b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10\n";
    let mut expected = HashMap::new();

    // a is increased by 1 (to 1) because b is less than 5 (it is 0).
    // c is decreased by -10 (to 10) because a is now greater than or equal to 1 (it is 1).
    // c is increased by -20 (to -10) because c is equal to 10.
    expected.insert("a".to_owned(), 1);
    expected.insert("c".to_owned(), -10);

    let mut machine = RegisterMachine::new();
    machine.eval(input);

    assert_eq!(machine.registers(), &expected);
}

#[test]
fn test_max_value() {
    let input = "b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10\n";
    let mut expected = HashMap::new();

    // a is increased by 1 (to 1) because b is less than 5 (it is 0).
    // c is decreased by -10 (to 10) because a is now greater than or equal to 1 (it is 1).
    // c is increased by -20 (to -10) because c is equal to 10.
    expected.insert("a".to_owned(), 1);
    expected.insert("c".to_owned(), -10);

    let mut machine = RegisterMachine::new();
    machine.eval(input);

    assert_eq!(machine.max(), 1);
}

#[test]
fn test_max_seen() {
    let input = "b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10\n";
    let mut expected = HashMap::new();

    // a is increased by 1 (to 1) because b is less than 5 (it is 0).
    // c is decreased by -10 (to 10) because a is now greater than or equal to 1 (it is 1).
    // c is increased by -20 (to -10) because c is equal to 10.
    expected.insert("a".to_owned(), 1);
    expected.insert("c".to_owned(), -10);

    let mut machine = RegisterMachine::new();
    machine.eval(input);

    assert_eq!(machine.max_seen(), 10);
}
