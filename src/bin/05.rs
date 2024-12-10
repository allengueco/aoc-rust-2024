use std::collections::HashMap;

advent_of_code::solution!(5);

#[derive(Clone, Default)]
struct Graph {
    table: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new(input: &str) -> Self {
        Self {
            table: Graph::init(input),
        }
    }

    fn init(input: &str) -> HashMap<usize, Vec<usize>> {
        let mut table = HashMap::new();

        for r in input.lines() {
            let p: Vec<usize> = r
                .split("|")
                .collect::<Vec<&str>>()
                .iter()
                .map(|c| c.parse().unwrap())
                .collect();

            table.entry(p[0]).or_insert(vec![]).push(p[1]);
        }
        table
    }

    fn process(&self, update: &Update) -> Option<usize> {
        let mut sum = 0;
        for (first, rest) in update.iter().skip(1) {
            let value = self.table.get(first)?;
            if let Some(m) = rest {
                eprintln!("{value:?} == {rest:?}");
                if value.iter().all(|v| m.contains(v)) {
                    sum += update.middle();
                }
            }
        }
        Some(sum)
    }
}

#[derive(Default, Clone, Debug)]
struct Update {
    node: Vec<usize>,
    size: usize,
}

impl Update {
    fn new(input: &str) -> Self {
        let updates = input
            .split(",")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Self {
            size: updates.len(),
            node: updates,
        }
    }
    fn middle(&self) -> usize {
        dbg!(self.node[self.size / 2])
    }
    fn iter(&self) -> UpdateIterator<'_> {
        UpdateIterator {
            update: self,
            current_index: 1,
        }
    }
}

#[derive(Clone)]
struct UpdateIterator<'a> {
    update: &'a Update,
    current_index: usize,
}

impl<'a> Iterator for UpdateIterator<'a> {
    type Item = (&'a usize, Option<&'a [usize]>);

    fn next(&mut self) -> Option<Self::Item> {
        self.current_index += 1;
        match self.update.node.split_at_checked(self.current_index) {
            None => None,
            Some((left, right)) => {
                match right {
                    [] => None,
                    r => left.last().map(|last| (last, Some(r)))
                }
            }
        }
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let p: Vec<&str> = input.split("\r\n\r\n").collect();
    let g = Graph::new(p[0]);
    let us = p[1]
        .split("\r\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(Update::new)
        .collect::<Vec<Update>>();


    Some(us.into_iter().filter_map(|u| g.process(&u)).sum())
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
