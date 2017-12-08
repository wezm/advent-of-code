use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::str::FromStr;

type Registers = HashMap<String, i64>;

fn main() {
    let mut input = String::new();
    let mut file = File::open("input").expect("unable to open input file");
    file.read_to_string(&mut input).expect("error reading input");

    let max = max_value(&eval(&input));
    println!("{}", max);
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

fn eval_line(registers: &mut Registers, line: &str) {
    let tokens = line.split_whitespace().collect::<Vec<_>>();

    // b inc 5 if a > 1
    let register = tokens[0];
    let op = tokens[1];
    let op_operand = i64::from_str(tokens[2]).expect("op operand is not a number");
    let cond_reg = tokens[4];
    let cond_op = tokens[5];
    let cond_operand = i64::from_str(tokens[6]).expect("cond operand is not a number");

    if eval_cond(registers.get(cond_reg).map(|val| *val).unwrap_or(0), cond_op, cond_operand) {
        if !registers.contains_key(register) {
            registers.insert(register.to_owned(), 0);
        }

        let register_val = registers.get_mut(register).unwrap();
        match op {
            "inc" => *register_val += op_operand,
            "dec" => *register_val -= op_operand,
            _ => panic!("unknown operation, '{}'", op)
        }
    }
}

fn eval(input: &str) -> Registers {
    let mut registers = HashMap::new();

    for line in input.lines() {
        eval_line(&mut registers, line);
    }

    registers
}

fn max_value(registers: &Registers) -> i64 {
    *registers.values().max().unwrap()
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

    assert_eq!(eval(input), expected);
}

#[test]
fn test_max_value() {
    let mut registers = HashMap::new();
    registers.insert("a".to_owned(), 1);
    registers.insert("c".to_owned(), -10);

    assert_eq!(max_value(&registers), 1);
}
