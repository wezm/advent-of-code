use std::{fs, io};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() -> io::Result<()> {
    let data = fs::read_to_string("input/day8.txt")?;
    let image = data.trim().chars().collect::<Vec<_>>();

    let layers = image.chunks(WIDTH * HEIGHT).collect::<Vec<_>>();
    let layer = layers
        .iter()
        .min_by_key(|layer| count_digits(layer, '0'))
        .unwrap();

    let ones = count_digits(layer, '1');
    let twos = count_digits(layer, '2');

    println!("Part 1: {}", ones * twos);

    Ok(())
}

fn count_digits(layer: &[char], digit: char) -> usize {
    layer.iter().filter(|&&c| c == digit).count()
}
