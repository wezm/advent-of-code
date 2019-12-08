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

    let picture = composite(&layers);

    for i in 0..(WIDTH * HEIGHT) {
        if i % WIDTH == 0 {
            println!();
        }
        let pixel = if picture[i] == '1' { '█' } else { ' ' };
        print!("{}", pixel);
    }
    println!();

    Ok(())
}

fn count_digits(layer: &[char], digit: char) -> usize {
    layer.iter().filter(|&&c| c == digit).count()
}

fn composite(layers: &Vec<&[char]>) -> Vec<char> {
    let mut image = Vec::with_capacity(WIDTH * HEIGHT);
    // For each pixel go top to bottom finding the first non-transparent colour and add that to the
    // final image.
    for i in 0..(WIDTH * HEIGHT) {
        let layer = layers.iter().find(|layer| layer[i] != '2').unwrap();
        image.push(layer[i]);
    }

    image
}
