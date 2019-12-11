use std::{fs, io};

use advent_of_code::computer::{ComputeResult, Computer};
use advent_of_code::input;
use advent_of_code::point::Point;
use std::collections::HashMap;

#[derive(Debug)]
enum Rotate {
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Robot {
    heading: Direction,
    position: Point,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Colour {
    Black,
    White,
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/day11.txt")?;
    let program = input::read_separated_line(',', &input)?;

    let painted = run_robot(Colour::Black, program.clone());
    println!("Part 1:  {}", painted.len());

    let painted = run_robot(Colour::White, program.clone());
    // Determine dimensions of image
    let minx = painted.iter().map(|(point, _)| point.0).min().unwrap();
    let maxx = painted.iter().map(|(point, _)| point.0).max().unwrap();
    let miny = painted.iter().map(|(point, _)| point.1).min().unwrap();
    let maxy = painted.iter().map(|(point, _)| point.1).max().unwrap();

    println!();
    for y in (miny..=maxy).rev() {
        for x in minx..=maxx {
            match painted.get(&Point(x, y)) {
                Some(Colour::White) => print!("█"),
                Some(Colour::Black) | None => print!(" "),
            }
        }
        println!();
    }
    println!();

    Ok(())
}

fn run_robot(initial_colour: Colour, program: Vec<i64>) -> HashMap<Point, Colour> {
    let mut robot = Robot::new();
    let mut painted = HashMap::new();
    let initial_input = match initial_colour {
        Colour::Black => 0,
        Colour::White => 1,
    };
    let mut computer = Computer::new('1', program, vec![initial_input], vec![]);

    loop {
        match computer.run(None, None) {
            ComputeResult::Halted => break,
            ComputeResult::NeedsInput => {
                let output = computer.output().to_vec();
                painted.insert(robot.position, Colour::from(output[0]));
                let rotation = Rotate::from(output[1]);
                robot.rotate(rotation);
                robot.go_forward();
                computer.clear_output();

                let tile_colour = painted
                    .get(&robot.position)
                    .map(|colour| match colour {
                        Colour::Black => 0,
                        Colour::White => 1,
                    })
                    .unwrap_or(0);
                computer.input(tile_colour);
            }
        }
    }

    painted
}

impl Robot {
    fn new() -> Self {
        Robot {
            heading: Direction::North,
            position: Point(0, 0),
        }
    }

    fn rotate(&mut self, rotate: Rotate) {
        self.heading = match rotate {
            Rotate::Left => match self.heading {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
                Direction::West => Direction::South,
            },
            Rotate::Right => match self.heading {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            },
        }
    }

    fn go_forward(&mut self) {
        match self.heading {
            Direction::North => self.position.1 += 1,
            Direction::South => self.position.1 -= 1,
            Direction::East => self.position.0 += 1,
            Direction::West => self.position.0 -= 1,
        }
    }
}

impl From<i64> for Rotate {
    fn from(n: i64) -> Self {
        match n {
            0 => Rotate::Left,
            1 => Rotate::Right,
            _ => unreachable!(),
        }
    }
}

impl From<i64> for Colour {
    fn from(n: i64) -> Self {
        match n {
            0 => Colour::Black,
            1 => Colour::White,
            _ => unreachable!(),
        }
    }
}
