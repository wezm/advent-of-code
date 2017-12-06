use std::f64::consts::PI;

// There might me a more elegant solution to this problem that I've missed (it was completed
// without any Internet access). At least it runs in constant time and space.
fn main() {
    println!("{}", manhattan_distance(289326));
}

// Determine the dimensions of the rectangle in the spiral that the given number must be on. Will
// always be an odd number as 1 is in the middle.
fn spiral_diameter(index: i32) -> i32 {
    let sqrt = (index as f64).sqrt().ceil() as i32;
    if sqrt % 2 == 0 { sqrt + 1 } else { sqrt }
}

// Returns angle in radians of number on spiral
fn angle(index: i32) -> f64 {
    if index == 1 { return 0. }

    let diameter = spiral_diameter(index);

    let area_of_inner_rectangle = (diameter - 2).pow(2);
    let squares_around_rectangle = diameter.pow(2) - area_of_inner_rectangle;
    let squares_per_side = squares_around_rectangle / 4;

    // Divide the space around the rectangle into squares_around_rectangle segments
    let angle_per_segment = 2. * PI / squares_around_rectangle as f64;

    // Determine how far around the rectangle this index is.
    // Offset adjusts for the last number being at the bottom right corner of any
    // given rectangle.
    let offset = squares_per_side / 2;
    let angle = (index - area_of_inner_rectangle - offset) as f64 * angle_per_segment;

    angle
}

fn manhattan_distance(index: i32) -> i32 {
    if index == 1 { return 0 }

    let angle = angle(index);
    let radius = (spiral_diameter(index) / 2) as f64;

    // Calculate the x and y coordinates, scale by √2 so that sin/cos at corners is 1
    let horizontal_distance = 1f64.min((2f64.sqrt() * angle.cos()).abs());
    let vertical_distance = 1f64.min((2f64.sqrt() * angle.sin()).abs());

    let distance = radius * (horizontal_distance + vertical_distance);

    distance.round() as i32
}

#[test]
fn test_angle_should_be_zero() {
    assert_eq!(angle(1), 0.);
    assert_eq!(angle(2), 0.);
    assert_eq!(angle(11), 0.);
    assert_eq!(angle(28), 0.);
}

#[test]
fn test_angle_should_be_half_pi() {
    use std::f64::consts::FRAC_PI_2;

    assert_eq!(angle(4), FRAC_PI_2);
    assert_eq!(angle(15), FRAC_PI_2);
    assert_eq!(angle(34), FRAC_PI_2);
}

#[test]
fn test_angle_should_be_1_point_75_pi() {
    assert_eq!(angle(9), 1.75 * PI);
    assert_eq!(angle(25), 1.75 * PI);
    assert_eq!(angle(49), 1.75 * PI);
    assert_eq!(angle(1089), 1.75 * PI);
}

#[test]
fn test_angle_37() {
    assert_eq!(angle(37), 0.75 * PI);
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

