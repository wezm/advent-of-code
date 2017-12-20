extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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


    fn tick(&mut self) {
        // Increase the X velocity by the X acceleration.
        // Increase the Y velocity by the Y acceleration.
        // Increase the Z velocity by the Z acceleration.
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        self.velocity.z += self.acceleration.z;

        // Increase the X position by the X velocity.
        // Increase the Y position by the Y velocity.
        // Increase the Z position by the Z velocity.
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn distance(&self) -> usize {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs()) as usize
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
        particles.push(particle);
    }

    println!("{:?}", closest_to_zero(&particles));
}

fn closest_to_zero(particles: &Vec<Particle>) -> Particle {
    particles[0].clone()
}

#[test]
fn test_example() {

}
