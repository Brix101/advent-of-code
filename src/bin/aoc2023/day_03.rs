use crate::Runner;

#[derive(Default)]
pub struct Day03 {
    records: Vec<String>,
}

impl Day03 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn solution01(&self) -> u32 {
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

                    let unique_locations = self.remove_sequence_position(possible_locations);

                    let founds: Vec<_> = unique_locations
                        .iter()
                        .map(|&index| self.get_number_around_index(curr_line, index))
                        .collect();

                    Some(founds)
                } else {
                    None
                }
            })
            .flatten()
            .map(|s| s.parse::<u32>().unwrap_or(0))
            .sum()
    }

    pub fn solution02(&self) -> u32 {
        self.records
            .iter()
            .enumerate()
            .filter_map(|(index, curr_line)| {
                if curr_line.contains('*') {
                    let (prev_line, next_line) = self.get_prev_next_line(&index);
                    let locations = curr_line
                        .chars()
                        .enumerate()
                        .filter(|&(_, c)| c == '*')
                        .map(|(index, _)| index)
                        .collect::<Vec<_>>();

                    println!("+++++++++++++++++++++++++++");
                    println!("prev: {}", prev_line);
                    println!("curr: {}", curr_line);
                    println!("next: {}", next_line);

                    println!("pos  :{:?}", locations);
                    let totals = locations
                        .iter()
                        .map(|&pos| {
                            let prev_left = prev_line
                                .chars()
                                .nth(pos.wrapping_sub(1))
                                .map_or(false, |c| c.is_digit(10));
                            let prev_center =
                                prev_line.chars().nth(pos).map_or(false, |c| c.is_digit(10));
                            let prev_right = prev_line
                                .chars()
                                .nth(pos + 1)
                                .map_or(false, |c| c.is_digit(10));

                            let curr_left = curr_line
                                .chars()
                                .nth(pos.wrapping_sub(1))
                                .map_or(false, |c| c.is_digit(10));
                            let curr_right = curr_line
                                .chars()
                                .nth(pos + 1)
                                .map_or(false, |c| c.is_digit(10));

                            let next_left = next_line
                                .chars()
                                .nth(pos.wrapping_sub(1))
                                .map_or(false, |c| c.is_digit(10));
                            let next_center =
                                next_line.chars().nth(pos).map_or(false, |c| c.is_digit(10));
                            let next_right = next_line
                                .chars()
                                .nth(pos + 1)
                                .map_or(false, |c| c.is_digit(10));

                            let left = if curr_left {
                                self.get_number_around_index(curr_line, index - 1)
                                    .parse::<u32>()
                                    .unwrap_or(0)
                            } else if prev_left || (prev_left || prev_center) {
                                self.get_number_around_index(prev_line, index - 1)
                                    .parse::<u32>()
                                    .unwrap_or(0)
                            } else if next_left || (next_left || next_center) {
                                self.get_number_around_index(next_line, index - 1)
                                    .parse::<u32>()
                                    .unwrap_or(0)
                            } else if !prev_left || (prev_right || prev_center) {
                                self.get_number_around_index(prev_line, index + 1)
                                    .parse::<u32>()
                                    .unwrap_or(0)
                            } else {
                                0
                            };

                            let right = if curr_right {
                                self.get_number_around_index(curr_line, index + 1)
                                    .parse::<u32>()
                                    .unwrap_or(0)
                            } else if prev_right || (prev_right || prev_center) {
                                self.get_number_around_index(prev_line, index + 1)
                                    .parse::<u32>()
                                    .unwrap_or(0)
                            } else if next_right || (next_right || next_center) {
                                self.get_number_around_index(next_line, index + 1)
                                    .parse::<u32>()
                                    .unwrap_or(0)
                            } else if !next_left || (next_right || next_center) {
                                self.get_number_around_index(next_line, index + 1)
                                    .parse::<u32>()
                                    .unwrap_or(0)
                            } else {
                                0
                            };

                            println!("{},{},{}", prev_left, prev_center, prev_right);
                            println!("{},     ,{}", curr_left, curr_right);
                            println!("{},{},{}", next_left, next_center, next_right);
                            println!("{} * {}", left, right);

                            left * right
                        })
                        .collect::<Vec<_>>();
                    println!("result  :{:?}", totals);
                    Some(totals)
                } else {
                    None
                }
            })
            .flatten()
            .sum()
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
        self.records = aoclib::read_str(input);
        // self.records = aoclib::read_lines("input/2023-03-01.txt");
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
    fn test_solution01() {
        let output = day().solution01();

        assert_eq!(4361, output);
    }

    #[test]
    fn test_solution02() {
        let output = day().solution02();

        assert_eq!(467835, output);
    }
}
