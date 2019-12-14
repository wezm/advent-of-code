use std::{fs, io};

use advent_of_code::computer::{ComputeResult, Computer};
use advent_of_code::input;
use advent_of_code::point::Point;
use std::collections::HashMap;
use std::convert::TryFrom;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/day13.txt")?;
    let program = input::read_separated_line(',', &input)?;

    part1(program.clone());
    part2(program);

    Ok(())
}

fn part1(program: Vec<i64>) {
    let mut computer = Computer::new('G', program, vec![], vec![]);
    let res = computer.run(None, None);
    assert_eq!(res, ComputeResult::Halted);
    // Process the output
    let final_state: HashMap<_, _> = computer
        .output()
        .chunks(3)
        .map(|chunks| {
            (
                Point(
                    i32::try_from(chunks[0]).unwrap(),
                    i32::try_from(chunks[1]).unwrap(),
                ),
                i32::try_from(chunks[2]).unwrap(),
            )
        })
        .collect();
    // How many block tiles are on the screen when the game exits?
    let block_count = final_state
        .iter()
        .filter(|(_point, &tile_id)| tile_id == 2)
        .count();
    println!("Part 1: {}", block_count);
}

fn part2(mut program: Vec<i64>) {
    // Memory address 0 represents the number of quarters that have been inserted; set it to 2 to play for free.
    program[0] = 2;
    let mut computer = Computer::new('G', program, vec![], vec![]);

    let mut score = 0;
    let mut paddle = None;
    let mut ball = None;
    let mut width = 0;
    let mut height = 0;
    let mut screen = HashMap::new();
    loop {
        let res = computer.run(None, None);
        screen.extend(computer.output().chunks(3).filter_map(|chunk| match chunk {
            [-1, 0, new_score] => {
                score = *new_score;
                None
            }
            [x, y, 0] => Some(((*x, *y), ' ')),
            [x, y, 1] => Some(((*x, *y), '|')),
            [x, y, 2] => Some(((*x, *y), '#')),
            [x, y, 3] => {
                paddle = Some((*x, *y));
                Some(((*x, *y), '_'))
            }
            [x, y, 4] => {
                ball = Some((*x, *y));
                Some(((*x, *y), '*'))
            }
            _ => unreachable!(),
        }));

        if width == 0 {
            computer.output().chunks(3).for_each(|chunk| match chunk {
                [x, y, _] if *x >= 0 => {
                    if *x > width {
                        width = *x;
                    }
                    if *y > height {
                        height = *y;
                    }
                }
                _ => {}
            })
        }

        // Show screen state
        for y in 0..=height {
            for x in 0..=width {
                if let Some(tile) = screen.get(&(x, y)) {
                    print!("{}", tile);
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        println!();

        // Find out where the paddle and ball are, provide joystick input accordingly
        if let (Some(ball), Some(paddle)) = (ball, paddle) {
            let input = if paddle.0 < ball.0 {
                1
            } else if paddle.0 > ball.0 {
                -1
            } else {
                0
            };
            computer.input(input);
        } else {
            panic!("ball or paddle missing");
        }

        computer.clear_output();

        if res == ComputeResult::Halted {
            break;
        }
    }

    println!("Part 2: {}", score);
}
