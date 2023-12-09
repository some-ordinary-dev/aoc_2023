use itertools::Itertools;
use std::{fmt::Display, str::FromStr};

pub fn solve(content: String) {
    part1(&content);
    part2(&content);
}

fn part1(content: &str) {
    let maps = parse_maps(content);
    let seeds = parse_seeds(content);

    for map in maps.iter() {
        println!("{:?} - {:?}", map.from, map.to);
    }

    let min_location = seeds
        .iter()
        .map(|s| {
            let mut map_type = MapType::Seed;
            let mut entry_num = *s;
            while map_type != MapType::Location {
                let map = maps
                    .iter()
                    .filter(|m| m.from == map_type)
                    .take(1)
                    .map(|m| m.clone())
                    .collect::<Vec<_>>();

                let map = map
                    .get(0)
                    .expect(&format!("Unable to find map of from type {:?}", map_type));

                entry_num = map.get_to_entry(entry_num);
                map_type = map.to.clone();

                println!("seed: {s}, type: {:?}, entry: {entry_num}", map_type);
            }

            return entry_num;
        })
        .min()
        .unwrap_or(usize::MAX);

    println!("Min location {min_location}");
}

fn part2(content: &str) {
    let maps = parse_maps(content);
    let seeds = parse_seeds(content);

    for map in maps.iter() {
        println!("{:?} - {:?}", map.from, map.to);
    }

    let seed_ranges = seeds
        .iter()
        .enumerate()
        .group_by(|&(i, _)| i / 2)
        .into_iter()
        .map(|(_, group)| {
            let parts = group.map(|pair| pair.1).collect::<Vec<_>>();
            if parts.len() != 2 {
                panic!("Unable to parse seed range");
            }

            let start = parts[0];
            let len = parts[1];
            Range::new(*start, *len)
        })
        .collect::<Vec<_>>();

    let min_location = seed_ranges
        .iter()
        .map(|range| {
            println!("seed: {range}");
            let mut map_type = MapType::Seed;
            let mut ranges = vec![range.clone()];

            while map_type != MapType::Location {
                let map = maps
                    .iter()
                    .filter(|m| m.from == map_type)
                    .take(1)
                    .map(|m| m.clone())
                    .collect::<Vec<_>>();

                let map = map
                    .get(0)
                    .expect(&format!("Unable to find map of from type {:?}", map_type));

                // println!("{:?} -> {:?}:", map_type, map.to);
                // ranges.sort_by(|a, b| a.start.cmp(&b.start));
                // for r in ranges.iter() {
                //     println!("\t{r}");
                // }
                //
                // println!("------------------------------------>");

                ranges = map.get_to_ranges(&ranges);
                map_type = map.to.clone();

                // ranges.sort_by(|a, b| a.start.cmp(&b.start));
                //
                // for r in ranges.iter() {
                //     println!("\t{r}");
                // }
                //
                // println!("<------------------------------------");
            }

            let min_location_of_range = ranges.iter().map(|r| r.start).min().unwrap_or(usize::MAX);
            println!("Min Location {min_location_of_range}");
            return min_location_of_range;
        })
        .min()
        .unwrap_or(usize::MAX);

    println!("Min location {min_location}");
}

fn parse_seeds(content: &str) -> Vec<usize> {
    content
        .lines()
        .nth(0)
        .expect("unable to parse seeds from top of content")
        .split(' ')
        .map(|s| usize::from_str(s))
        .filter(|u| u.is_ok())
        .map(|u| u.unwrap())
        .collect::<Vec<_>>()
}

fn parse_maps(content: &str) -> Vec<Map> {
    let mut maps = vec![];
    let mut cur_map: Option<Map> = None;
    for line in content.lines().skip(1) {
        if line.is_empty() {
            if cur_map.is_some() {
                maps.push(cur_map.unwrap().clone());
                cur_map = None;
            }
        } else if let Some(ref mut map) = cur_map {
            let parts = line
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| usize::from_str(s))
                .filter(|u| u.is_ok())
                .map(|u| u.unwrap())
                .collect::<Vec<_>>();

            if parts.len() != 3 {
                panic!("Unable to parse entry from line: {line}");
            }

            map.entries
                .push(MapEntry::new(parts[1], parts[0], parts[2]));
        } else {
            let map_types = line
                .split(' ')
                .nth(0)
                .unwrap()
                .split('-')
                .collect::<Vec<_>>();

            if map_types.len() != 3 {
                panic!("Unable to parse map types {line}");
            }

            let from_type = MapType::from_str(map_types[0]).unwrap();
            let to_type = MapType::from_str(map_types[2]).unwrap();

            cur_map = Some(Map::new(from_type, to_type));
        }
    }

    if cur_map.is_some() {
        maps.push(cur_map.unwrap().clone());
    }

    return maps;
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MapType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug, PartialEq, Eq)]
struct MapTypeParseErr;

impl FromStr for MapType {
    type Err = MapTypeParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed" => Ok(Self::Seed),
            "soil" => Ok(Self::Soil),
            "fertilizer" => Ok(Self::Fertilizer),
            "water" => Ok(Self::Water),
            "light" => Ok(Self::Light),
            "temperature" => Ok(Self::Temperature),
            "humidity" => Ok(Self::Humidity),
            "location" => Ok(Self::Location),
            _ => Err(MapTypeParseErr),
        }
    }
}

