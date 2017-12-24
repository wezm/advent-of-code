use std::collections::HashMap;

fn main() {
    println!("{}", manhattan_distance(289326));
    println!("{}", stress_test(289326));
}

fn spiral_to(index: usize) -> (isize, isize) {
    let mut x = 0isize;
    let mut y = 0isize;
    let mut edge = 1;
    let mut delta = 1isize;
    let mut i = 1usize;

    loop {
        // Horizontal
        for _i in 0..edge {
            x += delta;
            i += 1;
            if i == index { return (x, y) }
        }

        // Vertical
        for _i in 0..edge {
            y += delta;
            i += 1;
            if i == index { return (x, y) }
        }

        edge += 1;
        delta *= -1;
    }
}

fn sum_neighbors(x: isize, y: isize, data: &HashMap<(isize, isize), usize>) -> usize {
    let mut sum = 0;

    for dx in -1..2 {
        for dy in -1..2 {
            if dx == 0 && dy == 0 { continue }

            if let Some(value) = data.get(&(x + dx, y + dy)) {
                sum += value;
            }
        }
    }

    sum
}

fn stress_test(input: usize) -> usize {
    if input == 1 { return 2 }

    let mut data = HashMap::new();
    data.insert((0, 0), 1);

    let mut x = 0isize;
    let mut y = 0isize;
    let mut edge = 1;
    let mut delta = 1isize;

    loop {
        // Horizontal
        for _i in 0..edge {
            x += delta;
            let value = sum_neighbors(x, y, &data);
            if value > input { return value }
            data.insert((x, y), value);
        }

        // Vertical
        for _i in 0..edge {
            y += delta;
            let value = sum_neighbors(x, y, &data);
            if value > input { return value }
            data.insert((x, y), value);
        }

        edge += 1;
        delta *= -1;
    }
}

fn manhattan_distance(index: usize) -> usize {
    if index == 1 { return 0 }

    let (x, y) = spiral_to(index);

    (x.abs() + y.abs()) as usize
}

// Data from square 1 is carried 0 steps, since it's at the access port.
#[test]
fn test_example1() {
    assert_eq!(manhattan_distance(1), 0);
}

// Data from square 12 is carried 3 steps, such as: down, left, left.
#[test]
fn test_example2() {
    assert_eq!(manhattan_distance(12), 3);
}

// Data from square 23 is carried only 2 steps: up twice.
#[test]
fn test_example3() {
    assert_eq!(manhattan_distance(23), 2);
}

// Data from square 1024 must be carried 31 steps.
#[test]
fn test_example4() {
    assert_eq!(manhattan_distance(1024), 31);
}

/*---*/

#[test]
fn test_example5() {
    assert_eq!(manhattan_distance(46), 3);
}

#[test]
fn test_example6() {
    assert_eq!(manhattan_distance(11), 2);
}

#[test]
fn test_example7() {
    assert_eq!(manhattan_distance(10), 3);
}

#[test]
fn test_example8() {
    assert_eq!(manhattan_distance(9), 2);
}

#[test]
fn test_example9() {
    assert_eq!(manhattan_distance(49), 6);
}

#[test]
fn test_example10() {
    assert_eq!(manhattan_distance(28), 3);
}

#[test]
fn test_example11() {
    assert_eq!(manhattan_distance(1089), 32);
}

#[test]
fn test_example12() {
    assert_eq!(manhattan_distance(37), 6);
}

#[test]
fn test_example1_part2() {
    assert_eq!(stress_test(23), 25);
}
