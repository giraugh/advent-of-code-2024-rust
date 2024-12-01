use std::{collections::HashMap, fmt::Debug};

use itertools::Itertools;

pub struct Day01;

impl aoc::Puzzle for Day01 {
    type Parsed = (Vec<usize>, Vec<usize>);

    fn parse(input: &str) -> Self::Parsed {
        let mut lists: Self::Parsed = Default::default();

        for line in input.lines() {
            let (left, right) = line
                .split_whitespace()
                .take(2)
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap();
            lists.0.push(left);
            lists.1.push(right);
        }

        lists
    }

    fn solve_part1(mut input: Self::Parsed) -> impl Debug {
        // Sort lists
        input.0.sort();
        input.1.sort();

        // Compare matching
        let diff_sum: usize = std::iter::zip(input.0.iter(), input.1.iter())
            .map(|(&x, &y)| usize::abs_diff(x, y))
            .sum();

        diff_sum
    }

    fn solve_part2(input: Self::Parsed) -> impl Debug {
        // Count occs in right list
        let mut right_counts: HashMap<usize, usize> = Default::default();
        for &i in input.1.iter() {
            right_counts.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }

        input
            .0
            .iter()
            .map(|x| x * right_counts.get(x).unwrap_or(&0))
            .sum::<usize>()
    }
}

fn main() {
    aoc::run_puzzle::<Day01>()
}
