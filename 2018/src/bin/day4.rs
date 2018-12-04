#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate regex;

use chrono::{DateTime, TimeZone, Utc};
use regex::Regex;
use std::fs;

#[derive(Debug)]
enum Action {
    WakeUp,
    BeginShift(u32),
    FallAsleep,
}

#[derive(Debug)]
struct Entry {
    time: DateTime<Utc>,
    action: Action,
}

fn main() {
    let input = fs::read_to_string("input/2018/day4.txt").expect("input");

    let entries = {
        let mut entries = input
            .lines()
            .map(parse_entry)
            .collect::<Option<Vec<_>>>()
            .expect("error parsing input");
        entries.sort_unstable_by(|a, b| a.time.cmp(&b.time));
        entries
    };

    part1(&entries);
}

fn part1(entries: &[Entry]) {
    for entry in entries {
        println!("{:?}", entry);
    }
}

fn parse_entry(line: &str) -> Option<Entry> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"\A\[([^\]]+)\] (.+)\z"#).unwrap();
        static ref SHIFT: Regex = Regex::new(r#"\AGuard #(\d+) begins shift\z"#).unwrap();
    }
    let captures = RE.captures(line)?;
    let time = Utc.datetime_from_str(&captures[1], "%Y-%m-%d %H:%M").ok()?;

    match &captures[2] {
        "wakes up" => Some(Entry::new(time, Action::WakeUp)),
        "falls asleep" => Some(Entry::new(time, Action::FallAsleep)),
        other => {
            let captures = SHIFT.captures(other)?;
            Some(Entry::new(
                time,
                Action::BeginShift(captures[1].parse().ok()?),
            ))
        }
    }
}

impl Entry {
    fn new(time: DateTime<Utc>, action: Action) -> Self {
        Self { time, action }
    }
}
