use std::collections::HashMap;
use std::{fs, io};

#[derive(Debug)]
struct Material<'a> {
    name: &'a str,
    amount: usize,
}

#[derive(Debug)]
struct Reaction<'a> {
    inputs: Vec<Material<'a>>,
    output: Material<'a>,
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/day14.txt")?;
    let reactions = parse_input(&input);

    println!("Day 1 part 1: {}", ore_required(&reactions));

    Ok(())
}

fn parse_input(input: &str) -> Vec<Reaction> {
    input
        .lines()
        .map(|line| {
            let mut io = line.trim().split(" => ");
            let input = io.next().unwrap();
            let output = io.next().map(parse_material).unwrap();
            let inputs = input.split(", ").map(parse_material).collect();

            Reaction { inputs, output }
        })
        .collect()
}

fn parse_material(s: &str) -> Material {
    let mut parts = s.split_whitespace();
    Material {
        amount: parts.next().and_then(|amount| amount.parse().ok()).unwrap(),
        name: parts.next().unwrap(),
    }
}

fn ore_required(reactions: &[Reaction]) -> usize {
    // Find FUEL and work back to ORE to find paths?
    let map = reactions
        .iter()
        .map(|reaction| (reaction.output.name, reaction))
        .collect::<HashMap<_, _>>();

    let mut inventory = HashMap::new();
    run_reaction("FUEL", &map, &mut inventory);
    inventory.get("ORE").copied().unwrap_or_default().abs() as usize
}

// Run the named reaction, updating the inventory
fn run_reaction<'a>(
    name: &'a str,
    tree: &'a HashMap<&'a str, &'a Reaction>,
    inventory: &mut HashMap<&'a str, isize>,
) {
    let node = tree[name];
    node.inputs.iter().for_each(|input| {
        if input.name == "ORE" {
            *inventory.entry("ORE").or_default() -= input.amount as isize;
        } else {
            *inventory.entry(input.name).or_default() -= input.amount as isize;
            while inventory.get(input.name).copied().unwrap_or_default() < 0 {
                run_reaction(input.name, tree, inventory);
            }
        }
    });
    *inventory.entry(name).or_default() += node.output.amount as isize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL
";
        let reactions = parse_input(&input);

        assert_eq!(ore_required(&reactions), 165);
    }
}
