use std::collections::HashSet;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args_os()
        .skip(1)
        .next()
        .expect("missing input path");
    let input = fs::read_to_string(&path)?;

    let mut points = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let (fst, mine) = line.split_once(" | ").ok_or("expected |")?;
        let (fst, winning) = fst.split_once(": ").ok_or("expected :")?;
        let (_, card) = fst.split_once(' ').ok_or("expected ' '")?;
        let mine = mine
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse())
            .collect::<Result<HashSet<u32>, _>>()?;
        let winning = winning
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse())
            .collect::<Result<HashSet<u32>, _>>()?;

        let count: u32 = winning.intersection(&mine).count().try_into().unwrap();
        if count > 0 {
            points.push(2_u64.pow(count - 1));
        }
    }

    let total_points: u64 = points.iter().sum();
    println!("Part 1: {}", total_points);

    Ok(())
}
