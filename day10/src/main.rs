use std::{collections::HashSet, fmt::Debug};

use aoc::utils::grid::{Grid, GridPos};
use itertools::Itertools;

pub struct Day10;

fn score_trailhead(grid: &Grid<usize>, pos: GridPos, distinct: bool) -> usize {
    // Non-zeros cannot have score
    if grid.get(pos).unwrap() != 0 {
        return 0;
    }

    // Determine the score
    let mut score = 0;
    let mut visited = HashSet::new();
    let mut open = vec![pos];
    while let Some(next) = open.pop() {
        // Mark as visited
        visited.insert(next);
        let height = grid.get(next).unwrap();

        // Add to score
        if grid.get(next).unwrap() == 9 {
            score += 1;
        }

        // Flood fill
        open.extend(
            next.neighbours()
                .filter(|pos| grid.get(*pos) == Some(height + 1))
                .filter(|pos| distinct || !visited.contains(pos)),
        );
    }

    score
}

impl aoc::Puzzle for Day10 {
    type Parsed = Grid<usize>;

    fn parse(input: &str) -> Self::Parsed {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect_vec()
            })
            .collect_vec();
        Grid::new(grid)
    }

    fn solve_part1(grid: Self::Parsed) -> impl Debug {
        grid.positions()
            .map(|pos| score_trailhead(&grid, pos, false))
            .sum::<usize>()
    }

    fn solve_part2(grid: Self::Parsed) -> impl Debug {
        grid.positions()
            .map(|pos| score_trailhead(&grid, pos, true))
            .sum::<usize>()
    }
}

fn main() {
    aoc::run_puzzle::<Day10>()
}
