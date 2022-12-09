use std::collections::HashSet;
use std::error::Error;

type Pos = (i32, i32);

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input/day_9.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut tail_visited = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);

    for line in input.lines() {
        execute(line, &mut head, &mut tail, &mut tail_visited)?;
    }

    println!("Part 1: tail visited {} positions", tail_visited.len());

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn execute(
    line: &str,
    head: &mut Pos,
    tail: &mut Pos,
    visited: &mut HashSet<Pos>,
) -> Result<(), Box<dyn Error>> {
    // let h = *head;
    // let t = *tail;
    let instruction = line
        .split_once(' ')
        .and_then(|(x, count)| count.parse().ok().map(|count: u32| (x, count)));
    let (delta, count) = match instruction {
        Some(("U", count)) => ((0, 1), count),
        Some(("D", count)) => ((0, -1), count),
        Some(("L", count)) => ((-1, 0), count),
        Some(("R", count)) => ((1, 0), count),
        _ => Err(format!("invalid line: '{}'", line))?,
    };
    (0..count).for_each(|_| do_move(delta, head, tail, visited));
    // println!("{}: H{:?} T{:?} -> H{:?} T{:?}", line, h, t, head, tail);
    Ok(())
}

fn do_move(delta: Pos, head: &mut Pos, tail: &mut Pos, visited: &mut HashSet<Pos>) {
    *head = add(*head, delta);
    *tail = move_tail(*head, *tail);
    visited.insert(*tail);
}

fn move_tail(head: Pos, tail: Pos) -> Pos {
    match sub(head, tail) {
        // No movement if tail is within 1 unit of head
        (x, y) if x.abs() <= 1 && y.abs() <= 1 => tail,
        // Left-right plane
        (x, 0) if x.abs() == 2 => add(tail, (one(x), 0)),
        // Up-down plane
        (0, y) if y.abs() == 2 => add(tail, (0, one(y))),
        // Diagonal
        (x, y) if x.abs() <= 2 && y.abs() <= 2 => add(tail, (one(x), one(y))),
        _ => unreachable!("head is more than 2 units away"),
    }
}

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

fn sub(a: Pos, b: Pos) -> Pos {
    (a.0 - b.0, a.1 - b.1)
}

fn one(n: i32) -> i32 {
    if n.is_negative() {
        -1
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_tail() {
        assert_eq!(move_tail((0, 0), (-1, -2)), (0, -1));
        assert_eq!(move_tail((0, 0), (-2, -2)), (-1, -1));
        assert_eq!(move_tail((0, 0), (1, 2)), (0, 1));
        assert_eq!(move_tail((0, 0), (2, 2)), (1, 1));
        assert_eq!(move_tail((0, 0), (0, 2)), (0, 1));
        assert_eq!(move_tail((0, 0), (2, 0)), (1, 0));
    }
}
