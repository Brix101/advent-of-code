use std::fs;

pub fn part_01() {
    let input = fs::read_to_string("src/part-01.txt").unwrap();

    let output = solution(&input);
    println!("Part 01: {}", output)
}

fn solution(input: &str) -> u32 {
    let output = input
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
        .sum::<u32>();

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = "1abc2
                     pqr3stu8vwx
                     a1b2c3d4e5f
                     treb7uchet";

        let output = solution(input);

        assert_eq!(142, output);
    }
}
