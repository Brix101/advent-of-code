use std::fs;

pub fn part_02() {
    let input = fs::read_to_string("src/part-02.txt").unwrap();

    let output = solution(&input);
    println!("Part 02: {}", output)
}

fn solution(input: &str) -> u32 {
    let output = input
        .lines()
        .map(|line| {
            let mut it = (0..line.len()).filter_map(|idx| {
                let reduced_line = &line[idx..];
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
        .sum::<u32>();

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = "two1nine
                    eightwothree
                    abcone2threexyz
                    xtwone3four
                    4nineeightseven2
                    zoneight234
                    7pqrstsixteen";

        let output = solution(input);

        assert_eq!(281, output);
    }
}
