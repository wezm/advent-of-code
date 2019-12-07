use advent_of_code::{computer, input};
use std::{fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/day5.txt")?;
    let mut data = input::read_separated_line(',', &input)?;
    let mut program = computer::Computer::new(&mut data);

    program.run(None, None);

    Ok(())
}
