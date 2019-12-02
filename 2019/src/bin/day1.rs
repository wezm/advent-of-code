use std::io;

use advent_of_code::input;

fn main() -> io::Result<()> {
    // Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
    let result: i32 = input::read_number_list("input/day1.txt")?
        .into_iter()
        .map(|module_mass| (module_mass as f64 / 3.).floor() as i32 - 2)
        .sum();

    println!("Result: {}", result);

    Ok(())
}
