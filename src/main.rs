use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("src/part-01.txt").unwrap();

    let output = solution(&input);

    println!("{:#?}", output);
}

fn solution(input: &str) -> u32 {
    let limit = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let game_number = parts[0]
                .strip_prefix("Game")
                .unwrap_or(parts[0])
                .trim()
                .parse::<u32>()
                .expect("should be a valid number");

            let sets: Vec<_> = parts[1]
                .split(";")
                .map(|set| {
                    let colors: HashMap<String, u32> = set
                        .split(',')
                        .map(|part_line| {
                            let color_parts: Vec<&str> =
                                part_line.trim().split_whitespace().collect();
                            let count = color_parts[0].parse().unwrap_or(0);
                            let name = color_parts[1].to_string();
                            (name, count)
                        })
                        .filter(|(name, count)| match limit.get(name.as_str()) {
                            Some(&limit_count) => *count >= limit_count,
                            None => false,
                        })
                        .collect();
                    colors
                })
                .filter(|set| !set.is_empty())
                .collect();

            if sets.len() <= 0 {
                Some(game_number)
            } else {
                None
            }
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let output = solution(input);

        assert_eq!(8, output);
    }
}
