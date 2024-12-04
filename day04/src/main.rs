use std::fmt::Debug;

use itertools::Itertools;

pub struct Day04;

#[derive(Clone, Debug)]
pub struct WordSearch(Vec<Vec<char>>);

pub type Kernel = &'static [&'static [char]];

#[rustfmt::skip]
static XMAS_KERNELS: &[Kernel] = &[
    // Forwards
    &[&['X', 'M', 'A', 'S']],

    // Backwards
    &[&['S', 'A', 'M', 'X']],

    // Downwards
    &[&['X'], &['M'], &['A'], &['S']],

    // Upwards
    &[&['S'], &['A'], &['M'], &['X']],

    // Forwards diagonal
    &[
        &['X'],
        &['*', 'M'],
        &['*', '*', 'A'],
        &['*', '*', '*', 'S'],
    ],

    // Forwards diagonal rev
    &[
        &['S'],
        &['*', 'A'],
        &['*', '*', 'M'],
        &['*', '*', '*', 'X'],
    ],

    // Backwards diagonal
    &[
        &['*', '*', '*', 'S'],
        &['*', '*', 'A'],
        &['*', 'M'],
        &['X'],
    ],

    // Backwards diagonal rev
    &[
        &['*', '*', '*', 'X'],
        &['*', '*', 'M'],
        &['*', 'A'],
        &['S'],
    ],
];

#[rustfmt::skip]
static XMAS_CROSS_KERNELS: &[Kernel] = &[
    &[
        &['M', '*', 'M'],
        &['*', 'A', '*'],
        &['S', '*', 'S'],
    ],

    &[
        &['S', '*', 'M'],
        &['*', 'A', '*'],
        &['S', '*', 'M'],
    ],

    &[
        &['M', '*', 'S'],
        &['*', 'A', '*'],
        &['M', '*', 'S'],
    ],

    &[
        &['S', '*', 'S'],
        &['*', 'A', '*'],
        &['M', '*', 'M'],
    ],
];

impl WordSearch {
    pub fn check_kernel_at(&self, kernel: Kernel, check_i: usize, check_j: usize) -> bool {
        for (kernel_i, row) in kernel.iter().enumerate() {
            for (kernel_j, mask) in row.iter().enumerate() {
                // Ignore wildcard
                if *mask == '*' {
                    continue;
                }

                // if out of bounds, is not a match
                let i = check_i + kernel_i;
                let j = check_j + kernel_j;
                if !(0..self.0.len()).contains(&i) || !(0..self.0.len()).contains(&j) {
                    return false;
                }

                // Otherwise must match
                let sample = self.0[check_i + kernel_i][check_j + kernel_j];
                if sample != *mask {
                    return false;
                }
            }
        }

        true
    }

    pub fn count_kernels(&self, kernels: &[Kernel]) -> usize {
        let size = self.0.len();
        kernels
            .iter()
            .map(|kernel| {
                Itertools::cartesian_product(0..size, 0..size)
                    .filter(|(i, j)| self.check_kernel_at(kernel, *i, *j))
                    .count()
            })
            .sum::<usize>()
    }
}

impl aoc::Puzzle for Day04 {
    /// Rows
    /// note: is a square
    type Parsed = WordSearch;

    fn parse(input: &str) -> Self::Parsed {
        let grid = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        WordSearch(grid)
    }

    fn solve_part1(input: Self::Parsed) -> impl Debug {
        input.count_kernels(XMAS_KERNELS)
    }

    fn solve_part2(input: Self::Parsed) -> impl Debug {
        input.count_kernels(XMAS_CROSS_KERNELS)
    }
}

fn main() {
    aoc::run_puzzle::<Day04>()
}
