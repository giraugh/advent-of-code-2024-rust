use std::fmt::Debug;

use hashbag::HashBag;
use itertools::Itertools;
use tqdm::Iter;

pub struct Day11;

#[derive(Clone, Debug)]
pub struct Stones(HashBag<usize>);

impl Stones {
    pub fn new(items: impl IntoIterator<Item = usize>) -> Self {
        Self(items.into_iter().collect())
    }
}

impl Stones {
    pub fn blink(&mut self) {
        let mut new_bag = HashBag::new();
        self.0.set_iter().for_each(|(value, count)| match value {
            0 => {
                new_bag.insert_many(1, count);
            }
            x if (x.ilog10() + 1) % 2 == 0 => {
                let n = (x.ilog10() + 1) / 2;
                let p = 10_usize.pow(n);
                new_bag.insert_many(x / p, count);
                new_bag.insert_many(x % p, count);
            }
            _ => {
                new_bag.insert_many(value * 2024, count);
            }
        });

        self.0 = new_bag;
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}

impl aoc::Puzzle for Day11 {
    type Parsed = Stones;

    fn parse(input: &str) -> Self::Parsed {
        Stones::new(
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
        input.size()
    }

    fn solve_part2(mut input: Self::Parsed) -> impl Debug {
        for _ in 0..75 {
            input.blink();
        }
        input.size()
    }
}

fn main() {
    aoc::run_puzzle::<Day11>()
}
