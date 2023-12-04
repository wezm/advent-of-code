use std::collections::HashSet;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args_os()
        .skip(1)
        .next()
        .expect("missing input path");
    let input = fs::read_to_string(&path)?;

    let mut points = Vec::new();

    let card_count = input.trim().lines().count().try_into()?;
    let mut copies = vec![0; card_count];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let (fst, mine) = line.split_once(" | ").ok_or("expected |")?;
        let (fst, winning) = fst.split_once(": ").ok_or("expected :")?;
        let (_, card) = fst.split_once(' ').ok_or("expected ' '")?;
        let card = card
            .trim()
            .parse()
            .ok()
            .and_then(|n: usize| n.checked_sub(1))
            .ok_or("invalid card number")?;
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

        let count = winning.intersection(&mine).count();
        if count > 0 {
            points.push(2_u64.pow((count - 1).try_into()?));
        }

        // part 2 calculations
        // add the current card to the count
        copies[card] += 1;
        let instances = copies[card];

        // add 'count' copies of subsequent cards
        let next_card = card + 1;
        (next_card..next_card + count).for_each(|n| {
            if let Some(entry) = copies.get_mut(n) {
                *entry += instances;
            }
        });
    }

    let total_points: u64 = points.iter().sum();
    let total_cards: u64 = copies.iter().sum();

    println!("Part 1: {}", total_points);
    println!("Part 2: {}", total_cards);

    Ok(())
}
