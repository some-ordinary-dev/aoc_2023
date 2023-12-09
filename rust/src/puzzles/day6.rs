pub fn solve(content: String) {
    let games = parse_games(content.clone());

    part1(&games);
    part2(content);
}

fn part1(games: &Vec<Game>) {
    let winner_mult = games
        .iter()
        .map(|g| g.winning_method_count())
        .filter(|x| x > &0)
        .reduce(|acc, x| acc * x)
        .unwrap_or(usize::MAX);

    println!("Part1 | Winning method multiplier: {winner_mult}");
}

fn part2(content: String) {
    let mut lines = content.lines();
    let timing = lines
        .next()
        .expect("Unable to parse timings from content")
        .split(' ')
        .filter(|s| !s.is_empty())
        .filter(|s| s.parse::<u32>().is_ok())
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .expect("Unable to join timings into single time");

    let record_distance = lines
        .next()
        .expect("Unable to parse distances from content")
        .split(' ')
        .filter(|s| !s.is_empty())
        .filter(|s| s.parse::<u32>().is_ok())
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .expect("Unable to join distances into a single distance");

    let game = Game::new(timing, record_distance);
    let winning_method_count = game.winning_method_count();

    println!("Part2 | Winning method multiplier: {winning_method_count}");
}

fn parse_games(content: String) -> Vec<Game> {
    let mut lines = content.lines();
    let timings = lines
        .next()
        .expect("Unable to parse timings from content")
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>())
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap());

    let record_distances = lines
        .next()
        .expect("Unable to parse distances from content")
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>())
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap())
        .collect::<Vec<_>>();

    let games = timings
        .enumerate()
        .map(|(i, time)| {
            let record = record_distances
                .get(i)
                .expect(&format!("Couldn't find record for timing ID {i}"));

            Game::new(time, *record)
        })
        .collect::<Vec<_>>();

    return games;
}

struct Game {
    time: usize,
    record_distance: usize,
}

impl Game {
    fn new(time: usize, record_distance: usize) -> Self {
        Game {
            time,
            record_distance,
        }
    }

    fn winning_method_count(&self) -> usize {
        let mut winning_count: usize = 0;
        // only process up to half the entries because the winning matches will follow a bell curve
        for i in (1..=self.time / 2).rev() {
            let remaining_time = self.time - i;
            let distance = i * remaining_time;
            if distance > self.record_distance {
                let inverse = self.time - i;
                winning_count += if i == inverse {
                    // only add one if we're at the halfway point
                    1
                } else {
                    // add another entry for the inverse entry
                    2
                };
            } else {
                // once we hit one match that doesn't win we can early return
                break;
            }
        }
        return winning_count;
    }
}

#[cfg(test)]
mod tests {
    use crate::day6::*;
    use std::fs;

    #[test]
    fn it_passes_test() {
        let content = fs::read_to_string("data/test/day6.txt").unwrap();
        solve(content);
    }
}
