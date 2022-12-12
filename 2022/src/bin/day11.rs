use regex::Regex;
use std::error::Error;

#[derive(Debug)]
struct Monkey {
    items: Vec<isize>,
    op: Operation,
    test: Test,
    items_inspected: usize,
    relief: isize,
}

#[derive(Debug)]
struct Operation {
    operator: char,
    operand1: Operand,
    operand2: Operand,
}

#[derive(Debug)]
struct Test {
    divisible_by: isize,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug)]
enum Operand {
    Old,
    Const(isize),
}

struct TurnResult {
    if_true: (usize, Vec<isize>),
    if_false: (usize, Vec<isize>),
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input/day_11.txt")?;
    let re = Regex::new(
        r"^Monkey \d:
  Starting items: (?P<items>.+)
  Operation: new = (?P<opa>\w+) (?P<op>.) (?P<opb>\w+)
  Test: divisible by (?P<div>\d+)
    If true: throw to monkey (?P<if_true>\d+)
    If false: throw to monkey (?P<if_false>\d+)$",
    )?;

    part1(&input, &re)?;
    part2(&input, &re)?;

    Ok(())
}

fn part1(input: &str, re: &Regex) -> Result<(), Box<dyn Error>> {
    let monkey_business = run(input, re, 3, 20)?;
    println!("Monkey business: {}", monkey_business);
    Ok(())
}

fn part2(input: &str, re: &Regex) -> Result<(), Box<dyn Error>> {
    let monkey_business = run(input, re, 1, 10_000)?;
    println!("Monkey business: {}", monkey_business);
    Ok(())
}

fn run(input: &str, re: &Regex, relief: isize, rounds: usize) -> Result<usize, Box<dyn Error>> {
    let mut monkeys = input
        .split("\n\n")
        .map(|s| parse_monkey(s, re, relief))
        .collect::<Result<Vec<_>, _>>()?;
    let modulus = monkeys
        .iter()
        .fold(1, |product, m| product * m.test.divisible_by);
    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            take_turn(i, &mut monkeys, modulus)
        }
    }

    monkeys.sort_by(|a, b| a.items_inspected.cmp(&b.items_inspected));
    Ok(monkeys[monkeys.len() - 2].items_inspected * monkeys[monkeys.len() - 1].items_inspected)
}

fn parse_monkey(input: &str, re: &Regex, relief: isize) -> Result<Monkey, Box<dyn Error>> {
    let caps = re
        .captures(input.trim())
        .ok_or_else(|| format!("text did not match regex: {}", input))?;
    let items = caps
        .name("items")
        .unwrap()
        .as_str()
        .split(", ")
        .map(|item| item.parse())
        .collect::<Result<Vec<isize>, _>>()?;
    let opa = parse_operand(caps.name("opa").unwrap().as_str())?;
    let opb = parse_operand(caps.name("opb").unwrap().as_str())?;
    let op = caps.name("op").unwrap().as_str().chars().next().unwrap();
    let div = caps.name("div").unwrap().as_str().parse()?;
    let if_true = caps.name("if_true").unwrap().as_str().parse()?;
    let if_false = caps.name("if_false").unwrap().as_str().parse()?;

    Ok(Monkey {
        items,
        op: Operation {
            operator: op,
            operand1: opa,
            operand2: opb,
        },
        test: Test {
            divisible_by: div,
            if_true,
            if_false,
        },
        items_inspected: 0,
        relief,
    })
}

fn parse_operand(input: &str) -> Result<Operand, Box<dyn Error>> {
    match input {
        "old" => Ok(Operand::Old),
        otherwise => {
            let val = otherwise.parse()?;
            Ok(Operand::Const(val))
        }
    }
}

fn take_turn(i: usize, monkeys: &mut [Monkey], modulus: isize) {
    let mut res = monkeys[i].take_turn(modulus);
    monkeys[res.if_true.0].items.append(&mut res.if_true.1);
    monkeys[res.if_false.0].items.append(&mut res.if_false.1);
}

impl Monkey {
    fn take_turn(&mut self, modulus: isize) -> TurnResult {
        let mut if_true = Vec::new();
        let mut if_false = Vec::new();

        for item in self.items.iter().copied() {
            let new = self.op.eval(item, modulus) / self.relief;
            if self.test.test(new) {
                if_true.push(new);
            } else {
                if_false.push(new);
            }
        }
        self.items_inspected += self.items.len();
        self.items.clear();
        TurnResult {
            if_true: (self.test.if_true, if_true),
            if_false: (self.test.if_false, if_false),
        }
    }
}

impl Operation {
    fn eval(&self, old: isize, modulus: isize) -> isize {
        (match self.operator {
            '+' => self.operand1.val(old) + self.operand2.val(old),
            '*' => match self.operand1.val(old).checked_mul(self.operand2.val(old)) {
                Some(x) => x,
                None => panic!(
                    "mul fail {} x {}",
                    self.operand1.val(old),
                    self.operand2.val(old)
                ),
            },
            _ => unreachable!("unknown op"),
        }) % modulus
    }
}

impl Operand {
    fn val(&self, old: isize) -> isize {
        match self {
            Operand::Old => old,
            Operand::Const(v) => *v,
        }
    }
}

impl Test {
    fn test(&self, item: isize) -> bool {
        item % self.divisible_by == 0
    }
}
