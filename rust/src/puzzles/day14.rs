use std::collections::HashMap;

use crate::utils::Grid2D;

pub fn solve(content: String) {
    part1(&content);
    part2(&content);
}

fn part1(content: &String) {
    let mut platform = Grid2D::from_lines(content.lines());
    tilt_platform(&mut platform, CompassDirection::North);
    let total_load = calculate_load(&platform);
    println!("Part 1 | total load: {total_load}");
}

const TOTAL_ITERATIONS: usize = 1_000_000_000;

fn part2(content: &String) {
    let mut platform = Grid2D::from_lines(content.lines());
    let mut map_states = HashMap::new();

    let mut cycle_start = 0;
    let mut cycle_end = 0;

    for i in 0..TOTAL_ITERATIONS {
        tilt_platform(&mut platform, CompassDirection::North);
        tilt_platform(&mut platform, CompassDirection::West);
        tilt_platform(&mut platform, CompassDirection::South);
        tilt_platform(&mut platform, CompassDirection::East);

        // if we've seen this map state already then break and calculate final position using the
        // cached states
        if let Some((_, v)) = map_states.get_key_value(&platform.to_string()) {
            cycle_start = *v;
            cycle_end = i;
            println!("Cycle detected: {cycle_start} - {cycle_end}");
            break;
        } else {
            // cache map state
            map_states.insert(platform.to_string(), i);
        }
    }

    let remaining_iterations = TOTAL_ITERATIONS - cycle_end;
    let remaining_iterations = remaining_iterations % (cycle_end - cycle_start);
    //  walk through the loop for the remaining required iterations.
    //  could be faster if we changed map_states to a vec, and then we wouldn't have to recompute
    //  any map state, but this works fine
    for _ in 0..remaining_iterations - 1 {
        tilt_platform(&mut platform, CompassDirection::North);
        tilt_platform(&mut platform, CompassDirection::West);
        tilt_platform(&mut platform, CompassDirection::South);
        tilt_platform(&mut platform, CompassDirection::East);
    }

    let total_load = calculate_load(&platform);
    println!("Part 2 | total load: {total_load}");
}

fn calculate_load(platform: &Grid2D<char>) -> usize {
    println!("{platform}");
    (0..platform.height())
        .map(|y| {
            let row_num = platform.height() - y;
            let num_round_rocks = platform.row_iterator(y).filter(|&x| x == &'O').count();
            num_round_rocks * row_num
        })
        .sum::<usize>()
}

fn tilt_platform(platform: &mut Grid2D<char>, direction: CompassDirection) {
    match direction {
        CompassDirection::North => {
            for y in 1..platform.height() {
                for x in 0..platform.width() {
                    do_shit(platform, x, y, &direction);
                }
            }
        }
        CompassDirection::South => {
            for y in (0..platform.height() - 1).rev() {
                for x in 0..platform.width() {
                    do_shit(platform, x, y, &direction);
                }
            }
        }
        CompassDirection::East => {
            for x in (0..platform.width() - 1).rev() {
                for y in 0..platform.height() {
                    do_shit(platform, x, y, &direction);
                }
            }
        }
        CompassDirection::West => {
            for x in 1..platform.width() {
                for y in 0..platform.height() {
                    do_shit(platform, x, y, &direction);
                }
            }
        }
    }
}

fn do_shit(platform: &mut Grid2D<char>, x: usize, y: usize, direction: &CompassDirection) {
    let item = platform.get(x, y).unwrap();
    if item == &'O' {
        // enter while loop while we look at ancestors
        let old_pos = Point { x, y };
        let mut pos = Point { x, y };
        while can_decrement_pos(&pos, &platform, direction) {
            decrement_pos(&mut pos, direction);
            let prev_entry = platform.get(pos.x, pos.y).unwrap();
            if prev_entry != &'.' {
                // roll back last iteration
                increment_pos(&mut pos, direction);
                // break once we've hit an unmovable object
                break;
            }
        }

        if pos != old_pos {
            platform.replace('O', pos.x, pos.y);
            platform.replace('.', old_pos.x, old_pos.y);
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn decrement_pos(pos: &mut Point, direction: &CompassDirection) {
    match *direction {
        CompassDirection::North => pos.y -= 1,
        CompassDirection::South => pos.y += 1,
        CompassDirection::East => pos.x += 1,
        CompassDirection::West => pos.x -= 1,
    }
}

fn can_decrement_pos<T>(pos: &Point, platform: &Grid2D<T>, direction: &CompassDirection) -> bool {
    match *direction {
        CompassDirection::North => pos.y > 0,
        CompassDirection::South => pos.y < platform.height() - 1,
        CompassDirection::East => pos.x < platform.width() - 1,
        CompassDirection::West => pos.x > 0,
    }
}

fn increment_pos(pos: &mut Point, direction: &CompassDirection) {
    match *direction {
        CompassDirection::North => pos.y += 1,
        CompassDirection::South => pos.y -= 1,
        CompassDirection::East => pos.x -= 1,
        CompassDirection::West => pos.x += 1,
    }
}

#[derive(PartialEq, Eq, Debug)]
enum CompassDirection {
    North,
    South,
    East,
    West,
}
