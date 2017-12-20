extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
    z: isize
}

impl Point {
    fn new(x: &str, y: &str, z: &str) -> Point {
        Point {
            x: isize::from_str(x).unwrap(),
            y: isize::from_str(y).unwrap(),
            z: isize::from_str(z).unwrap(),
        }
    }
}

#[derive(Debug)]
struct Particle {
    position: Point,
    velocity: Point,
    acceleration: Point,
}

impl Particle {
    fn new(px: &str, py: &str, pz: &str, vx: &str, vy: &str, vz: &str, ax: &str, ay: &str, az: &str) -> Particle {
        Particle {
            position: Point::new(px, py, pz),
            velocity: Point::new(vx, vy, vz),
            acceleration: Point::new(ax, ay, az),
        }
    }
}


fn main() {

    // p=<-317,1413,1507>, v=<19,-102,-108>, a=<1,-3,-3>
    let re = Regex::new(r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>").unwrap();

    let mut input = String::new();
    let mut file = File::open("input").expect("unable to open input file");
    file.read_to_string(&mut input).expect("error reading input");

    let mut particles = Vec::new();
    for cap in re.captures_iter(&input) {
        let particle = Particle::new(&cap[1], &cap[2], &cap[3], &cap[4], &cap[5], &cap[6], &cap[7], &cap[8], &cap[9]);
        println!("{:?}", particle);
        particles.push(particle);
    }
}

#[test]
fn test_example() {

}
