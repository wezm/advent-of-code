use std::io;

use advent_of_code::input;

fn main() -> io::Result<()> {
    let module_masses = input::read_number_list("input/day1.txt")?;

    // Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
    let fuel: Vec<_> = module_masses
        .into_iter()
        .map(|module_mass| (module_mass as f64 / 3.).floor() as i32 - 2)
        .collect();

    println!("Part 1: {}", fuel.iter().sum::<i32>());
    part2(&fuel);

    Ok(())
}

fn part2(fuel: &[i32]) {
    // Fuel itself requires fuel just like a module - take its mass, divide by three, round down, and subtract 2.
    let result: i32 = fuel
        .iter()
        .map(|&fuel| {
            let mut total = 0;
            let mut extra = fuel_fuel(fuel);
            while extra > 0 {
                total += extra;
                extra = fuel_fuel(extra);
            }
            total
        })
        .sum();

    println!("Part 2: {}", result + fuel.iter().sum::<i32>());
}

fn fuel_fuel(fuel: i32) -> i32 {
    ((fuel as f64 / 3.).floor() as i32).saturating_sub(2)
}
