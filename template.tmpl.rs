use std::fmt::Debug;

use itertools::Itertools;

pub struct Day06;

impl aoc::Puzzle for Day06 {
    type Parsed = ();

    fn parse(input: &str) -> Self::Parsed {}

    fn solve_part1(input: Self::Parsed) -> impl Debug {}

    fn solve_part2(input: Self::Parsed) -> impl Debug {}
}

fn main() {
    aoc::run_puzzle::<Day06>()
}
