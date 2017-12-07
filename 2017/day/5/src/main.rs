use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let file = File::open("input").expect("unable to open input file");
    let reader = BufReader::new(file);

    let offsets = reader.lines()
        .map(|digit| i32::from_str(&digit.unwrap()).unwrap())
        .collect::<Vec<_>>();

    let steps = steps_to_exit(&offsets);
    println!("{}", steps);
}

fn steps_to_exit(jump_offsets: &[i32]) -> i32 {
    let mut offsets = jump_offsets.to_vec();
    let mut pc = 0i32;
    let mut steps = 0;

    loop {
        if let Some(offset) = offsets.get_mut(pc as usize) {
            pc += *offset;
            *offset += 1;
            steps += 1;
        }
        else {
            break;
        }

        if pc < 0 {
            break;
        }
    }

    steps
}

#[test]
fn test_example() {
    let jumps = vec![0, 3, 0, 1, -3];
    assert_eq!(steps_to_exit(&jumps), 5);
}
