use std::io::Read;
use std::fs::File;
use std::str::FromStr;
use std::ops::Range;

fn main() {
    let mut buffer = String::new();
    let mut file = File::open("input").expect("unable to open input file");
    file.read_to_string(&mut buffer).expect("error reading input");

    let lengths = buffer.split(",")
        .map(|chr| usize::from_str(chr.trim()).expect("invalid number"))
        .collect::<Vec<_>>();

    let mut hash = KnotHash::new(lengths);
    println!("{}", hash.hash(0..256));
}

// Not sure this struct is actually adding value...
struct KnotHash {
    current_position: usize,
    skip_size: usize,
    lengths: Vec<usize>,
}

impl KnotHash {
    pub fn new(lengths: Vec<usize>) -> Self {
        KnotHash {
            current_position: 0,
            skip_size: 0,
            lengths: lengths,
        }
    }

    pub fn hash(&mut self, list: Range<usize>) -> u16 {
        let mut list: Vec<usize> = list.collect();

        for &length in self.lengths.iter() {
            self.reverse(list.as_mut_slice(), length);
            if length <= list.len() {
                self.current_position = (self.current_position + length + self.skip_size) % list.len();
            }
            self.skip_size += 1;
        }

        list[0] as u16 * list[1] as u16
    }

    fn reverse(&self, list: &mut [usize], length: usize) {
        if length == 1 { return };

        let list_len = list.len();

        let mut slice = Vec::new();
        for i in 0..length {
            slice.push(list[(self.current_position + length - 1 - i) % list_len]);

        }

        for i in 0..length {
            list[(self.current_position + i) % list_len] = slice[i];
        }
    }
}

#[test]
fn test_knot_hash() {
    let mut hash = KnotHash::new(vec![3, 4, 1, 5]);
    assert_eq!(hash.hash(0..5), 12);
}
