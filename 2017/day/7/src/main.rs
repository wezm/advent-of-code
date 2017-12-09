use std::io::Read;
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    let mut file = File::open("input").expect("unable to open input file");
    file.read_to_string(&mut input).expect("error reading input");

    println!("{}", bottom_program(&input));
}

fn bottom_program(input: &str) -> String {
    // Bottom one must be:
    // - "holding other towers"
    // - Have no other towers pointing at it
    let mut programs = HashSet::new();
    let mut referenced = HashSet::new();

    for line in input.lines() {
        let parts = line.split(" -> ").collect::<Vec<_>>();
        let name_and_weight = parts[0].split_whitespace().collect::<Vec<_>>();
        let name = name_and_weight[0];
        let _weight = name_and_weight[1];

        if let Some(holding) = parts.get(1) {
            for child in holding.split(", ") {
                referenced.insert(child);
            }
        }

        programs.insert(name);
    }

    let candidates = programs.difference(&referenced).collect::<Vec<_>>();
    if candidates.len() != 1 {
        panic!("more than one candidate result");
    }

    (*candidates[0]).to_owned()
}

#[test]
fn test_example() {
    let input = r"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    assert_eq!(bottom_program(input), "tknk".to_owned());
}
