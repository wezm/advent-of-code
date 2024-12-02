use std::env;
use std::path::PathBuf;

mod day1;

fn main() {
    let Some(input) = env::args().skip(1).next().map(PathBuf::from) else {
        eprintln!("Usage advent-of-code dayN.txt");
        return;
    };

    let day = input
        .file_stem()
        .and_then(|stem| stem.to_str())
        .and_then(|stem| stem.strip_prefix("day").unwrap_or(stem).parse::<u8>().ok());
    let Some(day) = day else {
        eprintln!("Unable to determine day from {}", input.display());
        return;
    };

    match day {
        1 => day1::main(&input),
        _ => {
            eprintln!("Unknown day {day}");
            return;
        }
    }
}
