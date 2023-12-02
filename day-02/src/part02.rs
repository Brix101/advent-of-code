use std::{collections::HashMap, fs};

pub fn part02() {
    let input = fs::read_to_string("day-02/src/part-01.txt").unwrap();

    let output = solution(&input);

    println!("Part 02: {:#?}", output);
}

fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let mut colors: HashMap<String, u32> = HashMap::new();

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

        assert_eq!(2286, output);
    }
}
