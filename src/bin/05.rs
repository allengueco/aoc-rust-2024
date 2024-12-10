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
        eprintln!("{update:?}");
        for (first, rest) in update.iter() {
            eprintln!("\t{{ {first:?} {rest:?}");
            let value = self.table.get(first)?;
            let rest = rest?;
            eprintln!("{{ {first:?} : {value:?} }} rest: {rest:?}");
            if rest == value || rest.iter().all(|r| value.contains(r)) {
                continue;
            } else {
                eprintln!("\trest: {rest:?} value: {value:?}");
                return None;
            }
        }
        eprintln!("GOT HERE");
        Some(update.middle())
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
        self.node[self.size / 2]
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
        let (left, right) = self.update.node.split_at_checked(self.current_index)?;
        self.current_index += 1;
        left.last().map(|last| (last, Some(right)))
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let p: Vec<&str> = input.split("\n\n").collect();
    let g = Graph::new(p[0]);
    let us = p[1]
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(Update::new)
        // .skip(3)
        // .take(1)
        .collect::<Vec<Update>>();

    Some(us.into_iter().filter_map(|u| g.process(&u)).inspect(|u| eprintln!("\t\t{u:?}")).sum())
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
