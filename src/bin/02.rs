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
    Start,
}

impl Default for State {
    fn default() -> Self {
        Self {
            prev: u32::MIN,
            direction: Direction::Start,
        }
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let reports: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.split(" ").map(|n| n.parse().unwrap()).collect())
        .collect();

    eprintln!("{:?}", reports);

    fn is_safe(numbers: Vec<u32>) -> Option<()> {
        numbers.iter().fold(None, |state, &current| {
            eprintln!("Current state: {:?}", state);
            // if the first, we just set the prev to current
            if state.is_none() {
                return Some(State {
                    prev: current,
                    direction: Direction::Start,
                });
            }

            let mut state = state.unwrap();

            // what is the direction after current?
            let dir = if state.prev > current {
                Direction::Decreasing
            } else {
                Direction::Increasing
            };
            state.direction = dir;

            // its not the same as the current direction, return early
            if dir != state.direction {
                return None;
            }

            if (1..=3).contains(&state.prev.abs_diff(current)) {
                state.prev = current;
                Some(state)
            } else {
                None
            }
        })?;
        eprintln!();
        Some(())
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
