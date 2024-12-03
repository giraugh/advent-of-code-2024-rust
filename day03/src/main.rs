use std::fmt::Debug;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{self, value},
    IResult,
};

pub struct Day03;

#[derive(Debug, Clone)]
pub struct Mul(u32, u32);

#[derive(Debug, Clone)]
pub enum Command {
    Mul(Mul),
    Do,
    Dont,
}

impl Mul {
    pub fn result(&self) -> u32 {
        self.0 * self.1
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("mul(")(input)?;
        let (input, x) = nom::character::complete::u32(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y) = nom::character::complete::u32(input)?;
        let (input, _) = tag(")")(input)?;

        Ok((input, Mul(x, y)))
    }
}

impl Command {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            combinator::map(Mul::parse, Self::Mul),
            value(Self::Do, tag("do()")),
            value(Self::Dont, tag("don't()")),
        ))(input)
    }
}

impl aoc::Puzzle for Day03 {
    type Parsed = Vec<Command>;

    fn parse(input: &str) -> Self::Parsed {
        let mut input = input.to_string();
        let mut commands = Vec::new();
        while !input.is_empty() {
            // match a mul
            match Command::parse(&input) {
                Ok((rem, command)) => {
                    input = rem.to_string();
                    commands.push(command)
                }
                Err(_) => {
                    // slow smh
                    input.remove(0);
                }
            }
        }

        commands
    }

    fn solve_part1(input: Self::Parsed) -> impl Debug {
        input
            .iter()
            .filter_map(|r| match r {
                Command::Mul(mul) => Some(mul.result()),
                Command::Do => None,
                Command::Dont => None,
            })
            .sum::<u32>()
    }

    fn solve_part2(input: Self::Parsed) -> impl Debug {
        let mut enabled = true;
        let mut sum = 0;
        for command in input.iter() {
            match command {
                Command::Mul(mul) => {
                    if enabled {
                        sum += mul.result();
                    }
                }
                Command::Do => {
                    enabled = true;
                }
                Command::Dont => {
                    enabled = false;
                }
            }
        }

        sum
    }
}

fn main() {
    aoc::run_puzzle::<Day03>()
}
