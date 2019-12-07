use std::{fs, io};

#[derive(Debug, Eq, PartialEq)]
struct Orbit<'a>(&'a str, &'a str);

#[derive(Debug, Eq, PartialEq)]
struct Node<'a> {
    name: &'a str,
    parent: Option<usize>, // Only COM should have None at the end
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/day6.txt")?;
    let orbits = parse_input(&input);

    println!("Part 1: {}", number_of_orbits(&orbits));

    Ok(())
}

fn parse_input(input: &str) -> Vec<Orbit<'_>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.trim().split(')');
            Orbit(parts.next().unwrap(), parts.next().unwrap())
        })
        .collect()
}

fn number_of_orbits(input: &[Orbit<'_>]) -> usize {
    // Build the tree from the input
    let mut nodes: Vec<Node<'_>> = Vec::with_capacity(input.len());

    // Except for the universal Center of Mass (COM), every object in space is in orbit around
    // exactly one other object.
    for orbit in input {
        // Try to find the parent of this object
        let parent_index = nodes
            .iter()
            .position(|node| node.name == orbit.0)
            .unwrap_or_else(|| {
                nodes.push(Node {
                    name: orbit.0,
                    parent: None,
                });
                nodes.len() - 1
            });

        // See if this object is already in nodes. If it is, then ensure its parent is
        // set, otherwise insert it.
        nodes
            .iter_mut()
            .find(|node| node.name == orbit.1)
            .map(|node| {
                if node.parent.is_none() {
                    node.parent = Some(parent_index);
                }
            })
            .unwrap_or_else(|| {
                nodes.push(Node {
                    name: orbit.1,
                    parent: Some(parent_index),
                });
            });
    }

    // Now traverse the node and sum the orbits of each one
    (0..nodes.len())
        .map(|index| count_orbits(&nodes, index, 0))
        .sum()
}

fn count_orbits(nodes: &[Node], node_index: usize, count: usize) -> usize {
    let node = &nodes[node_index];
    if let Some(parent) = node.parent {
        count_orbits(nodes, parent, count + 1)
    } else {
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "B)C
COM)B
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";

    #[test]
    fn test_number_of_orbits() {
        let orbits = parse_input(TEST_INPUT);
        assert_eq!(number_of_orbits(&orbits), 42);
    }
}
