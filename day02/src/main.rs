use std::fmt::Debug;

use itertools::Itertools;

pub struct Day02;

fn is_report_safe(report: &[isize]) -> bool {
    let mut diffs = report.iter().tuple_windows::<(_, _)>().map(|(x, y)| y - x);
    let signs_equal = diffs.clone().map(|d| d.cmp(&0)).all_equal();
    let safe_diffs = diffs.all(|d| (1..=3).contains(&d.abs()));
    signs_equal && safe_diffs
}

/// just brute force it
/// feels like there would be a cool backtracking approach to this tho hmmm
fn is_report_safe_with_dampening(report: &[isize]) -> bool {
    for i in 0..report.len() {
        let report_without_i = report.iter().cloned().take(i - 1).skip(1).collect_vec();
        if is_report_safe(&report_without_i) {
            return true;
        }
    }

    false
}

impl aoc::Puzzle for Day02 {
    type Parsed = Vec<Vec<isize>>;

    fn parse(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect()
            })
            .collect()
    }

    fn solve_part1(input: Self::Parsed) -> impl Debug {
        input.into_iter().filter(|r| is_report_safe(r)).count()
    }

    fn solve_part2(input: Self::Parsed) -> impl Debug {
        input
            .into_iter()
            .filter(|r| is_report_safe_with_dampening(r))
            .count()
    }
}

fn main() {
    aoc::run_puzzle::<Day02>()
}

#[test]
fn test_part1() {
    use aoc::Puzzle;

    let reports = Day02::parse(
        "7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9",
    );
    let count = Day02::solve_part1(reports);
    assert_eq!(format!("{:?}", count), "2")
}
