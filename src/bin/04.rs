advent_of_code::solution!(4);

const WORD: &str = "XMAS";

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut words = 0;
    for (i, row) in puzzle.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == 'X' {
                words += search((i, j), 0);
            }
        }
    }
    Some(words)
}

struct Neighbor {
    i: isize,
    j: isize,
}

enum Direction {
    N,
    S,
    E,
    W,
    NE,
    SE,
    SW,
    NW,
}

impl Direction {
    fn dir(&self) -> (isize, isize) {
        match &self {
            Direction::N => (-1, 0),
            Direction::S => (1, 0),
            Direction::E => (0, 1),
            Direction::W => (0, -1),
            Direction::NE => (-1, 1),
            Direction::SE => (1, 1),
            Direction::SW => (1, -1),
            Direction::NW => (-1, -1),
        }
    }

    fn all() -> impl Iterator<Item = Direction> {
        [
            Self::N,
            Self::S,
            Self::E,
            Self::W,
            Self::NE,
            Self::SE,
            Self::SW,
            Self::NW,
        ]
        .into_iter()
    }
}

fn neighbors(root: (usize, usize)) -> impl Iterator<Item = Neighbor> {
    Direction::all().map(|d| d.dir()).map(|(i, j)| Neighbor {
        i: root.0.wrapping_add(i),
        j: root.1 + j,
    })
}

fn search(root: (usize, usize), index: usize) -> u32 {
    let mut frontier: Vec<(usize, usize)> = vec![root];

    while let Some(current) = frontier.pop() {}
    0
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
