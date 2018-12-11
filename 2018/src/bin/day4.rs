#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate regex;

use chrono::{DateTime, Duration, TimeZone, Timelike, Utc};
use regex::Regex;
use std::collections::HashMap;
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

impl Entry {
    fn new(time: DateTime<Utc>, action: Action) -> Self {
        Self { time, action }
    }
}

// If anyone is reading this, it's not great Rust code...

fn main() {
    let input = fs::read_to_string("input/day4.txt").expect("input");

    let entries = {
        let mut entries = input
            .lines()
            .map(parse_entry)
            .collect::<Option<Vec<_>>>()
            .expect("error parsing input");
        entries.sort_unstable_by(|a, b| a.time.cmp(&b.time));
        entries
    };
    let entries_by_guard = entries_by_guard(&entries);

    part1(&entries_by_guard);
    part2(&entries_by_guard);
}

fn part1(entries_by_guard: &HashMap<u32, Vec<&Entry>>) {
    // Find the one that was asleep the most
    let guards_by_time_asleep = calculate_time_asleep(&entries_by_guard);
    let asleep_the_most = guards_by_time_asleep.iter().max_by(
        |(_guard_id, time_asleep), (_other_guard_id, other_time_asleep)| {
            time_asleep.cmp(other_time_asleep)
        },
    );

    // Now find the minute that guard is asleep the most
    let sleepiest_guard = asleep_the_most.unwrap().0;
    let sleep_by_minute = sleep_by_minute(&entries_by_guard[sleepiest_guard]);
    let sleepiest_minute = sleep_by_minute
        .iter()
        .max_by(|(_minute, count), (_other_minute, other_count)| count.cmp(other_count))
        .unwrap();

    println!("Part 1: {}", sleepiest_minute.0 * sleepiest_guard);
}

fn part2(entries_by_guard: &HashMap<u32, Vec<&Entry>>) {
    let results: HashMap<_, _> = entries_by_guard
        .keys()
        .filter_map(|guard_id| {
            let sleep_by_minute = sleep_by_minute(&entries_by_guard[&guard_id]);

            if sleep_by_minute.is_empty() {
                None
            } else {
                Some((guard_id, sleep_by_minute))
            }
        })
        .collect();

    // Now find the answer
    if let Some((guard_id, minute_counts)) =
        results
            .iter()
            .max_by(|(_, minute_counts), (_, other_minute_counts)| {
                max_minute(&minute_counts)
                    .1
                    .cmp(&max_minute(&other_minute_counts).1)
            })
    {
        let minute = max_minute(minute_counts).0;
        println!(
            "Part 2: guard {} * minute {} = {}",
            *guard_id,
            minute,
            *guard_id * minute
        );
    } else {
        println!("Part 2: No result found");
    }
}

fn entries_by_guard(entries: &[Entry]) -> HashMap<u32, Vec<&Entry>> {
    let mut entries_by_guard = HashMap::new();
    let mut current_guard = None;

    for entry in entries {
        match entry.action {
            Action::BeginShift(guard_id) => {
                let guard_entry = entries_by_guard
                    .entry(guard_id)
                    .or_insert_with(|| Vec::new());
                guard_entry.push(entry);
                current_guard = Some(guard_id);
            }
            Action::FallAsleep => {
                entries_by_guard
                    .get_mut(&current_guard.unwrap())
                    .unwrap()
                    .push(entry);
            }
            Action::WakeUp => {
                entries_by_guard
                    .get_mut(&current_guard.unwrap())
                    .unwrap()
                    .push(entry);
            }
        }
    }

    entries_by_guard
}

fn calculate_time_asleep(entries_by_guard: &HashMap<u32, Vec<&Entry>>) -> HashMap<u32, Duration> {
    let mut time_asleep = HashMap::with_capacity(entries_by_guard.len());

    for (&guard_id, entries) in entries_by_guard {
        let mut fell_asleep = None;

        for entry in entries {
            match entry.action {
                Action::BeginShift(_guard_id) => {}
                Action::FallAsleep => fell_asleep = Some(entry.time),
                Action::WakeUp => {
                    let total_time_asleep = time_asleep
                        .entry(guard_id)
                        .or_insert_with(|| Duration::zero());
                    *total_time_asleep = *total_time_asleep + (entry.time - fell_asleep.unwrap());
                }
            }
        }
    }

    time_asleep
}

fn sleep_by_minute(entries: &[&Entry]) -> HashMap<u32, u32> {
    let mut result = HashMap::new();
    let mut fell_asleep = None;

    for entry in entries {
        match entry.action {
            Action::BeginShift(_guard_id) => {}
            Action::FallAsleep => fell_asleep = Some(entry.time),
            Action::WakeUp => {
                for minute in fell_asleep.unwrap().minute()..entry.time.minute() {
                    let count_asleep = result.entry(minute).or_insert(0);
                    *count_asleep += 1;
                }
            }
        }
    }

    result
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

fn max_minute(hash: &HashMap<u32, u32>) -> (u32, u32) {
    hash
        .iter()
        .max_by(|(_, count), (_, other_count)| count.cmp(other_count))
        .map(|(&minute, &count)| (minute, count))
        .unwrap()
}
