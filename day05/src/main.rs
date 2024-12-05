use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

use itertools::Itertools;

pub struct Day05;

#[derive(Debug, Clone)]
pub struct SuccessorMap(HashMap<usize, Vec<usize>>);

impl SuccessorMap {
    pub fn update_is_valid(&self, update: &[usize]) -> bool {
        let mut update_queue: VecDeque<_> = update.iter().collect();
        while !update_queue.is_empty() {
            let page = update_queue.pop_front().unwrap();
            let successors = self.0.get(page);
            if let Some(successors) = successors {
                // All successors must be after this or not in the update at all
                let right_spot = successors
                    .iter()
                    .all(|succ| !update.contains(succ) || update_queue.contains(&succ));
                if !right_spot {
                    return false;
                }
            }
        }

        true
    }

    /// Little bit lazy but this just a sort of "bubble sort"
    pub fn fix_update(&self, update: &[usize]) -> Vec<usize> {
        let mut update = update.iter().cloned().collect_vec();
        while !self.update_is_valid(&update) {
            update.sort_by(|a, b| {
                let succs = self.0.get(a);
                match succs {
                    Some(succs) => {
                        if succs.contains(b) {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    }
                    None => Ordering::Equal,
                }
            });
        }
        update
    }
}

pub type SafetyManual = (SuccessorMap, Vec<Vec<usize>>);

impl aoc::Puzzle for Day05 {
    type Parsed = SafetyManual;

    fn parse(input: &str) -> Self::Parsed {
        // Lets start by splitting it in two
        let (ordering, updates) = input.split_once("\n\n").unwrap();

        // we put each ordering entry into a hashmap
        // In this set, if (k,v) is present then k < v
        let mut successor_map = HashMap::new();
        for line in ordering.lines() {
            let (x, y) = line.split_once("|").unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            successor_map
                .entry(x)
                .and_modify(|succs: &mut Vec<usize>| succs.push(y))
                .or_insert_with(|| vec![y]);
        }

        // Parse each update
        let updates = updates
            .lines()
            .map(|l| l.split(",").map(|x| x.parse().unwrap()).collect_vec())
            .collect_vec();

        (SuccessorMap(successor_map), updates)
    }

    fn solve_part1((successor_map, updates): Self::Parsed) -> impl Debug {
        updates
            .iter()
            .filter(|update| successor_map.update_is_valid(update))
            .map(|update| update[update.len() / 2])
            .sum::<usize>()
    }

    fn solve_part2((successor_map, updates): Self::Parsed) -> impl Debug {
        updates
            .iter()
            .filter(|update| !successor_map.update_is_valid(update))
            .map(|update| successor_map.fix_update(update))
            .map(|update| update[update.len() / 2])
            .sum::<usize>()
    }
}

fn main() {
    aoc::run_puzzle::<Day05>()
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use aoc::Puzzle;

    use super::*;

    #[test]
    fn test_p1() {
        let sample_input = read_to_string("./sample.txt").expect("Cannot read sample.txt");
        let sample_input = Day05::parse(&sample_input);
        let output = Day05::solve_part1(sample_input);
        dbg!(output);
    }

    #[test]
    fn test_p2() {
        let sample_input = read_to_string("./sample.txt").expect("Cannot read sample.txt");
        let sample_input = Day05::parse(&sample_input);
        let output = Day05::solve_part2(sample_input);
        dbg!(output);
    }
}
