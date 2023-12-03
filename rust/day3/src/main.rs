use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> std::io::Result<()> {
    let input_path = std::env::args().nth(1).expect("FAILED: no path provided");
    let p = Path::new(&input_path);

    if !p.exists() {
        panic!("invalid file path received");
    }

    let ext = p.extension();
    if ext.is_none() || ext.unwrap() != "txt" {
        panic!("invalid file path received");
    }

    let file = File::open(p)?;

    let grid = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap_or(String::new()))
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    part1(&grid);
    part2(&grid);

    Ok(())
}

fn part1(grid: &Vec<Vec<char>>) {
    let mut sum = 0;
    let mut num = String::new();
    let mut start = usize::max_value();
    for (i, line) in grid.iter().enumerate() {
        println!("line {i}");
        for (j, char) in line.iter().enumerate() {
            if char.is_digit(10) {
                if start == usize::max_value() {
                    start = j;
                }

                num.push(*char);
            } else if !num.is_empty() {
                if is_adjacent_to_symbol(i, start, num.len(), grid, None).is_some() {
                    sum += num.parse::<u32>().expect("unable to parse num");
                }
                num.clear();
                start = usize::max_value();
            }
        }

        if !num.is_empty() {
            if is_adjacent_to_symbol(i, start, num.len(), grid, None).is_some() {
                sum += num.parse::<u32>().expect("unable to parse num");
            }

            num.clear();
            start = usize::max_value();
        }
    }

    println!("sum of engine parts is {sum}");
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Point(usize, usize);

fn part2(grid: &Vec<Vec<char>>) {
    let mut gears = HashMap::new();

    let mut num = String::new();
    let mut start = usize::max_value();
    for (i, line) in grid.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if char.is_digit(10) {
                if start == usize::max_value() {
                    start = j;
                }

                num.push(*char);
            } else if !num.is_empty() {
                if let Some(point) = is_adjacent_to_symbol(i, start, num.len(), grid, Some(&'*')) {
                    gears
                        .entry(point)
                        .or_insert(vec![])
                        .push(num.parse::<u32>().expect("unable to parse num"));
                }
                num.clear();
                start = usize::max_value();
            }
        }

        if !num.is_empty() {
            if let Some(point) = is_adjacent_to_symbol(i, start, num.len(), grid, Some(&'*')) {
                gears
                    .entry(point)
                    .or_insert(vec![])
                    .push(num.parse::<u32>().expect("unable to parse num"));
            }

            num.clear();
            start = usize::max_value();
        }
    }

    let sum = gears
        .into_values()
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().copied().reduce(|acc, e| acc * e).unwrap_or(0))
        .sum::<u32>();

    println!("sum of gear ratios is {sum}");
}

fn is_adjacent_to_symbol(
    line: usize,
    column: usize,
    len: usize,
    grid: &Vec<Vec<char>>,
    symbol: Option<&char>,
) -> Option<Point> {
    let min_col = if column > 0 { column - 1 } else { column };
    let cur_line = grid.get(line).unwrap();
    if is_symbol(min_col, cur_line, symbol) {
        return Some(Point(min_col, line));
    }

    if column + len < cur_line.len() - 1 {
        if is_symbol(column + len, cur_line, symbol) {
            return Some(Point(column + len, line));
        }
    }

    if line > 0 {
        let prev_line = grid.get(line - 1);
        if let Some(prev_line) = prev_line {
            for i in min_col..=(prev_line.len() - 1).min(column + len) {
                if is_symbol(i, prev_line, symbol) {
                    return Some(Point(i, line - 1));
                }
            }
        }
    }

    let next_line = grid.get(line + 1);
    if let Some(next_line) = next_line {
        for i in min_col..=(next_line.len() - 1).min(column + len) {
            if is_symbol(i, next_line, symbol) {
                return Some(Point(i, line + 1));
            }
        }
    }

    return None;
}

fn is_symbol(index: usize, line: &Vec<char>, symbol: Option<&char>) -> bool {
    let char = line.get(index).unwrap();
    if symbol.is_some() {
        return char == symbol.unwrap();
    }

    return char.ne(&'.') && !char.is_digit(10) && !char.is_whitespace();
}
