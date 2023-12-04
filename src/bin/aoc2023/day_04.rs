use crate::Runner;

#[derive(Default)]
pub struct Day04 {
    records: Vec<String>,
}

impl Day04 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn solution01(&self) -> u32 {
        self.records
            .iter()
            .map(|line| {
                let (winning, owned) = self.get_numbers(line);

                let matches: Vec<_> = winning
                    .iter()
                    .filter(|&&num| owned.contains(&num))
                    .collect();

                if matches.len() > 0 {
                    1 << (matches.len() - 1)
                } else {
                    0
                }
            })
            .sum::<u32>()
    }

    pub fn solution02(&self) -> u32 {
        let mut cards = self
            .records
            .iter()
            .map(|line| {
                let (winning, owned) = self.get_numbers(line);
                let matches: Vec<_> = winning
                    .iter()
                    .filter(|&&num| owned.contains(&num))
                    .collect();

                (matches.len(), 1)
            })
            .collect::<Vec<_>>();

        for (index, card) in cards.clone().iter().enumerate() {
            let binding = cards.clone();
            let active_card = binding.get(index).unwrap_or(card);

            for next_index in index..(index + card.0) {
                let next_card = cards.get_mut(next_index + 1).unwrap();
                next_card.1 += active_card.1;
            }
        }

        cards.iter().map(|(_, instances)| instances).sum::<u32>()
    }

    fn get_numbers(&self, line: &str) -> (Vec<u32>, Vec<u32>) {
        let parts: Vec<&str> = line.split(':').collect();
        let scratchcards: Vec<&str> = parts[1].split("|").collect();
        let numbers = (
            scratchcards[0]
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect(),
            scratchcards[1]
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect(),
        );

        numbers
    }
}

impl Runner for Day04 {
    fn name(&self) -> (usize, usize) {
        (2023, 4)
    }

    fn parse(&mut self) {
        self.records = aoclib::read_lines("input/2023-04-01.txt");
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

    fn day() -> Day04 {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let mut day = Day04::new();
        day.records = aoclib::read_str(input);

        day
    }

    #[test]
    fn test_part01() {
        let output = day().solution01();

        assert_eq!(13, output);
    }

    #[test]
    fn test_part02() {
        let output = day().solution02();

        assert_eq!(30, output);
    }
}
