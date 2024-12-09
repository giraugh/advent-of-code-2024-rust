use std::{fmt::Debug, iter};

use itertools::Itertools;

pub struct Day09;

#[derive(Clone, Copy, PartialEq)]
pub enum Run {
    Empty { length: usize },
    Filled { id: usize, length: usize },
}

impl Run {
    pub fn is_empty(&self) -> bool {
        matches!(self, Run::Empty { .. })
    }

    pub fn length(&self) -> usize {
        match self {
            Run::Empty { length } => *length,
            Run::Filled { length, .. } => *length,
        }
    }
}

fn compute_runs_checksum(runs: &[Run]) -> usize {
    runs.iter()
        .flat_map(|r| match r {
            Run::Empty { length } => iter::repeat_n(0, *length),
            Run::Filled { id, length } => iter::repeat_n(*id, *length),
        })
        .enumerate()
        .map(|(id, i)| id * i)
        .sum::<usize>()
}

impl std::fmt::Debug for Run {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Run::Empty { length } => write!(f, "{}", ".".repeat(*length)),
            Run::Filled { id, length } => write!(f, "{}", id.to_string().repeat(*length)),
        }
    }
}

impl aoc::Puzzle for Day09 {
    type Parsed = Vec<Run>;

    fn parse(input: &str) -> Self::Parsed {
        input
            .lines()
            .take(1)
            .flat_map(|line| line.chars())
            .map(|d| d.to_digit(10).unwrap() as usize)
            .enumerate()
            .map(|(i, length)| {
                if i % 2 == 0 {
                    Run::Filled { id: i / 2, length }
                } else {
                    Run::Empty { length }
                }
            })
            .filter(|run| run.length() > 0)
            .collect_vec()
    }

    fn solve_part1(mut runs: Self::Parsed) -> impl Debug {
        // We want to move runs from the end into empty runs at the start
        // so to do that lets look for empties to fill until there aren't any
        for index in 0..runs.len() {
            if index >= runs.len() {
                break;
            }

            // Skip filled runs from this end
            let Run::Empty { length } = runs[index] else {
                continue;
            };

            // Trim any gaps at end of runs
            for index in (0..runs.len()).rev() {
                if runs[index].is_empty() {
                    runs.pop();
                } else {
                    break;
                }
            }

            // Get next file from end of runs
            let next_file = runs.pop().unwrap();
            let Run::Filled {
                id: file_id,
                length: file_length,
            } = next_file
            else {
                panic!();
            };

            // Is this run too long?
            if file_length > length {
                // Split it into two and
                // leave the correct amount behind
                let leftovers_length = file_length - length;
                let new_run = Run::Filled {
                    id: file_id,
                    length: leftovers_length,
                };
                runs.push(new_run);

                // Swap empty for filled
                runs[index] = Run::Filled {
                    id: file_id,
                    length,
                };
            }

            // Is this run too short to fill the gap?
            if file_length < length {
                // We need to split the empty into two
                let remaining_length = length - file_length;
                let new_empty = Run::Empty {
                    length: remaining_length,
                };

                // Swap empty for filled
                runs[index] = Run::Filled {
                    id: file_id,
                    length: length.min(file_length),
                };

                runs.insert(index + 1, new_empty);
            }

            // Is the run the perfect length? :relieved:
            if file_length == length {
                // Swap empty for filled
                runs[index] = Run::Filled {
                    id: file_id,
                    length,
                };
            }
        }

        compute_runs_checksum(&runs)
    }

    fn solve_part2(mut runs: Self::Parsed) -> impl Debug {
        // Find max id
        let max_id = runs
            .iter()
            .filter_map(|r| match r {
                Run::Empty { .. } => None,
                Run::Filled { id, .. } => Some(id),
            })
            .max()
            .unwrap();

        // Attempt to move each block once
        // starting with maximum id
        for current_id in (0..=*max_id).rev() {
            // Find where this block is
            let (index, run) = runs
                .iter()
                .enumerate()
                .find(|(_, r)| match r {
                    Run::Empty { .. } => false,
                    Run::Filled { id, .. } => *id == current_id,
                })
                .unwrap();

            // Its a file... I hope..
            let Run::Filled { length, id } = *run else {
                panic!();
            };

            // Now scan from the left and look for empty space
            if let Some((hole_index, hole)) = (0..index)
                .map(|i| runs[i])
                .enumerate()
                .find(|(_, run)| run.is_empty() && run.length() >= length)
            {
                // Swap the file for a hole
                runs[index] = Run::Empty { length };

                // And set the empty hole to be this
                runs[hole_index] = Run::Filled { id, length };

                // But was there extra room?
                if hole.length() > length {
                    runs.insert(
                        hole_index + 1,
                        Run::Empty {
                            length: hole.length() - length,
                        },
                    );
                }
            }
        }

        eprintln!("{}", runs.iter().map(|r| format!("{:?}", r)).join(""));
        compute_runs_checksum(&runs)
    }
}

fn main() {
    aoc::run_puzzle::<Day09>()
}
