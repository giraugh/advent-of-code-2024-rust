use aoc::Puzzle;
use criterion::{criterion_group, criterion_main, Criterion};

use day11::Day11;

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");
    c.bench_function("part_1", |b| {
        b.iter(|| {
            let input = Day11::parse(input);
            Day11::solve_part1(input);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
