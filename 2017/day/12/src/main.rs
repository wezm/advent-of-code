use std::io::Read;
use std::fs::File;
use std::str::FromStr;
use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();
    let mut file = File::open("input").expect("unable to open input file");
    file.read_to_string(&mut buffer).expect("error reading input");

    let mut programs = Vec::new();

    for line in buffer.lines() {
        // 1 <-> 66, 1682
        let parts: Vec<&str> = line.split(" <-> ").collect();
        let _pid = usize::from_str(parts[0]).unwrap();
        let linked: Vec<usize> = parts[1].split(", ").map(|id| usize::from_str(id).unwrap()).collect();

        // Assumes input is a sequential list of program ids with no gaps
        programs.push(linked);
    }

    println!("{}", reachable(0, &programs).len());
    println!("{}", groups(&programs));
}

// This isn't real pretty but it works
fn groups(list: &Vec<Vec<usize>>) -> usize {
    let mut groups = 0;
    let mut to_visit = HashSet::with_capacity(list.len());

    for idx in 0..list.len() {
        to_visit.insert(idx);
    }

    while !to_visit.is_empty() {
        let group = reachable(*to_visit.iter().next().unwrap(), list);
        let mut new_to_visit = HashSet::new();
        for idx in to_visit.difference(&group) {
            new_to_visit.insert(*idx);
        }
        to_visit = new_to_visit;
        groups += 1
    }

    groups
}

fn reachable(from: usize, list: &Vec<Vec<usize>>) -> HashSet<usize> {
    let mut visited = HashSet::new();

    visit(from, &mut visited, list);

    visited
}

fn visit(pid: usize, visited: &mut HashSet<usize>, list: &Vec<Vec<usize>>) {
    if visited.contains(&pid) {
        return
    }

    visited.insert(pid);
    for child in list[pid].iter() {
        visit(*child, visited, list);
    }
}
