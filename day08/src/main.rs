use std::{collections::HashSet, fmt::Debug};

use aoc::utils::grid::{Grid, GridPos};
use itertools::Itertools;

pub struct Day08;

fn get_antennae(grid: &Grid<char>) -> Vec<(GridPos, char)> {
    // Pull out the antanae
    let mut ants = Vec::new();
    for loc in grid.positions() {
        let cell = grid.get(loc).unwrap();
        if cell != '.' {
            ants.push((loc, cell));
        }
    }
    ants
}

impl aoc::Puzzle for Day08 {
    type Parsed = Grid<char>;

    fn parse(input: &str) -> Self::Parsed {
        let grid = input.lines().map(|c| c.chars().collect()).collect();
        Grid::new(grid)
    }

    fn solve_part1(input: Self::Parsed) -> impl Debug {
        // Consider each pair of antenae
        let antinodes: HashSet<_> = get_antennae(&input)
            .into_iter()
            .tuple_combinations()
            // Only keep matching antennae
            .filter(|((_, a), (_, b))| a == b)
            // Pull out possible antinode positions
            .flat_map(|((pos_a, _), (pos_b, _))| {
                let a_to_b = pos_b - pos_a;
                let possible_1 = pos_a - a_to_b;
                let possible_2 = pos_b + a_to_b;
                vec![possible_1, possible_2].into_iter()
            })
            // Keep inbounds positions
            .filter(|pos| pos.in_grid(&input))
            .collect();

        antinodes.len()
    }

    fn solve_part2(input: Self::Parsed) -> impl Debug {
        // Consider each pair of antenae
        let antinodes: HashSet<_> = get_antennae(&input)
            .into_iter()
            .tuple_combinations()
            // Only keep matching antennae
            .filter(|((_, a), (_, b))| a == b)
            // Pull out possible antinode positions
            // this time with harmonic resonancy :taps brain:
            .flat_map(|((pos_a, _), (pos_b, _))| {
                let a_to_b = pos_b - pos_a;
                // Yeah... I know....
                (-150..=150).map(move |x| pos_a + a_to_b * x)
            })
            // Keep inbounds positions
            .filter(|pos| pos.in_grid(&input))
            .collect();

        antinodes.len()
    }
}

fn main() {
    aoc::run_puzzle::<Day08>()
}
