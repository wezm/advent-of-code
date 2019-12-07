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
    println!("Part 2: {}", orbital_transfers(&orbits, "YOU", "SAN"));

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
    let nodes = build_tree(input);
    // Now traverse the node and sum the orbits of each one
    (0..nodes.len())
        .map(|index| count_orbits(&nodes, index, 0))
        .sum()
}

fn orbital_transfers(input: &[Orbit<'_>], from: &str, to: &str) -> usize {
    let nodes = build_tree(input);

    // Find the to and from nodes
    let from = nodes
        .iter()
        .find(|node| node.name == from)
        .expect("unable to find from node");
    let to = nodes
        .iter()
        .find(|node| node.name == to)
        .expect("unable to find to node");

    // Find the longest shared path they have on the way to COM and then the two remaining parts
    // give the path between them.
    let from_nodes_to_com = nodes_to_com(&nodes, from, Vec::new());
    let to_nodes_to_com = nodes_to_com(&nodes, to, Vec::new());
    let mut i = 0;
    while from_nodes_to_com[i].name == to_nodes_to_com[i].name {
        i += 1;
    }

    // - 2 since we don't count the path from 'from' to the object it's orbiting. Same for 'to'.
    from_nodes_to_com[i..].len() + to_nodes_to_com[i..].len() - 2
}

fn build_tree<'a>(input: &'a [Orbit<'a>]) -> Vec<Node<'a>> {
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

    nodes
}

fn count_orbits(nodes: &[Node], node_index: usize, count: usize) -> usize {
    let node = &nodes[node_index];
    if let Some(parent) = node.parent {
        count_orbits(nodes, parent, count + 1)
    } else {
        count
    }
}

fn nodes_to_com<'a>(
    nodes: &'a [Node],
    current_node: &'a Node,
    mut path: Vec<&'a Node<'a>>,
) -> Vec<&'a Node<'a>> {
    path.push(current_node);
    if let Some(parent_index) = current_node.parent {
        nodes_to_com(nodes, &nodes[parent_index], path)
    } else {
        path.reverse();
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "B)C
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

    const TEST_INPUT2: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
";

    #[test]
    fn test_number_of_orbits() {
        let orbits = parse_input(TEST_INPUT1);
        assert_eq!(number_of_orbits(&orbits), 42);
    }

    #[test]
    fn test_orbital_transfers() {
        let orbits = parse_input(TEST_INPUT2);
        assert_eq!(orbital_transfers(&orbits, "YOU", "SAN"), 4);
    }
}
