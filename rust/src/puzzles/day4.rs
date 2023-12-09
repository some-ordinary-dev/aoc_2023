use std::collections::HashMap;

pub fn solve(content: String) {
    let mut cards = content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Card::new(line))
        .collect::<Vec<_>>();

    cards.sort_by(|a, b| a.id.cmp(&b.id));

    part1(&cards);
    part2(cards);
}

fn part1(cards: &Vec<Card>) {
    let total_score = cards.iter().map(|card| card.score()).sum::<i64>();
    println!("total score {total_score}");
}

fn part2(cards: Vec<Card>) {
    let mut accumulation_by_card_id = cards
        .iter()
        .map(|card| (card.id, 1))
        .collect::<HashMap<_, u64>>();

    for card in cards.iter() {
        let match_count = card.match_count() as u32;
        let accumulated = accumulation_by_card_id.get(&card.id).unwrap_or(&0).clone();
        println!(
            "card ID: {0}, matches: {match_count}, accumulated: {accumulated}",
            card.id
        );
        for offset in 1..=match_count {
            *accumulation_by_card_id.entry(card.id + offset).or_insert(0) += accumulated;
        }
    }

    let winning_card_count = accumulation_by_card_id.into_values().sum::<u64>();

    println!("winning card count: {winning_card_count}");
}

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    scratch_numbers: Vec<u32>,
}

impl Card {
    fn new(line: &str) -> Self {
        let mut parts = line.split(':');

        let id = parts
            .nth(0)
            .expect("couldn't parse card title")
            .split(' ')
            .filter(|s| !s.is_empty())
            .nth(1)
            .expect("couldn't find space to split on in card title");

        let id = id
            .trim()
            .parse::<u32>()
            .expect("unable to parse card number as u32");

        let mut nums = parts
            .nth(0)
            .expect("couldn't parse numbers from line")
            .split('|');

        let winning_numbers = nums
            .nth(0)
            .expect("couldn't get winning numbers from line")
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| {
                n.trim()
                    .parse::<u32>()
                    .expect("couldn't parse number as u32")
            })
            .collect::<Vec<_>>();

        let scratch_numbers = nums
            .nth(0)
            .expect("couldn't get our numbers from line")
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| {
                n.trim()
                    .parse::<u32>()
                    .expect("couldn't parse number as u32")
            })
            .collect::<Vec<_>>();

        Card {
            id,
            winning_numbers,
            scratch_numbers,
        }
    }

    fn score(&self) -> i64 {
        let match_count = self.match_count();
        let score = if match_count == 0 {
            0
        } else {
            2_i64.pow((match_count - 1).try_into().unwrap())
        };

        return score;
    }

    fn match_count(&self) -> usize {
        self.scratch_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
    }
}
