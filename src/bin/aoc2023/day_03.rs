use itertools::Itertools;
use std::{collections::BTreeMap, usize};

use crate::Runner;

#[derive(Default)]
pub struct Day03 {
    records: Vec<String>,
}

#[derive(Debug)]
enum Value {
    Symbol(char),
    Empty,
    Number(u32),
}

impl Day03 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn solution01(&self) -> usize {
        self.records
            .iter()
            .enumerate()
            .filter_map(|(index, curr_line)| {
                if curr_line.chars().any(|ch| ch.is_digit(10)) {
                    let (prev_line, next_line) = self.get_prev_next_line(&index);

                    let locations: Vec<_> = curr_line
                        .chars()
                        .enumerate()
                        .filter(|&(_, c)| c.is_digit(10))
                        .map(|(index, _)| index)
                        .collect();

                    let possible_locations: Vec<_> = locations
                        .iter()
                        .filter(|&&pos| {
                            self.check_position(&prev_line, pos)
                                || self.check_position(&curr_line, pos)
                                || self.check_position(&next_line, pos)
                        })
                        .cloned()
                        .collect();

                    let unique_positions = self.remove_sequence_position(possible_locations);

                    let founds: Vec<_> = unique_positions
                        .iter()
                        .map(|&index| self.get_number_around_index(curr_line, index))
                        .collect();

                    Some(founds)
                } else {
                    None
                }
            })
            .flatten()
            .map(|s| s.parse::<usize>().unwrap_or(0))
            .sum()
    }

    pub fn solution02(&self) -> usize {
        let map = self
            .records
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, character)| {
                    (
                        (y as i32, x as i32),
                        match character {
                            '.' => Value::Empty,
                            c if c.is_ascii_digit() => {
                                Value::Number(c.to_digit(10).expect("should be a number"))
                            }
                            c => Value::Symbol(c),
                        },
                    )
                })
            })
            .collect::<BTreeMap<(i32, i32), Value>>();

        let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];
        for ((y, x), value) in map.iter() {
            if let Value::Number(num) = value {
                match numbers.iter().last() {
                    Some(v) => {
                        let last_num = v.iter().last();
                        match last_num {
                            Some(((last_num_x, _), _)) => {
                                if last_num_x + 1 == *x {
                                    let last = numbers.iter_mut().last().expect("should exist");
                                    last.push(((*x, *y), *num));
                                } else {
                                    numbers.push(vec![((*x, *y), *num)]);
                                }
                            }
                            None => unimplemented!("shouldn't happen"),
                        }
                    }
                    None => {
                        numbers.push(vec![((*x, *y), *num)]);
                    }
                }
            }
        }

        // map: entire grid
        // numbers: sequential numbers
        let mut total = 0;
        for symbol in map
            .iter()
            .filter(|(_, value)| matches!(value, Value::Symbol('*')))
        {
            // (x,y)
            let positions = [
                (1, 0),
                (1, -1),
                (0, -1),
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ];
            let pos_to_check: Vec<(i32, i32)> = positions
                .iter()
                .map(|outer_pos| {
                    // outer_pos.x + pos.x, .y + .y
                    (outer_pos.0 + symbol.0 .1, outer_pos.1 + symbol.0 .0)
                })
                .collect();

            // dbg!(pos_to_check.len(), pos_to_check);
            let mut indexes_of_numbers = vec![];

            for pos in pos_to_check {
                for (i, num_list) in numbers.iter().enumerate() {
                    if num_list
                        .iter()
                        .find(|(num_pos, _)| num_pos == &pos)
                        .is_some()
                    {
                        indexes_of_numbers.push(i);
                    }
                }
            }

            let is_gear = indexes_of_numbers.iter().unique().count() == 2;

            if is_gear {
                total += indexes_of_numbers
                    .iter()
                    .unique()
                    .map(|index| {
                        numbers[*index]
                            .iter()
                            .map(|(_, num)| num.to_string())
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap()
                    })
                    .product::<usize>();
            }
        }
        total.to_string().parse::<usize>().unwrap_or(0)
    }

    fn check_position(&self, line: &str, pos: usize) -> bool {
        let result1 = line
            .chars()
            .nth(pos.wrapping_sub(1))
            .map_or(false, |c| c != '.' && !c.is_digit(10));
        let result2 = line
            .chars()
            .nth(pos)
            .map_or(false, |c| c != '.' && !c.is_digit(10));
        let result3 = line
            .chars()
            .nth(pos + 1)
            .map_or(false, |c| c != '.' && !c.is_digit(10));

        result1 || result2 || result3
    }

    fn get_number_around_index(&self, line: &str, index: usize) -> String {
        let mut result = String::new();

        // Collect digits to the left of the index
        let mut left_index = index;
        while let Some(ch) = line.chars().nth(left_index) {
            if ch.is_digit(10) {
                result.insert(0, ch);
                left_index = left_index.wrapping_sub(1);
            } else {
                break;
            }
        }

        // Collect digits to the right of the index
        let mut right_index = index + 1;
        while let Some(ch) = line.chars().nth(right_index) {
            if ch.is_digit(10) {
                result.push(ch);
                right_index += 1;
            } else {
                break;
            }
        }

        result
    }

    fn remove_sequence_position(&self, numbers: Vec<usize>) -> Vec<usize> {
        let mut result = Vec::new();

        let mut current_sequence = Vec::new();
        let mut current_max = 0;

        for &num in numbers.iter() {
            if let Some(&prev_number) = current_sequence.last() {
                if num == prev_number + 1 {
                    // Continue the sequence
                    current_sequence.push(num);

                    // Update the current_max if num is greater
                    current_max = current_max.max(num);
                } else {
                    // End of the sequence, add the current_max to the result
                    result.push(current_max);

                    // Reset current_max and start a new sequence
                    current_max = num;
                    current_sequence = vec![num];
                }
            } else {
                // Start the first sequence
                current_max = num;
                current_sequence.push(num);
            }
        }

        // If the last sequence is complete, add the current_max to the result
        result.push(current_max);

        result
    }

    fn get_prev_next_line(&self, index: &usize) -> (&str, &str) {
        let records = &self.records;

        let prev_line: &str = records
            .get(index.wrapping_sub(1))
            .map_or("..........", |prev_line_value| prev_line_value.as_str());
        let next_line: &str = records
            .get(*index + 1)
            .map_or("..........", |next_line_value| next_line_value.as_str());

        (prev_line, next_line)
    }
}

impl Runner for Day03 {
    fn name(&self) -> (usize, usize) {
        (2023, 3)
    }

    fn parse(&mut self) {
        //         let input = "467..114..
        // ...*......
        // ..35..633.
        // ......#...
        // 617*......
        // .....+.58.
        // ..592.....
        // ......755.
        // ...$.*....
        // .664.598..";
        //         self.records = aoclib::read_str(input);
        self.records = aoclib::read_lines("input/2023-03-01.txt");
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

    fn day() -> Day03 {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let mut day = Day03::new();
        day.records = aoclib::read_str(input);

        day
    }

    #[test]
    fn test_part01() {
        let output = day().solution01();

        assert_eq!(4361, output);
    }

    #[test]
    fn test_part02() {
        let output = day().solution02();

        assert_eq!(467835, output);
    }
}
