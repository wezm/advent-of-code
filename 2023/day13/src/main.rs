use std::string::FromUtf8Error;
use std::{env, fs};

type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    let input_path = env::args_os()
        .skip(1)
        .next()
        .ok_or("missing input file path")?;
    let input = fs::read_to_string(&input_path)?;

    let mut vertical = Vec::new();
    let mut horizontal = Vec::new();

    for pattern in input.split("\n\n") {
        if pattern.is_empty() {
            continue;
        }

        println!("{pattern}");
        let lines = pattern.lines().collect::<Vec<_>>();
        if let Some(i) = find_mirror_point(&lines) {
            horizontal.push(i);
            continue;
        }

        // transpose the lines to try to a vertical mirror point
        let transposed = transpose(&lines)?;
        if let Some(i) = find_mirror_point(&transposed) {
            vertical.push(i);
        } else {
            return Err("no vertical or horizontal split".into());
        }
        println!()
    }

    let part1 = vertical.iter().sum::<usize>() + horizontal.iter().map(|h| *h * 100).sum::<usize>();
    println!("Part 1: {part1}");

    Ok(())
}

fn find_mirror_point<T: AsRef<str>>(lines: &[T]) -> Option<usize> {
    let mut prev: Option<&str> = None;
    for (i, line) in lines.iter().enumerate() {
        match prev {
            Some(prev) if prev == line.as_ref() => {
                print!("possible mirror point at {i}: ");
                // split the array in half, see if they're the same
                let (first, second) = lines.split_at(i);
                // ensure they're the same length
                let len = first.len().min(second.len());
                let start = first.len() - len;
                let (first, second) = (&first[start..], &second[..len]);
                if first
                    .iter()
                    .rev()
                    .zip(second.iter())
                    .all(|(l, r)| l.as_ref() == r.as_ref())
                {
                    println!("seems to be mirror point");
                    return Some(i);
                } else {
                    println!("no match");
                }
            }
            Some(_) | None => {}
        }
        prev = Some(line.as_ref())
    }
    None
}

fn transpose(lines: &[&str]) -> Result<Vec<String>, FromUtf8Error> {
    let mut transposed = Vec::new();
    for i in 0..lines[0].len() {
        let mut col = Vec::new();
        for line in lines.iter() {
            col.push(line.as_bytes()[i])
        }
        transposed.push(String::from_utf8(col)?);
    }
    Ok(transposed)
}
