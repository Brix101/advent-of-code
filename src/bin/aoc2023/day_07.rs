use crate::Runner;
use itertools::{Itertools, Position};
use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Default)]
pub struct Day07 {
    input: String,
}

impl Day07 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn solution01(&self) -> usize {
        use HandType::*;
        let hands = self
            .input
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(" ").unwrap();

                let counts = hand.chars().counts();
                let values = counts.values().sorted().join("");
                let hand_type = match values.deref() {
                    "5" => FiveOfAKind,
                    "14" => FourOfAKind,
                    "23" => FullHouse,
                    "113" => ThreeOfAKind,
                    "122" => TwoPair,
                    "1112" => OnePair,
                    "11111" => HighCard,
                    value => panic!("should never happen. Encountered `{}`", value),
                };
                let card_scores: (u32, u32, u32, u32, u32) = hand
                    .chars()
                    .map(|card| match card {
                        'A' => 14,
                        'K' => 13,
                        'Q' => 12,
                        'J' => 11,
                        'T' => 10,
                        value => value.to_digit(10).unwrap(),
                    })
                    .collect_tuple()
                    .unwrap();
                let hand_score = (hand_type, card_scores);
                (hand, bid.parse::<usize>().unwrap(), hand_score)
            })
            .sorted_by_key(|x| (x.2 .0 as u8, x.2 .1))
            .enumerate()
            .map(|(index, (_hand, bid, _))| (index as usize + 1) * bid)
            .sum::<usize>();

        hands
    }

    pub fn solution02(&self) -> usize {
        use HandType::*;
        let hands = self
            .input
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(" ").unwrap();

                let counts = hand.chars().counts();

                let values = if let Some(joker_count) = counts.get(&'J') {
                    if *joker_count == 5 {
                        "5".to_string()
                    } else {
                        counts
                            .iter()
                            .filter_map(|(key, value)| (key != &'J').then_some(value))
                            .sorted()
                            .with_position()
                            .map(|(position, value)| match position {
                                Position::Last | Position::Only => value + joker_count,
                                _ => *value,
                            })
                            .join("")
                    }
                } else {
                    counts.values().sorted().join("")
                };

                let hand_type = match values.deref() {
                    "5" => FiveOfAKind,
                    "14" => FourOfAKind,
                    "23" => FullHouse,
                    "113" => ThreeOfAKind,
                    "122" => TwoPair,
                    "1112" => OnePair,
                    "11111" => HighCard,
                    value => panic!("should never happen. Encountered `{}`", value),
                };
                let card_scores: (u32, u32, u32, u32, u32) = hand
                    .chars()
                    .map(|card| match card {
                        'A' => 14,
                        'K' => 13,
                        'Q' => 12,
                        'J' => 1,
                        'T' => 10,
                        value => value.to_digit(10).unwrap(),
                    })
                    .collect_tuple()
                    .unwrap();
                let hand_score = (hand_type, card_scores);

                (hand, bid.parse::<usize>().unwrap(), hand_score)
            })
            .sorted_by_key(|x| (x.2 .0 as u8, x.2 .1))
            .enumerate()
            .map(|(index, (_hand, bid, _))| (index as usize + 1) * bid)
            .sum::<usize>();
        hands
    }
}

impl Runner for Day07 {
    fn name(&self) -> (usize, usize) {
        (2023, 7)
    }

    fn parse(&mut self) {
        let file = include_str!("../../../input/2023-07-01.txt");
        self.input = file.to_string();
    }

    fn part1(&mut self) -> Vec<String> {
        let output = self.solution01();
        crate::output(output)
    }

    fn part2(&mut self) -> Vec<String> {
        let output = self.solution02();
        crate::output(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day() -> Day07 {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let mut day = Day07::new();
        day.input = input.to_string();

        day
    }

    #[test]
    fn test_part01() {
        let output = day().solution01();

        assert_eq!(6440, output);
    }

    #[test]
    fn test_part02() {
        let output = day().solution02();

        assert_eq!(5905, output);
    }
}