struct RangeResult {
    overlap: Range,
    orphan_before: Option<Range>,
    orphan_after: Option<Range>,
}

impl RangeResult {
    fn new(overlap: Range, orphan_before: Option<Range>, orphan_after: Option<Range>) -> Self {
        Self {
            overlap,
            orphan_before,
            orphan_after,
        }
    }
}

#[derive(Clone)]
struct MapEntry {
    from_range: Range,
    to_range: Range,
}

impl MapEntry {
    fn new(from_start: usize, to_start: usize, len: usize) -> Self {
        MapEntry {
            from_range: Range::new(from_start, len),
            to_range: Range::new(to_start, len),
        }
    }

    fn get_to_entry(&self, from_entry: usize) -> Option<usize> {
        if Range::new(from_entry, 0).is_in(&self.from_range) {
            // println!("{from_entry} - {}", self.from_range.start);
            let offset = from_entry - self.from_range.start;
            return Some(self.to_range.start + offset);
        }

        None
    }

    fn get_range_overlaps(&self, from_range: &Range) -> Option<RangeResult> {
        if from_range.overlaps(&self.from_range) {
            // clamp the start and end points of the from range
            let max_start = self.from_range.start.max(from_range.start);
            let min_end = self.from_range.end.min(from_range.end);
            let len = min_end - max_start;

            let offset_from_start = max_start - self.from_range.start;
            let to_range = Range::new(self.to_range.start + offset_from_start, len);


            // from_range start is before this range's start
            let orphan_before = if max_start != from_range.start {
                Some(Range::new(
                    from_range.start,
                    max_start - from_range.start - 1,
                ))
            } else {
                None
            };

            // from_range end extends past this range's end
            let orphan_after = if min_end != from_range.end {
                let new_start = min_end + 1;
                let new_len = from_range.end - min_end - 1;
                Some(Range::new(new_start, new_len))
            } else {
                None
            };

            // println!("{to_range} | {:?} | {:?}", orphan_before, orphan_after);

            return Some(RangeResult::new(to_range, orphan_before, orphan_after));
        }

        None
    }
}

#[derive(Clone)]
struct Map {
    from: MapType,
    to: MapType,
    entries: Vec<MapEntry>,
}

impl Map {
    fn new(from: MapType, to: MapType) -> Self {
        Map {
            from,
            to,
            entries: vec![],
        }
    }

    fn get_to_entry(&self, from_entry: usize) -> usize {
        let valid_entries = self
            .entries
            .iter()
            .map(|e| e.get_to_entry(from_entry))
            .filter(|e| e.is_some())
            .take(1);

        for e in valid_entries {
            return e.unwrap();
        }

        return from_entry;
    }

    fn get_to_ranges(&self, from_ranges: &Vec<Range>) -> Vec<Range> {
        let mut ranges = vec![];
        let mut stack = from_ranges.clone();
        while stack.len() > 0 {
            let range_to_process = stack.pop().unwrap();
            let mut has_some = false;
            for entry in self
                .entries
                .iter()
                .sorted_by(|a, b| a.from_range.start.cmp(&b.from_range.start))
            {
                if let Some(overlap_result) = entry.get_range_overlaps(&range_to_process) {
                    if let Some(orphan_before) = overlap_result.orphan_before {
                        stack.push(orphan_before.clone());
                    }

                    ranges.push(overlap_result.overlap.clone());

                    if let Some(orphan_after) = overlap_result.orphan_after {
                        stack.push(orphan_after.clone());
                    }

                    has_some = true;
                    break;
                }
            }

            if !has_some {
                // println!("Couldn't find match for {range_to_process}");
                ranges.push(range_to_process.clone());
            }
        }

        return ranges;
    }
}

#[derive(Clone, Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}-{}", self.start, self.end))
    }
}

impl Range {
    fn new(start: usize, len: usize) -> Self {
        Range {
            start,
            // len is inclusive of start... doh!
            end: start + len - 1,
        }
    }

    fn is_in(&self, other: &Self) -> bool {
        other.start <= self.start && self.start <= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.is_in(other) || other.is_in(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_parses_orphan_before() {
        let map_entry = MapEntry::new(10, 20, 5);
        let range = Range::new(5, 10);
        let result = map_entry.get_range_overlaps(&range);

        assert!(result.is_some());
        let result = result.unwrap();

        let orphan_before = result.orphan_before;
        assert!(orphan_before.is_some());

        let orphan_before = orphan_before.unwrap();
        assert_eq!(5, orphan_before.start);
        assert_eq!(8, orphan_before.end);

        assert!(result.orphan_after.is_none());
    }

    #[test]
    fn it_parses_orphan_after() {
        let map_entry = MapEntry::new(10, 20, 5);
        let range = Range::new(12, 10);
        let result = map_entry.get_range_overlaps(&range);

        assert!(result.is_some());
        let result = result.unwrap();

        let orphan_after = result.orphan_after;
        assert!(orphan_after.is_some());

        let orphan_after = orphan_after.unwrap();
        assert_eq!(15, orphan_after.start);
        assert_eq!(20, orphan_after.end);

        assert!(result.orphan_before.is_none());
    }
}
