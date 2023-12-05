use std::collections::HashMap;
use std::ops::Range;
use std::{env, fs};

type BoxError = Box<dyn std::error::Error>;

#[derive(Debug)]
struct Map {
    source: String,
    dest: String,
    mappings: Vec<MapRange>,
}

#[derive(Debug)]
struct MapRange {
    destination_range_start: usize,
    source_range_start: usize,
    length: usize,
}

fn main() -> Result<(), BoxError> {
    let input_path = env::args_os()
        .skip(1)
        .next()
        .ok_or("missing input file path")?;
    let input = fs::read_to_string(input_path)?;
    let mut chunks = input.split("\n\n");
    let seeds_text = chunks.next().ok_or("missing seeds")?;
    if !seeds_text.starts_with("seeds: ") {
        return Err("seeds text does not start with expected prefix".into());
    }
    let seeds = seeds_text
        .split_ascii_whitespace()
        .skip(1)
        .map(|text| text.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;
    let mut maps = HashMap::new();
    for map_text in chunks {
        let map = read_map(map_text)?;
        maps.insert(map.source.clone(), map);
    }

    // chase the seed ids through the maps
    let locations = seeds
        .iter()
        .copied()
        .map(|seed| {
            let mut map = &maps["seed"];
            let mut id = seed;
            while map.dest != "location" {
                id = map.lookup(id);
                map = &maps[&map.dest];
            }
            map.lookup(id)
        })
        .collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        locations.iter().min().ok_or("no min location")?
    );

    Ok(())
}

fn read_map(input: &str) -> Result<Map, BoxError> {
    let mut lines = input.lines();
    let title = lines.next().ok_or("map missing title")?;
    let (from_to, _) = title.split_once(' ').ok_or("unable to split title")?;
    let mut words = from_to.split('-');
    let source = words.next().ok_or("title missing source")?;
    let _to = words.next();
    let destination = words.next().ok_or("title missing destination")?;

    let mut mappings = Vec::new();
    for line in lines {
        match line
            .split_ascii_whitespace()
            .map(|part| part.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?
            .as_slice()
        {
            &[destination_range_start, source_range_start, length] => mappings.push(MapRange {
                destination_range_start,
                source_range_start,
                length,
            }),
            _ => return Err(format!("unable to parse '{}'", line).into()),
        }
    }

    Ok(Map {
        source: source.to_owned(),
        dest: destination.to_owned(),
        mappings,
    })
}

impl Map {
    fn lookup(&self, id: usize) -> usize {
        self.mappings
            .iter()
            .find_map(|range| range.map(id))
            .unwrap_or(id)
    }
}

impl MapRange {
    fn map(&self, id: usize) -> Option<usize> {
        self.source_range().contains(&id).then(|| {
            let offset = id - self.source_range_start;
            self.destination_range_start + offset
        })
    }

    fn source_range(&self) -> Range<usize> {
        self.source_range_start..self.source_range_start + self.length
    }
}
