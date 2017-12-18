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

    println!("{}", reachable(&programs));
}

fn reachable(list: &Vec<Vec<usize>>) -> usize {
    let mut visited = HashSet::new();

    visit(0, &mut visited, list);

    visited.len()
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
