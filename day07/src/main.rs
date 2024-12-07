use std::fmt::Debug;

pub struct Day07;

#[derive(Clone, Debug)]
pub struct EquationTest {
    test: usize,
    values: Vec<usize>,
}

fn concat_nums(a: usize, b: usize) -> usize {
    (format!("{}{}", a, b)).parse().unwrap()
}

impl EquationTest {
    pub fn solveable(&self) -> bool {
        let (head, tail) = self.values.split_first().unwrap();
        self._solveable(*head, tail)
    }

    pub fn _solveable(&self, current: usize, values: &[usize]) -> bool {
        let Some((head, tail)) = values.split_first() else {
            return current == self.test;
        };

        self._solveable(current * head, tail) || self._solveable(current + head, tail)
    }

    pub fn solveable_with_concat(&self) -> bool {
        let (head, tail) = self.values.split_first().unwrap();
        self._solveable_with_concat(*head, tail)
    }

    pub fn _solveable_with_concat(&self, current: usize, values: &[usize]) -> bool {
        let Some((head, tail)) = values.split_first() else {
            return current == self.test;
        };

        self._solveable_with_concat(current * head, tail)
            || self._solveable_with_concat(current + head, tail)
            || self._solveable_with_concat(concat_nums(current, *head), tail)
    }
}

impl aoc::Puzzle for Day07 {
    type Parsed = Vec<EquationTest>;

    fn parse(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                let (test, values) = line.split_once(":").unwrap();
                let test = test.parse().unwrap();
                let values = values
                    .strip_prefix(" ")
                    .unwrap()
                    .split_whitespace()
                    .map(|d| d.parse().unwrap())
                    .collect();

                EquationTest { test, values }
            })
            .collect()
    }

    fn solve_part1(input: Self::Parsed) -> impl Debug {
        input
            .into_iter()
            .filter(|r| r.solveable())
            .map(|r| r.test)
            .sum::<usize>()
    }

    fn solve_part2(input: Self::Parsed) -> impl Debug {
        input
            .into_iter()
            .filter(|r| r.solveable_with_concat())
            .map(|r| r.test)
            .sum::<usize>()
    }
}

fn main() {
    aoc::run_puzzle::<Day07>()
}

#[test]
fn test_7290() {
    let record = EquationTest {
        test: 7290,
        values: vec![6, 8, 6, 15],
    };
    assert!(record.solveable_with_concat());
}

#[test]
fn test_concat() {
    assert_eq!(concat_nums(12, 345), 12345);
}
