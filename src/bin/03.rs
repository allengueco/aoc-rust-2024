advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let pattern = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();
    let mut sum = 0;
    for (_, [a, b]) in pattern.captures_iter(input).map(|m| m.extract()) {
        let result = a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
        sum += result;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let disable =
        Regex::new(r"(mul\((?<a>\d+),(?<b>\d+)\))|(?<dont>don't\(\))|(?<do>do\(\))").unwrap();
    let mut enabled = true;
    let mut sum = 0;

    for m in disable.captures_iter(input) {
        if m.name("dont").is_some() {
            enabled = false;
        }

        if m.name("do").is_some() {
            enabled = true;
        }

        if enabled {
            if let (Some(a), Some(b)) = (m.name("a"), m.name("b")) {
                sum += a.as_str().parse::<usize>().unwrap() * b.as_str().parse::<usize>().unwrap();
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
