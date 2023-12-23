use std::fmt::Write;

use crate::utils::Grid2D;

pub fn solve(content: String) {
    part1(&content);
    // part2(&content);
}

fn part1(content: &String) {
    solve_internal(content, false);
}

fn part2(content: &String) {
    solve_internal(content, true);
}

fn solve_internal(content: &String, from_hex: bool) {
    let instructions = parse_instructions(&content, from_hex);
    let max = instructions.iter().map(|x| x.meters).sum::<usize>() / 2;
    let mut grid = Grid2D::with_size(
        max,
        max,
        Entry {
            dir: None,
            filled: false,
        },
    );

    println!("{grid}");
    execute_instructions(&mut grid, &instructions);
    println!("{grid}");
    fill_grid_area(&mut grid);
    let filled_entries = count_filled_entries(&grid);

    println!("{grid}");
    println!("Total filled entries: {filled_entries}");
}

fn execute_instructions(grid: &mut Grid2D<Entry>, instructions: &Vec<Instruction>) {
    let mut pos = (grid.width() / 2, grid.height() / 2);
    for (idx, i) in instructions.iter().enumerate() {
        let (cur_x, cur_y) = pos;
        let (mut next_x, mut next_y) = (cur_x, cur_y);

        let next = instructions
            .get(idx + 1)
            .unwrap_or(instructions.first().unwrap());

        match i.dir {
            Direction::Right => {
                next_x += i.meters;

                for x in 1..=i.meters {
                    let e = grid.get_mut(cur_x + x, cur_y).unwrap();
                    e.dir = Some(get_dir(x == i.meters, &Direction::Right, &next.dir));
                    e.filled = true;
                }
            }
            Direction::Left => {
                next_x -= i.meters;

                for x in 1..=i.meters {
                    let e = grid.get_mut(cur_x - x, cur_y).unwrap();
                    e.dir = Some(get_dir(x == i.meters, &Direction::Left, &next.dir));
                    e.filled = true;
                }
            }
            Direction::Down => {
                next_y += i.meters;

                for y in 1..=i.meters {
                    let e = grid.get_mut(cur_x, cur_y + y).unwrap();
                    e.dir = Some(get_dir(y == i.meters, &Direction::Down, &next.dir));
                    e.filled = true;
                }
            }
            Direction::Up => {
                next_y -= i.meters;

                for y in 1..=i.meters {
                    let e = grid.get_mut(cur_x, cur_y - y).unwrap();
                    e.dir = Some(get_dir(y == i.meters, &Direction::Up, &next.dir));
                    e.filled = true;
                }
            }
        }

        pos = (next_x, next_y);
    }
}

fn get_dir(is_final: bool, dir: &Direction, next_dir: &Direction) -> [Direction; 2] {
    if is_final {
        [dir.opposite(), next_dir.clone()]
    } else {
        [dir.clone(), dir.opposite()]
    }
}

fn fill_grid_area(grid: &mut Grid2D<Entry>) {
    for y in 0..grid.height() {
        let mut in_grid = false;
        for x in 0..grid.width() {
            let e = grid.get_mut(x, y).unwrap();
            if let Some(dir) = &e.dir {
                if dir.contains(&Direction::Up) {
                    in_grid = !in_grid;
                }
            } else if in_grid {
                e.filled = true;
            }
        }
    }
}

fn count_filled_entries(grid: &Grid2D<Entry>) -> usize {
    let mut count = 0;

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let e = grid.get(x, y).unwrap();
            if e.filled {
                count += 1;
            }
        }
    }
    return count;
}

fn parse_instructions(content: &String, from_hex: bool) -> Vec<Instruction> {
    content
        .lines()
        .map(|l| Instruction::new(l, from_hex))
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right = 0,
    Left = 1,
    Down = 2,
    Up = 3,
}

impl Direction {
    fn from_char(ch: char) -> Self {
        match ch {
            'R' => Self::Right,
            'L' => Self::Left,
            'U' => Self::Up,
            'D' => Self::Down,
            _ => panic!("Unable to parse direction"),
        }
    }

    fn from_index(idx: usize) -> Self {
        match idx {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => panic!("unable to parse direction"),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
        }
    }
}

struct Instruction {
    dir: Direction,
    meters: usize,
}

impl Instruction {
    fn new(line: &str, from_hex: bool) -> Self {
        let parts = line.split(' ').collect::<Vec<_>>();
        if from_hex {
            let color = &parts[2].to_string()[2..7];
            let dir = &parts[2].to_string()[7..8];
            let meters = usize::from_str_radix(color, 16).unwrap();
            Self {
                dir: Direction::from_index(dir.parse::<usize>().unwrap()),
                meters,
            }
        } else {
            Self {
                dir: Direction::from_char(parts[0].chars().nth(0).unwrap()),
                meters: parts[1].to_string().parse::<usize>().unwrap(),
            }
        }
    }
}

#[derive(Clone)]
struct Entry {
    dir: Option<[Direction; 2]>,
    filled: bool,
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(if self.filled { '#' } else { '.' })
    }
}
