use std::fmt::Debug;

use itertools::Itertools;
use tqdm::Iter;

pub struct Day11;

#[derive(Clone, Debug)]
pub struct Stones(Vec<usize>);

impl Stones {
    pub fn blink(&mut self) {
        for index in (0..self.0.len()).rev() {
            match self.0[index] {
                0 => {
                    self.0[index] = 1;
                }
                x if (x.ilog10() + 1) % 2 == 0 => {
                    let n = x.ilog10() as usize;
                    let p: usize = (n + 1) / 2;
                    let o = 10_usize.pow(p as u32);
                    self.0[index] = x / o;
                    self.0.insert(index + 1, x % o);
                }
                _ => {
                    self.0[index] *= 2024;
                }
            }
        }
    }
}

impl aoc::Puzzle for Day11 {
    type Parsed = Stones;

    fn parse(input: &str) -> Self::Parsed {
        Stones(
            input
                .lines()
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect_vec(),
        )
    }

    fn solve_part1(mut input: Self::Parsed) -> impl Debug {
        for _ in 0..25 {
            input.blink();
        }
        input.0.len()
    }

    fn solve_part2(mut input: Self::Parsed) -> impl Debug {
        for _ in (0..75).tqdm() {
            input.blink();
        }
        input.0.len()
    }
}

fn main() {
    aoc::run_puzzle::<Day11>()
}
