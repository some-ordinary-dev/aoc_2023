use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(content: String) {
    // part1(&content);
    part2(&content);
}

fn part1(content: &String) {
    let actual_loop = get_loop(content);
    let farthest_step = actual_loop.len() / 2;
    println!("Part1 | The farthest point is {farthest_step} steps away");
}

fn part2(content: &String) {
    let actual_loop = get_loop(content);
    let loop_points = actual_loop
        .iter()
        .map(|entry| (entry.pos, entry))
        .collect::<HashMap<_, _>>();

    // get loop boundaries
    let min_x = actual_loop.iter().map(|entry| entry.pos.0).min().unwrap();
    let max_x = actual_loop.iter().map(|entry| entry.pos.0).max().unwrap();

    let min_y = actual_loop.iter().map(|entry| entry.pos.1).min().unwrap();
    let max_y = actual_loop.iter().map(|entry| entry.pos.1).max().unwrap();

    // generate matrix around loop boundaries
    let mut point_matrix = HashMap::new();

    for y in min_y..=max_y {
        let mut is_inside = false;
        for x in min_x..=max_x {
            let point = Point::new(x, y);
            if let Some(loop_point) = loop_points.get(&point) {
                if loop_point.direction.uses_map_direction(MapDirection::North) {
                    is_inside = !is_inside;
                }
            } else if is_inside {
                *point_matrix.entry(point).or_insert(0_usize) += 1;
            }
        }
    }

    // count entries in the point matrix that have a value of 2 (i.e., were marked as in the border
    // twice)
    let area = point_matrix.into_values().filter(|v| v == &1).count();
    println!("Area enclosed by loop is {area}");
}

