advent_of_code::solution!(5);

#[derive(Clone, Default)]
struct Graph {
    adjacency: Vec<Vec<usize>>,
}

impl Graph {
    fn new(input: &str) -> Self {
        Self {
            adjacency: Graph::process(input),
        }
    }

    fn process(input: &str) -> Vec<Vec<usize>> {
        let mut list = vec![];

        for r in input.lines() {
            let p: Vec<usize> = r
                .split("|")
                .collect::<Vec<&str>>()
                .iter()
                .map(|c| c.parse().unwrap())
                .collect();
        }

        list
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
