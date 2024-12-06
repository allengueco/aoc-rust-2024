advent_of_code::solution!(2);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct State {
    prev: u32,
    direction: Direction,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Increasing,
    Decreasing,
}

pub fn part_one(input: &str) -> Option<usize> {
    let reports: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.split(" ").map(|n| n.parse().unwrap()).collect())
        .collect();

    fn is_safe(numbers: Vec<u32>) -> Option<()> {

        let is_asc = numbers.windows(2).all(|w| w[0] < w[1]);
        let is_des = numbers.windows(2).all(|w| w[0] > w[1]);

        if !(is_asc || is_des) {
            return None;
        }

        numbers.windows(2).all(|w| (1..=3).contains(&w[0].abs_diff(w[1]))).then_some(())
    }

    Some(
        reports
            .into_iter()
            // .inspect(|m| println!("about to filter {:?}", m))
            .map(is_safe)
            // .inspect(|m| println!("made it through {:?}", m))
            .filter(Option::is_some)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
