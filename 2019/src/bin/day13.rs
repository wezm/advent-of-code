use std::{fs, io};

use advent_of_code::computer::{ComputeResult, Computer};
use advent_of_code::input;
use advent_of_code::point::Point;
use std::collections::HashMap;
use std::convert::TryFrom;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/day13.txt")?;
    let program = input::read_separated_line(',', &input)?;

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

    Ok(())
}
