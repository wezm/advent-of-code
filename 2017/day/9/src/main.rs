use std::fs::File;
use std::io::{Read, BufReader};
use std::char;

#[derive(Debug, Copy, Clone)]
enum Flags {
    InGarbage,
    CancelNext,
    CancelNextInGarbage,
    Normal
}

#[derive(Debug)]
struct State {
    group_depth: usize,
    flags: Flags,
}

fn main() {
    let file = File::open("input").expect("unable to open input file");
    let reader = BufReader::new(file);

    println!("{}", stream(reader));
}

fn stream<R>(reader: R) -> usize where R: Read {
    let mut state = State { group_depth: 0, flags: Flags::Normal };
    let mut score = 0;

    for byte in reader.bytes() {
        state = match (state.flags, byte.ok().and_then(|b| char::from_u32(b as u32))) {
            (Flags::CancelNext, _)          => State { group_depth: state.group_depth, flags: Flags::Normal },
            (Flags::CancelNextInGarbage, _) => State { group_depth: state.group_depth, flags: Flags::InGarbage },
            (Flags::Normal, Some('{'))      => State { group_depth: state.group_depth + 1, flags: Flags::Normal },
            (Flags::Normal, Some('}'))      => {
                score += state.group_depth;
                State { group_depth: state.group_depth - 1, flags: Flags::Normal }
            },
            (Flags::Normal, Some('<'))      => State { group_depth: state.group_depth, flags: Flags::InGarbage },
            (Flags::InGarbage, Some('>'))   => State { group_depth: state.group_depth, flags: Flags::Normal },
            (Flags::InGarbage, Some('!'))   => State { group_depth: state.group_depth, flags: Flags::CancelNextInGarbage },
            (Flags::Normal, Some('!'))      => State { group_depth: state.group_depth, flags: Flags::CancelNext },
            (_, Some(_))                    => state,
            (_, None)                       => panic!("error reading/converting byte")
        };

        // println!("{:?}", state);
    }

    score
}

// {}, score of 1.
#[test]
fn test_empty() {
    use std::io::Cursor;

    let cursor = Cursor::new(b"{}");
    assert_eq!(stream(cursor), 1);
}

// {{{}}}, score of 1 + 2 + 3 = 6.
#[test]
fn test_empty_nested() {
    use std::io::Cursor;

    let cursor = Cursor::new(b"{{{}}}");
    assert_eq!(stream(cursor), 6);
}

// {{},{}}, score of 1 + 2 + 2 = 5.
#[test]
fn test_empty_empty() {
    use std::io::Cursor;

    let cursor = Cursor::new(b"{{},{}}");
    assert_eq!(stream(cursor), 5);
}

// {{{},{},{{}}}}, score of 1 + 2 + 3 + 3 + 3 + 4 = 16.
#[test]
fn test_empty_empty_nested_empty() {
    use std::io::Cursor;

    let cursor = Cursor::new(b"{{{},{},{{}}}}");
    assert_eq!(stream(cursor), 16);
}

// {<a>,<a>,<a>,<a>}, score of 1.
#[test]
fn test_four_garbage() {
    use std::io::Cursor;

    let cursor = Cursor::new(b"{<a>,<a>,<a>,<a>}");
    assert_eq!(stream(cursor), 1);
}

// {{<ab>},{<ab>},{<ab>},{<ab>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
#[test]
fn test_groups_with_garbage() {
    use std::io::Cursor;

    let cursor = Cursor::new(b"{{<ab>},{<ab>},{<ab>},{<ab>}}");
    assert_eq!(stream(cursor), 9);
}

// {{<!!>},{<!!>},{<!!>},{<!!>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
#[test]
fn test_groups_with_cancellations() {
    use std::io::Cursor;

    let cursor = Cursor::new(b"{{<!!>},{<!!>},{<!!>},{<!!>}}");
    assert_eq!(stream(cursor), 9);
}

// {{<a!>},{<a!>},{<a!>},{<ab>}}, score of 1 + 2 = 3.
#[test]
fn test_groups_with_garbage_exclamation() {
    use std::io::Cursor;

    let cursor = Cursor::new(b"{{<a!>},{<a!>},{<a!>},{<ab>}}");
    assert_eq!(stream(cursor), 3);
}


