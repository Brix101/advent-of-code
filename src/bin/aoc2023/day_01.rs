use std::fs;

use crate::Runner;

#[derive(Default)]
pub struct Day01 {
    input01: String,
    input02: String,
}

impl Day01 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn solution01(&self) -> u32 {
        self.input01
            .lines()
            .map(|line| {
                let mut it = line.chars().filter_map(|ch| ch.to_digit(10));
                let first = it.next().expect("should be a number");

                match it.last() {
                    Some(num) => format!("{first}{num}"),
                    None => format!("{first}{first}"),
                }
                .parse::<u32>()
                .expect("should be a valid number")
            })
            .sum::<u32>()
    }

    pub fn solution02(&self) -> u32 {
        self.input02
            .lines()
            .map(|line| {
                let mut it = (0..line.len()).filter_map(|index| {
                    let reduced_line = &line[index..];
                    let result = match reduced_line {
                        s if s.starts_with("one") => '1',
                        s if s.starts_with("two") => '2',
                        s if s.starts_with("three") => '3',
                        s if s.starts_with("four") => '4',
                        s if s.starts_with("five") => '5',
                        s if s.starts_with("six") => '6',
                        s if s.starts_with("seven") => '7',
                        s if s.starts_with("eight") => '8',
                        s if s.starts_with("nine") => '9',
                        _ => reduced_line.chars().next().unwrap(),
                    };

                    result.to_digit(10)
                });

                let first = it.next().expect("should be a number");

                match it.last() {
                    Some(num) => format!("{first}{num}"),
                    None => format!("{first}{first}"),
                }
                .parse::<u32>()
                .expect("should be a valid number")
            })
            .sum::<u32>()
    }
}

impl Runner for Day01 {
    fn name(&self) -> (usize, usize) {
        (2023, 1)
    }

    fn parse(&mut self) {
        self.input01 = fs::read_to_string("input/2023-01-01.txt").unwrap();
        self.input02 = fs::read_to_string("input/2023-01-02.txt").unwrap();
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

    fn day() -> Day01 {
        let input01 = "1abc2
                   pqr3stu8vwx
                   a1b2c3d4e5f
                   treb7uchet";
        let input02 = "two1nine
                    eightwothree
                    abcone2threexyz
                    xtwone3four
                    4nineeightseven2
                    zoneight234
                    7pqrstsixteen";

        let mut day = Day01::new();
        day.input01 = input01.to_string();

        day.input02 = input02.to_string();

        day
    }

    #[test]
    fn test_solution01() {
        let output = day().solution01();

        assert_eq!(142, output);
    }

    #[test]
    fn test_solution02() {
        let output = day().solution02();

        assert_eq!(281, output);
    }
}
