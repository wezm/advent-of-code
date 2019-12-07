use advent_of_code::{computer, input};
use std::{fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/day5.txt")?;
    let data = input::read_separated_line(',', &input)?;

    let mut program = data.clone();
    let mut computer = computer::Computer::new(&mut program, vec![1]);
    computer.run(None, None);
    println!("Part 1:  {:?}", computer.output());

    let mut program = data.clone();
    let mut computer = computer::Computer::new(&mut program, vec![5]);
    computer.run(None, None);
    println!("Part 2:  {:?}", computer.output());

    Ok(())
}
