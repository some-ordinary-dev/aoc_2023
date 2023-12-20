use std::fmt::{Display, Write};

use crate::utils::Grid2D;

pub fn solve(content: String) {
    part1(&content);
    part2(&content);
}

fn part1(content: &String) {
    let grid = parse_entries(&content);
    let starting_point = LightSource::new(usize::MAX, 0, Direction::Right);
    let total_energized_entries = process_grid(starting_point, &grid);
    println!("Count of energized entries: {total_energized_entries}");
}

fn part2(content: &String) {
    let grid = parse_entries(&content);

    let mut total_counts = vec![];
    for x in 0..grid.width() {
        let starting_point = LightSource::new(x, usize::MAX, Direction::Down);
        let total_energized_entries = process_grid(starting_point, &grid);
        total_counts.push(total_energized_entries);
    }

    for x in 0..grid.width() {
        let starting_point = LightSource::new(x, usize::MAX, Direction::Up);
        let total_energized_entries = process_grid(starting_point, &grid);
        total_counts.push(total_energized_entries);
    }

    for y in 0..grid.height() {
        let starting_point = LightSource::new(usize::MAX, y, Direction::Right);
        let total_energized_entries = process_grid(starting_point, &grid);
        total_counts.push(total_energized_entries);
    }

    for y in 0..grid.height() {
        let starting_point = LightSource::new(usize::MAX, y, Direction::Left);
        let total_energized_entries = process_grid(starting_point, &grid);
        total_counts.push(total_energized_entries);
    }

    let max_count = total_counts.iter().max().unwrap();
    println!("Starting point that results in maximum energized entries is: {max_count}");
}

fn process_grid(starting_source: LightSource, grid: &Grid2D<Entry>) -> usize {
    let mut grid = grid.clone();
    let mut sources = vec![starting_source];
    while sources.len() > 0 {
        // move each source one step
        let mut broken_streams = vec![];
        let mut new_streams = vec![];
        for (idx, source) in sources.iter_mut().enumerate() {
            if source.can_step(grid.width(), grid.height()) {
                source.step(grid.width(), grid.height());

                if let Some(entry) = grid.get_mut(source.x, source.y) {
                    if entry.seen_dirs.contains(&source.dir) {
                        broken_streams.push(idx);
                    } else {
                        entry.seen_dirs.push(source.dir.clone());
                    }

                    entry.is_energized = true;

                    match entry.t {
                        EntryType::Mirror(mirror_type) => match mirror_type {
                            '\\' => match source.dir {
                                Direction::Left => source.dir = Direction::Up,
                                Direction::Right => source.dir = Direction::Down,
                                Direction::Up => source.dir = Direction::Left,
                                Direction::Down => source.dir = Direction::Right,
                            },
                            '/' => match source.dir {
                                Direction::Left => source.dir = Direction::Down,
                                Direction::Right => source.dir = Direction::Up,
                                Direction::Up => source.dir = Direction::Right,
                                Direction::Down => source.dir = Direction::Left,
                            },
                            _ => {}
                        },
                        EntryType::Splitter(split_type) => {
                            match split_type {
                                '-' if source.dir == Direction::Up
                                    || source.dir == Direction::Down =>
                                {
                                    source.dir = Direction::Left;
                                    new_streams.push(LightSource::new(
                                        source.x,
                                        source.y,
                                        Direction::Right,
                                    ));
                                }
                                '|' if source.dir == Direction::Right
                                    || source.dir == Direction::Left =>
                                {
                                    source.dir = Direction::Up;
                                    new_streams.push(LightSource::new(
                                        source.x,
                                        source.y,
                                        Direction::Down,
                                    ));
                                }
                                _ => {}
                            };
                        }
                        EntryType::Empty => {}
                    }
                }
            } else {
                broken_streams.push(idx);
            }
        }

        for idx in broken_streams.iter().rev() {
            sources.remove(*idx);
        }

        sources.extend(new_streams);
    }

    let total_energized_entries = (0..grid.height()).fold(0, |acc, y| {
        acc + grid.row_iterator(y).filter(|x| x.is_energized).count()
    });

    return total_energized_entries;
}

fn parse_entries(content: &String) -> Grid2D<Entry> {
    let rows = content
        .lines()
        .map(|l| l.chars().map(|c| Entry::new(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Grid2D::new(rows)
}

#[derive(Clone)]
struct Entry {
    t: EntryType,
    is_energized: bool,
    seen_dirs: Vec<Direction>,
}

impl Entry {
    fn new(ch: char) -> Self {
        let t = match ch {
            '.' => EntryType::Empty,
            '|' | '-' => EntryType::Splitter(ch),
            '/' | '\\' => EntryType::Mirror(ch),
            _ => panic!("invalid char received when parsing entry"),
        };

        Self {
            t,
            is_energized: false,
            seen_dirs: vec![],
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.t {
            EntryType::Mirror(ch) => f.write_char(ch),
            EntryType::Splitter(ch) => f.write_char(ch),
            EntryType::Empty => f.write_char('.'),
        }
    }
}

#[derive(Clone)]
enum EntryType {
    Mirror(char),
    Splitter(char),
    Empty,
}

#[derive(Debug)]
struct LightSource {
    x: usize,
    y: usize,
    dir: Direction,
}

impl LightSource {
    fn new(x: usize, y: usize, dir: Direction) -> Self {
        Self { x, y, dir }
    }

    fn can_step(&self, max_x: usize, max_y: usize) -> bool {
        if self.x == usize::MAX || self.y == usize::MAX {
            return true;
        }

        match self.dir {
            Direction::Up => self.y > 0,
            Direction::Down => self.y < max_y,
            Direction::Left => self.x > 0,
            Direction::Right => self.x < max_x,
        }
    }

    fn step(&mut self, max_x: usize, max_y: usize) {
        match (self.dir, self.x, self.y) {
            (Direction::Up, _, usize::MAX) => self.y = max_y - 1,
            (Direction::Up, _, _) => self.y -= 1,
            (Direction::Down, _, usize::MAX) => self.y = 0,
            (Direction::Down, _, _) => self.y += 1,
            (Direction::Left, usize::MAX, _) => self.x = max_x - 1,
            (Direction::Left, _, _) => self.x -= 1,
            (Direction::Right, usize::MAX, _) => self.x = 0,
            (Direction::Right, _, _) => self.x += 1,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
