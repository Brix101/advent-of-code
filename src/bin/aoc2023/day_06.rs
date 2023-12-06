use crate::Runner;

#[derive(Default)]
pub struct DayO6 {
    input: String,
}

impl DayO6 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn solution01(&self) -> u32 {
        let maps = self
            .input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|item| item.parse::<u32>().ok())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let time = maps[0].clone();
        let distances = maps[1].clone();

        let result = time
            .into_iter()
            .zip(distances)
            .map(|(time, record_distance)| {
                (0..time)
                    .filter_map(|speed| {
                        let my_distance = (time - speed) * speed;
                        (my_distance > record_distance).then_some(my_distance)
                    })
                    .count()
            })
            .product::<usize>();

        result as u32
    }

    pub fn solution02(&self) -> u32 {
        let maps = self
            .input
            .lines()
            .map(|line| {
                let item = line
                    .split_whitespace()
                    .filter_map(|item| item.parse::<u32>().ok())
                    .map(|num| num.to_string())
                    .collect::<Vec<_>>()
                    .join("");
                item.parse::<u64>().unwrap_or(0)
            })
            .collect::<Vec<_>>();

        let time = maps[0].clone();
        let distance = maps[1].clone();
        let result = (0..time)
            .filter_map(|speed| {
                let my_distance = (time - speed) * speed;
                (my_distance > distance).then_some(my_distance)
            })
            .count();

        result as u32
    }
}

impl Runner for DayO6 {
    fn name(&self) -> (usize, usize) {
        (2023, 6)
    }

    fn parse(&mut self) {
        let file = include_str!("../../../input/2023-06-01.txt");
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

    fn day() -> DayO6 {
        let input = "Time:      7  15   30
        Distance:  9  40  200";

        let mut day = DayO6::new();
        day.input = input.to_string();

        day
    }

    #[test]
    fn test_part01() {
        let output = day().solution01();

        assert_eq!(288, output);
    }

    #[test]
    fn test_part02() {
        let output = day().solution02();

        assert_eq!(71503, output);
    }
}
