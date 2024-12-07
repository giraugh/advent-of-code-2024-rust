use std::{collections::HashSet, fmt::Debug};

use aoc::{
    pos,
    utils::{
        direction::Dir,
        grid::{Grid, GridPos},
    },
};

pub struct Day06;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Cell {
    Free,
    Obstacle,
}

#[derive(Clone, Debug)]
pub struct Map(Grid<Cell>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Continue {
    Continue,
    Stop,
}

impl Map {
    pub fn trace_from(
        &self,
        start_pos: GridPos,
        start_dir: Dir,
        mut transition_cb: impl FnMut(GridPos, Dir) -> Continue,
    ) {
        let mut pos = start_pos;
        let mut dir = start_dir;

        loop {
            if transition_cb(pos, dir) == Continue::Stop {
                break;
            };

            let check_pos = pos + dir.into();
            match self.0.get(check_pos) {
                None => break,
                Some(cell) => match cell {
                    Cell::Free => {
                        pos = check_pos;
                    }
                    Cell::Obstacle => dir = dir.turn_right(),
                },
            }
        }
    }
}

impl aoc::Puzzle for Day06 {
    type Parsed = (Map, GridPos);

    fn parse(input: &str) -> Self::Parsed {
        let mut start_pos = None;
        let grid = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| match c {
                        '#' => Cell::Obstacle,
                        '.' => Cell::Free,
                        '^' => {
                            start_pos = Some(pos!(col, row));
                            Cell::Free
                        }
                        _ => panic!("unexpected char {c}"),
                    })
                    .collect()
            })
            .collect();
        let grid = Grid::new(grid);

        (Map(grid), start_pos.unwrap())
    }

    fn solve_part1((grid, start_pos): Self::Parsed) -> impl Debug {
        let mut visited = HashSet::<GridPos>::new();
        grid.trace_from(start_pos, Dir::North, |pos, _| {
            visited.insert(pos);
            Continue::Continue
        });

        visited.len()
    }

    fn solve_part2((grid, start_pos): Self::Parsed) -> impl Debug {
        let mut obstacles: HashSet<GridPos> = Default::default();
        let mut visited = HashSet::new();

        // trace entire path
        grid.trace_from(start_pos, Dir::North, |pos, _| {
            visited.insert(pos);
            Continue::Continue
        });

        // Consider inserting an obstacle at each point
        let mut grid = grid;
        for pos in visited.into_iter() {
            // Skip start pos
            if pos == start_pos {
                continue;
            }

            // Would inserting an obstacle here cause a loop?
            grid.0.set(pos, Cell::Obstacle).unwrap();

            let mut does_loop = false;
            let mut past: HashSet<(GridPos, Dir)> = Default::default();
            grid.trace_from(start_pos, Dir::North, |pos, dir| {
                if past.contains(&(pos, dir)) {
                    does_loop = true;
                    Continue::Stop
                } else {
                    past.insert((pos, dir));
                    Continue::Continue
                }
            });

            if does_loop {
                obstacles.insert(pos);
            }

            grid.0.set(pos, Cell::Free).unwrap();
        }

        obstacles.len()
    }
}

fn main() {
    aoc::run_puzzle::<Day06>()
}
