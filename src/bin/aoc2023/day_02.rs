use std::collections::HashMap;

use crate::Runner;

#[derive(Default)]
pub struct Day02 {
    records: Vec<String>,
}

impl Day02 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn solution01(&self) -> usize {
        let set_limit = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
        self.records
            .iter()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(':').collect();
                let game_number = parts[0]
                    .strip_prefix("Game")
                    .unwrap_or("0")
                    .trim()
                    .parse::<usize>()
                    .expect("should be a valid number");

                let impossible_sets: Vec<_> = parts[1]
                    .split(";")
                    .map(|set| {
                        let colors: HashMap<String, usize> = set
                            .split(',')
                            .map(|part_line| {
                                let color_parts: Vec<&str> =
                                    part_line.trim().split_whitespace().collect();
                                let count = color_parts[0].parse().unwrap_or(0);
                                let name = color_parts[1].to_string();
                                (name, count)
                            })
                            .filter(|(name, count)| match set_limit.get(name.as_str()) {
                                Some(&limit_count) => *count > limit_count,
                                None => false,
                            })
                            .collect();
                        colors
                    })
                    .filter(|set| !set.is_empty())
                    .collect();

                if impossible_sets.len() <= 0 {
                    Some(game_number)
                } else {
                    None
                }
            })
            .sum::<usize>()
    }

    pub fn solution02(&self) -> usize {
        self.records
            .iter()
            .map(|line| {
                let parts: Vec<&str> = line.split(':').collect();
                let mut colors: HashMap<String, usize> = HashMap::new();

                parts[1].split(";").for_each(|set| {
                    set.split(',').for_each(|part_line| {
                        let color_parts: Vec<&str> = part_line.trim().split_whitespace().collect();
                        let count = color_parts[0].parse().unwrap_or(0);
                        let name = color_parts[1].to_string();
                        if let Some(existing_count) = colors.get_mut(&name) {
                            // If it exists and the new count is bigger, update the count
                            if count > *existing_count {
                                *existing_count = count;
                            }
                        } else {
                            colors.insert(name, count);
                        }
                    });
                });

                let result = colors.values().fold(1, |acc, count| acc * count);

                result
            })
            .sum::<usize>()
    }
}

impl Runner for Day02 {
    fn name(&self) -> (usize, usize) {
        (2023, 2)
    }

    fn parse(&mut self) {
        self.records = aoclib::read_lines("input/2023-02-01.txt");
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

    fn day() -> Day02 {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let mut day = Day02::new();
        day.records = aoclib::read_str(input);

        day
    }

    #[test]
    fn test_part01() {
        let output = day().solution01();

        assert_eq!(8, output);
    }

    #[test]
    fn test_part02() {
        let output = day().solution02();

        assert_eq!(2286, output);
    }
}
