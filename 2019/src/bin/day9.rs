use std::{fs, io};

use advent_of_code::computer::Computer;
use advent_of_code::input;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/day9.txt")?;
    let program = input::read_separated_line(',', &input)?;

    let mut computer = Computer::new('1', program.clone(), vec![1], vec![]);
    computer.run(None, None);
    println!("Part 1:  {}", computer.output());

    //    let mut computer = computer::Computer::new('2', data.clone(), vec![5], vec![]);
    //    computer.run(None, None);
    //    println!("Part 2:  {}", computer.output());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_example1() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut computer = Computer::new('T', program, vec![], vec![]);
        computer.run(None, None);

        assert_eq!(computer.output(), 1219070632396864);
    }

    #[test]
    fn test_relative_example2() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut computer = Computer::new('T', program, vec![], vec![]);
        computer.run(None, None);

        assert_eq!(computer.output(), 99);
    }

    #[test]
    fn test_relative_example3() {
        let program = vec![104, 1125899906842624, 99];
        let mut computer = Computer::new('T', program, vec![], vec![]);
        computer.run(None, None);

        assert_eq!(computer.output(), 1125899906842624);
    }
}
