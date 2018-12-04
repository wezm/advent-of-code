#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs;

const FABRIC_SIZE: usize = 1000;

#[derive(Clone)]
struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[derive(Clone)]
struct Claim {
    id: u32,
    rect: Rect,
}

fn main() {
    let input = fs::read_to_string("input/2018/day3.txt").expect("input");

    let claims = input
        .lines()
        .map(parse_claim)
        .collect::<Option<Vec<_>>>()
        .expect("input error");

    let fabric = claim_counts(&claims);

    part1(&fabric);
    part2(&fabric);
}

fn part1<'a>(input: &[Vec<&'a Claim>]) {
    let over_two_claims = input.into_iter().filter(|claims| claims.len() >= 2).count();

    println!("{}", over_two_claims);
}

fn part2<'a>(input: &[Vec<&'a Claim>]) {
    let non_overlapping_claim = input
        .into_iter()
        .filter_map(|claims| {
            if claims.len() == 1 {
                Some(claims[0])
            } else {
                None
            }
        }).find(|candidate| winner(input, candidate))
        .expect("did not find non-overlapping claim");

    println!("{}", non_overlapping_claim.id);
}

fn parse_claim(line: &str) -> Option<Claim> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"\A#(\d+) @ (\d+),(\d+): (\d+)x(\d+)\z"#).unwrap();
    }
    let captures = RE.captures(line)?;
    let id = captures[1].parse().ok()?;
    let x = captures[2].parse().ok()?;
    let y = captures[3].parse().ok()?;
    let width = captures[4].parse().ok()?;
    let height = captures[5].parse().ok()?;

    Some(Claim {
        id,
        rect: Rect {
            x,
            y,
            width,
            height,
        },
    })
}

fn claim_counts<'a>(claims: &'a [Claim]) -> Vec<Vec<&'a Claim>> {
    let mut claim_counts = vec![Vec::new(); FABRIC_SIZE * FABRIC_SIZE];

    for claim in claims {
        for y in claim.rect.y..claim.rect.max_y() {
            for x in claim.rect.x..claim.rect.max_x() {
                let existing = claim_counts
                    .get_mut(y as usize * FABRIC_SIZE + x as usize)
                    .unwrap();
                existing.push(claim)
            }
        }
    }

    claim_counts
}

fn winner<'a>(claims: &[Vec<&'a Claim>], candidate: &Claim) -> bool {
    for y in candidate.rect.y..candidate.rect.max_y() {
        for x in candidate.rect.x..candidate.rect.max_x() {
            if claims[y as usize * FABRIC_SIZE + x as usize].len() != 1 {
                return false;
            }
        }
    }

    true
}

impl Rect {
    fn max_x(&self) -> u32 {
        self.x + self.width
    }

    fn max_y(&self) -> u32 {
        self.y + self.height
    }
}
