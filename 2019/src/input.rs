use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_number_list<P: AsRef<Path>>(path: P) -> io::Result<Vec<i32>> {
    let input = BufReader::new(File::open(path)?);
    let mut output = Vec::new();

    for line in input.lines() {
        let line = line?;
        output.push(
            line.parse()
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?,
        )
    }

    Ok(output)
}