fn get_loop(content: &String) -> Vec<Pipe> {
    let map = parse_content(content);
    let starting_point = get_starting_point(&map);
    let mut potential_loops = get_potential_loops(&starting_point, &map);

    let actual_loop;
    loop {
        // get next step for each potential loop
        let mut ids_for_removal = vec![];
        for (i, l) in potential_loops.iter_mut().enumerate() {
            let last = l.last().unwrap();
            // get neighbours
            let neighbors = last.get_neighbors(&map);
            // println!("Last: {:?}", last);
            // println!("Valid Neighbors: {:?}", neighbors);
            // except the previous entry (or starting point)
            let prev_entry = l.iter().rev().skip(1).take(1).copied().collect::<Vec<_>>();

            let prev_entry = prev_entry.get(0).unwrap_or(&starting_point);

            let neighbors = neighbors
                .iter()
                .filter(|n| n.pos != prev_entry.pos)
                .collect_vec();

            if let Some(next) = neighbors.first() {
                // if some then add that entry to the potential loop
                l.push(**next);
            } else {
                // remove this loop
                ids_for_removal.push(i);
            }
        }

        // remove invalid loops
        for i in ids_for_removal {
            potential_loops.remove(i);
        }

        if potential_loops.len() == 0 {
            panic!("All loops failed!");
        }

        // if any potential_loop has finished then break with this as the main loop
        let finished_loop = potential_loops
            .iter()
            .filter(|&l| l.last().unwrap().pos == starting_point.pos)
            .map(|l| l.iter().rev().skip(1).rev().copied().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        if finished_loop.len() > 0 {
            actual_loop = finished_loop.first().unwrap().clone();
            break;
        }

        // if any two loops are connected then combine and break with the combination as the main
        // loop
        let loops_by_end_point = potential_loops
            .iter()
            .map(|p| p.last().unwrap())
            .sorted_by(|&a, &b| a.pos.cmp(&b.pos))
            .group_by(|&a| &a.pos);

        let connected_loops = loops_by_end_point
            .into_iter()
            .map(|(key, group)| (key, group.count()))
            .filter(|(_, count)| count > &1)
            .collect::<Vec<_>>();

        if connected_loops.len() > 0 {
            let connected_loops = potential_loops
                .iter()
                .filter(|&l| l.last().unwrap().pos == *connected_loops[0].0)
                .collect::<Vec<_>>();

            let mut actual = connected_loops[0].clone();
            actual.extend(
                connected_loops[1]
                    .clone()
                    .iter()
                    .skip(1)
                    .rev()
                    .skip(1)
                    .collect_vec(),
            );

            actual_loop = actual;
            break;
        }
    }

    return actual_loop;
}

fn parse_content(content: &String) -> HashMap<Point, Pipe> {
    content
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    '.' => {
                        let p = Point::new(x, y);
                        Pipe::new(p, Direction::None)
                    }
                    _ => {
                        let p = Point::new(x, y);
                        let direction: Direction = ch.into();
                        Pipe::new(p, direction)
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .map(|entry| (entry.pos, entry))
        .collect::<HashMap<_, _>>()
}

fn get_starting_point(map: &HashMap<Point, Pipe>) -> Pipe {
    let mut starting_point = None;
    for entry in map.clone().into_values() {
        if entry.direction == Direction::Unknown {
            starting_point = Some(entry.clone());
        }
    }

    if starting_point.is_none() {
        panic!("Unable to find starting point");
    }

    return starting_point.unwrap();
}

fn get_potential_loops(starting_point: &Pipe, map: &HashMap<Point, Pipe>) -> Vec<Vec<Pipe>> {
    starting_point
        .get_neighbors(map)
        .iter()
        .map(|p| vec![starting_point.clone(), p.clone()])
        .collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Unknown,
    None,
}

enum MapDirection {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn uses_map_direction(&self, dir: MapDirection) -> bool {
        match dir {
            MapDirection::East if self == &Self::NE || self == &Self::SE || self == &Self::EW => {
                true
            }
            MapDirection::West if self == &Self::NW || self == &Self::SW || self == &Self::EW => {
                true
            }
            MapDirection::South if self == &Self::NS || self == &Self::SE || self == &Self::SW => {
                true
            }
            MapDirection::North if self == &Self::NE || self == &Self::NW || self == &Self::NS => {
                true
            }
            _ => self == &Self::Unknown,
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            'S' => Self::Unknown,
            _ => panic!("invalid char received"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Ord, PartialOrd)]
struct Point(usize, usize);

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }
}

#[derive(Debug, Clone, Copy)]
struct Pipe {
    pos: Point,
    direction: Direction,
}

impl Pipe {
    fn new(pos: Point, direction: Direction) -> Self {
        Self { pos, direction }
    }

    fn connects(&self, other: &Self) -> bool {
        if self.pos.1 == other.pos.1 {
            // compare east and west directions
            if self.pos.0 < other.pos.0 {
                // other is to the east of self
                if self.direction.uses_map_direction(MapDirection::East)
                    && other.direction.uses_map_direction(MapDirection::West)
                {
                    return true;
                }
            } else {
                // other is to the west of self
                if self.direction.uses_map_direction(MapDirection::West)
                    && other.direction.uses_map_direction(MapDirection::East)
                {
                    return true;
                }
            }
        }

        if self.pos.0 == other.pos.0 {
            // compare north and south directions
            if self.pos.1 < other.pos.1 {
                // other is to the south of self
                if self.direction.uses_map_direction(MapDirection::South)
                    && other.direction.uses_map_direction(MapDirection::North)
                {
                    return true;
                }
            } else {
                // other is to the north of self
                if self.direction.uses_map_direction(MapDirection::North)
                    && other.direction.uses_map_direction(MapDirection::South)
                {
                    return true;
                }
            }
        }

        return false;
    }

    fn get_neighbors(&self, map: &HashMap<Point, Pipe>) -> Vec<Pipe> {
        let mut neighbors = vec![];
        if self.pos.0 > 0 {
            // must be a West facing pipe
            let point = Point::new(self.pos.0 - 1, self.pos.1);
            if let Some(pipe) = map.get(&point) {
                if self.connects(&pipe) {
                    neighbors.push(pipe.clone());
                }
            }
        }
        if self.pos.1 > 0 {
            // must be a South facing pipe
            let point = Point::new(self.pos.0, self.pos.1 - 1);
            if let Some(pipe) = map.get(&point) {
                if self.connects(&pipe) {
                    neighbors.push(pipe.clone());
                }
            }
        }

        // must be a North facing pipe
        let point = Point::new(self.pos.0, self.pos.1 + 1);
        if let Some(pipe) = map.get(&point) {
            if self.connects(&pipe) {
                neighbors.push(pipe.clone());
            }
        }

        // must be an East facing pipe
        let point = Point::new(self.pos.0 + 1, self.pos.1);
        if let Some(pipe) = map.get(&point) {
            if self.connects(&pipe) {
                neighbors.push(pipe.clone());
            }
        }

        return neighbors;
    }
}
