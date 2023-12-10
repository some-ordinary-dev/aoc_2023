use std::cmp::Ordering;

use itertools::Itertools;

pub fn solve(content: String) {
    let hands = parse_hands(content);

    part1(&hands);
}

fn part1(hands: &Vec<Hand>) {
    let total_score: usize = hands
        .iter()
        .sorted_by(|a, b| a.cmp(&b))
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1))
        .sum();

    println!("Total Score: {total_score}");
}

fn parse_hands(content: String) -> Vec<Hand> {
    content
        .lines()
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<_>>();
            let cards = parts
                .get(0)
                .expect("unable to parse cards from line")
                .chars()
                .map(|c| c.into())
                .collect::<Vec<Card>>();

            let bid = parts
                .get(1)
                .expect("unable to parse bid from line")
                .parse::<usize>()
                .expect("Couldn't parse bid as usize");

            Hand::new(cards, bid)
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Num(u32),
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.get_rank(), other.get_rank()) {
            (a, b) if a < b => std::cmp::Ordering::Less,
            (a, b) if a > b => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            v if v.is_digit(10) && v >= '1' && v <= '9' => Self::Num(v.to_digit(10).unwrap()),
            _ => panic!("Invalid char received when parsing card"),
        }
    }
}

impl Card {
    fn get_rank(&self) -> u32 {
        match self {
            &Self::A => 14,
            &Self::K => 13,
            &Self::Q => 12,
            &Self::T => 10,
            &Self::Num(v) => v,
            &Self::J => 1,
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: usize) -> Self {
        Self { cards, bid }
    }

    fn get_rank(&self) -> u32 {
        let mut match_groups = self
            .cards
            .iter()
            // remove jokers
            .filter(|&c| c != &Card::J)
            .sorted_by(|a, b| a.cmp(b))
            .group_by(|&c| c)
            .into_iter()
            .map(|(_, group)| group.count())
            .collect::<Vec<_>>();

        match_groups.sort_by(|a, b| b.cmp(&a));

        let joker_count = self.cards.iter().filter(|&c| c == &Card::J).count();

        match (match_groups.len(), joker_count) {
            // five of a kind
            (0, jc) | (1, jc) if match_groups.get(0).unwrap_or(&0) + jc == 5 => 7,
            // four of a kind
            (2, jc) if match_groups[0] + jc == 4 => 6,
            // full house
            (2, jc) if match_groups[0] + jc == 3 => 5,
            // three of a kind
            (3, jc) if match_groups[0] + jc == 3 => 4,
            // two pair
            (3, 0) if match_groups[0] == 2 => 3,
            // one pair
            (4, jk) if match_groups[0] + jk == 2 => 2,
            // high card
            _ => 1,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let rank_cmp = self.get_rank().cmp(&other.get_rank());
        if rank_cmp == Ordering::Equal {
            for (i, c) in self.cards.iter().enumerate() {
                let other_card = &other.cards[i];
                let card_cmp = c.cmp(other_card);
                if card_cmp != Ordering::Equal {
                    return card_cmp;
                }
            }
        }
        return rank_cmp;
    }
}
