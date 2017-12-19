use std::io::Read;
use std::fs::File;
use std::ops::{Add, Sub};

enum Direction {
    Up,
    Down,
    Left,
    Right
}

use Direction::*;

impl Direction {
    fn left(&self) -> Direction {
        match *self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up
        }
    }

    fn right(&self) -> Direction {
        match *self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down
        }
    }
}

trait Move: Add + Sub + Sized {
    fn move_x(&self, direction: &Direction) -> Self;
    fn move_y(&self, direction: &Direction) -> Self;
}

impl Move for usize {
    fn move_x(&self, direction: &Direction) -> Self {
        match *direction {
            Up | Down => *self,
            Left => self - 1,
            Right => self + 1,
        }
    }

    fn move_y(&self, direction: &Direction) -> Self {
        match *direction {
            Up => self - 1,
            Down => self + 1,
            Left | Right => *self,
        }
    }
}

struct Diagram {
    data: Vec<u8>,
    width: usize,
    height: usize,
    stride: usize,
}

impl Diagram {
    fn new(data: Vec<u8>) -> Self {
        let width = data.iter().position(|&b| b == 0x0A).expect("no newlines?"); // \n
        let stride = width + 1; // Theres 1 byte of padding (the newline on every row)
        let height = data.len() / stride;

        Diagram { data, width, height, stride }
    }

    fn char_at(&self, row: usize, col: usize) -> char {
        self.data[row * self.stride + col] as char
    }

    fn start_col(&self) -> usize {
        self.data.iter().position(|&b| b == 0x7C).expect("unable to find start position") // |
    }
}

fn main() {
    let mut input = Vec::new();
    let mut file = File::open("input").expect("unable to open input file");
    file.read_to_end(&mut input).expect("error reading input");

    let diagram = Diagram::new(input);

    let (steps, path) = navigate(&diagram);

    println!("{} in {} steps", path, steps);
}

// This is pretty ugly... can be improved
fn navigate(diagram: &Diagram) -> (usize, String) {
    let mut col = diagram.start_col();
    let mut row = 0;
    let mut direction = Down;

    let mut path = String::new();
    let mut steps = 0;

    loop {
        steps += 1;
        let c = diagram.char_at(row, col);
        // println!("Visit ({}, {}): '{}'", col, row, c);

        if c >= 'A' && c <= 'Z' {
            path.push(c);
        }

        // Try continuing in the same direction
        let next_col = col.move_x(&direction);
        let next_row = row.move_y(&direction);
        if diagram.char_at(next_row, next_col) != ' ' {
            row = next_row;
            col = next_col;
            continue;
        }

        // Try turning left
        let left = direction.left();
        let next_col = col.move_x(&left);
        let next_row = row.move_y(&left);
        if diagram.char_at(next_row, next_col) != ' ' {
            row = next_row;
            col = next_col;
            direction = left;
            continue;
        }

        // Try turning right
        let right = direction.right();
        let next_col = col.move_x(&right);
        let next_row = row.move_y(&right);
        if diagram.char_at(next_row, next_col) != ' ' {
            row = next_row;
            col = next_col;
            direction = right;
            continue;
        }

        break;
    }

    (steps, path)
}

#[test]
fn test_example() {
let data = b"     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
                ";
    let diagram = Diagram::new(data.to_vec());

    assert_eq!(navigate(&diagram), (38, "ABCDEF".to_string()))
}
