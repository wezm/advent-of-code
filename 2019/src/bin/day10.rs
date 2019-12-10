use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::PI;
use std::{fs, io};

type Angle = isize;
type Distance = isize;

const PRECISION: f64 = 1000.;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Asteroid {
    x: usize,
    y: usize,
}

fn main() -> io::Result<()> {
    let data = fs::read_to_string("input/day10.txt")?;
    let map = data
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim().chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Asteroid { x, y })
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    // From a given asteroid find the angle and distance to every other asteroid
    // only keep the closest one for each angle.
    let maxsteroid = map
        .iter()
        .map(|&asteroid| {
            let stats = angles_and_distances(asteroid, &map);
            stats
                .iter()
                .map(|(_angle, distances)| distances.iter().next().unwrap())
                .count()
        })
        .max()
        .unwrap();
    println!("Part 1: {}", maxsteroid);

    Ok(())
}

fn angles_and_distances(
    from: Asteroid,
    asteroids: &[Asteroid],
) -> BTreeMap<Angle, BTreeSet<(Distance, Asteroid)>> {
    asteroids
        .iter()
        .fold(BTreeMap::new(), |mut result, &asteroid| {
            let angle = from.angle_to(asteroid);
            let distance = from.distance_to(asteroid);
            result
                .entry(angle)
                .or_insert(BTreeSet::new())
                .insert((distance, asteroid));
            result
        })
}

impl Asteroid {
    // https://en.wikipedia.org/wiki/Atan2#Definition_and_computation
    fn angle_to(&self, other: Asteroid) -> Angle {
        let x = other.y as f64 - self.y as f64;
        let y = other.x as f64 - self.x as f64;

        let angle = if x > 0. {
            (y / x).atan()
        } else if y > 0. {
            (PI / 2.) - (x / y).atan()
        } else if y < 0. {
            -(PI / 2.) - (x / y).atan()
        } else if x < 0. {
            (y / x).atan() + PI
        } else {
            // undefined but for our purposes 0 will do
            0.
        } * PRECISION;

        angle.round() as Angle
    }

    fn distance_to(&self, other: Asteroid) -> Distance {
        (((self.x as f64 - other.x as f64).powf(2.) + (self.y as f64 - other.y as f64).powf(2.))
            .sqrt()
            * PRECISION)
            .round() as Distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let this = Asteroid { x: 3, y: 4 };
        let other = Asteroid { x: 1, y: 0 };

        assert_eq!(this.distance_to(other), 4472);
    }

    #[test]
    fn test_angle() {
        let this = Asteroid { x: 0, y: 0 };
        let other = Asteroid { x: 0, y: 9 };
        assert_eq!(this.angle_to(other), 0);

        let this = Asteroid { x: 0, y: 0 };
        let other = Asteroid { x: 9, y: 0 };
        assert_eq!(
            this.angle_to(other),
            ((PI / 2.) * PRECISION).round() as Angle
        );

        let this = Asteroid { x: 3, y: 4 };
        let other = Asteroid { x: 1, y: 0 };
        assert_eq!(this.angle_to(other), -2678);
    }
}
