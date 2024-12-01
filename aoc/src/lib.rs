use std::{env, fmt, fs};

pub trait Puzzle {
    type Parsed: Clone;

    fn parse(input: &str) -> Self::Parsed;

    fn solve_part1(input: Self::Parsed) -> impl fmt::Debug;

    fn solve_part2(input: Self::Parsed) -> impl fmt::Debug;
}

pub fn run_puzzle<T: Puzzle>() {
    // Read input
    let input_path = env::args().nth(1).unwrap_or("./input.txt".to_owned());
    let input_text = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| panic!("Can't find AOC input file {}", &input_path));
    println!("[Read {}]", input_path);

    // Parse
    let parsed_input = T::parse(&input_text);
    println!("[Parsed input]");

    // Solve part 1
    let part_1_answer = T::solve_part1(parsed_input.clone());
    println!("[Part 1] {:?}", part_1_answer);

    // Solve part 2
    let part_2_answer = T::solve_part2(parsed_input);
    println!("[Part 2] {:?}", part_2_answer);
}
