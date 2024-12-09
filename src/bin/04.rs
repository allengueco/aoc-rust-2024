use std::ops::Index;

advent_of_code::solution!(4);

const WORD: &str = "XMAS";

#[derive(Debug)]
struct Puzzle {
    data: Vec<Vec<char>>,
    m: usize,
    n: usize,
}

impl Index<(usize, usize)> for Puzzle {
    type Output = char;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl Puzzle {
    fn new(input: &str) -> Self {
        let data: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let m = data.len();
        let n = data.first().unwrap().len();
        Self { data, m, n }
    }

    // fn neighbors(&self, coord: (usize, usize)) -> impl Iterator<Item = Neighbor> {
    //     Direction::all()
    //         .map(|d| d.dir())
    //         .map(move |(i, j)| {
    //             (
    //                 i.checked_add_unsigned(coord.0),
    //                 j.checked_add_unsigned(coord.1),
    //             )
    //         })
    //         .filter_map(move |coord| match coord {
    //             (Some(i), Some(j)) => Some((i as usize, j as usize)),
    //             _ => None,
    //         })
    //         .filter_map(move |coord| match coord {
    //             (Some(i), Some(j)) => {
    //                 if (0..self.m).contains(&i) && (0..self.n).contains(&j) {
    //                     Some((i, j))
    //                 } else {
    //                     None
    //                 }
    //             }
    //             _ => None,
    //         })
    //         .filter_map(move |coord| match coord {
    //             (Some(i), Some(j)) => Some((i, j)),
    //             _ => None,
    //         })
    //         .map(|(i, j)| Neighbor { i, j })
    // }
}

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut words = 0;
    for (i, row) in puzzle.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == 'X' {
                words += search(&puzzle, (i, j));
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
    Direction::all()
        .map(|d| d.dir())
        .map(move |(i, j)| {
            (
                i.checked_add_unsigned(root.0),
                j.checked_add_unsigned(root.1),
            )
        })
        .filter_map(|coord| match coord {
            (Some(i), Some(j)) => Some((i, j)),
            _ => None,
        })
        .map(|(i, j)| Neighbor { i, j })
}

fn search(puzzle: &[Vec<char>], root: (usize, usize)) -> u32 {
    let mut index = 0;
    let mut frontier: Vec<(usize, usize)> = vec![root];

    while let Some(current) = frontier.pop() {
        // If current matches the letter we're looking for
        if WORD.as_bytes()[index] as char == puzzle[current.0][current.1] {
            index += 1;
        }

        for n in neighbors(current) {
            if WORD.as_bytes()[index] as char == puzzle[n.i][n.j] {
                frontier.push((n.i, n.j));
            }
        }
    }
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
