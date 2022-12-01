use std::fs;

type InputType = Vec<u32>;

fn read_input() -> InputType {
    fs::read_to_string("./inputs/day14.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1(_input: InputType) -> u32 {
    0
}

fn part2(_input: InputType) -> u32 {
    0
}

pub fn run() {
    let input = read_input();

    println!("--- Day 14 ---");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}
