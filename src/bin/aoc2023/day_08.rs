use std::collections::BTreeMap;

use crate::Runner;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, multispace1},
    combinator::eof,
    multi::{fold_many1, many1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};
use num_integer::gcd;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Default)]
pub struct Day08 {
    input: String,
}

impl Day08 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn solution01(&self) -> usize {
        let (input, (instructions, map)) = self.parser().expect("should validly parse");

        debug_assert_eq!(input, "");

        let mut current_node = "AAA";
        let Some(step_count) =
            instructions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(index, instruction)| {
                    let options = map.get(current_node).expect("always exist at a valid node");
                    let next_node = match instruction {
                        Direction::Left => options.0,
                        Direction::Right => options.1,
                    };
                    if next_node == "ZZZ" {
                        Some(index + 1)
                    } else {
                        current_node = next_node;
                        None
                    }
                })
        else {
            panic!("infinite iterator can't produce None")
        };

        step_count
    }

    pub fn solution02(&self) -> usize {
        let (input, (instructions, map)) = self.parser().expect("should validly parse");

        debug_assert_eq!(input, "");

        let starting_nodes: Vec<&str> = map
            .keys()
            .filter(|key| key.ends_with("A"))
            .cloned()
            .collect();

        let results = starting_nodes
            .iter()
            .map(|node| {
                let mut visited_nodes = vec![*node];
                let mut current_node = *node;
                // cycle is magically "the first Z",
                // and other assorted conditions due
                // to how the input is constructed.
                instructions
                    .iter()
                    .cycle()
                    .enumerate()
                    .find_map(|(index, instruction)| {
                        let options = map.get(current_node).expect("always exist at a valid node");
                        let next_node = match instruction {
                            Direction::Left => options.0,
                            Direction::Right => options.1,
                        };
                        if next_node.ends_with("Z") {
                            Some(index + 1)
                        } else {
                            visited_nodes.push(next_node);
                            current_node = next_node;
                            None
                        }
                    })
                    .expect("should find a cycle")
            })
            .collect::<Vec<usize>>();

        let mut lcm = 1;

        for &num in &results {
            lcm = lcm * num / gcd(lcm, num);
        }

        lcm
    }

    fn parser(&self) -> IResult<&str, (Vec<Direction>, BTreeMap<&str, (&str, &str)>)> {
        let input = self.input.as_str();
        let (input, instructions) = many1(alt((
            complete::char('R').map(|_| Direction::Right),
            complete::char('L').map(|_| Direction::Left),
        )))(input)?;
        let (input, _) = multispace1(input)?;
        let (input, map) = fold_many1(
            terminated(
                separated_pair(
                    alpha1,
                    tag(" = "),
                    delimited(
                        complete::char('('),
                        separated_pair(alpha1, tag(", "), alpha1),
                        complete::char(')'),
                    ),
                ),
                alt((line_ending, eof)),
            ),
            BTreeMap::new,
            |mut acc: BTreeMap<&str, (&str, &str)>, (key, value)| {
                acc.insert(key, value);
                acc
            },
        )(input)?;

        Ok((input, (instructions, map)))
    }
}

impl Runner for Day08 {
    fn name(&self) -> (usize, usize) {
        (2023, 7)
    }

    fn parse(&mut self) {
        let file = include_str!("../../../input/2023-08-01.txt");
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

    #[test]
    fn test_part01() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let mut day = Day08::new();
        day.input = input.to_string();

        let output = day.solution01();

        assert_eq!(6, output);
    }

    #[test]
    fn test_part02() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let mut day = Day08::new();
        day.input = input.to_string();

        let output = day.solution02();

        assert_eq!(6, output);
    }
}
