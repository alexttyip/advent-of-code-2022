use std::fs;

use itertools::Itertools;

type InputType = Vec<u32>;

fn read_input() -> InputType {
    fs::read_to_string("./inputs/day01.txt")
        .unwrap()
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|cal| cal.parse::<u32>().expect("Ahh it's not an int"))
                .sum()
        })
        .collect_vec()
}

fn part1(input: InputType) -> u32 {
    *input.iter().max().expect("Surely there's a max")
}

fn part2(input: InputType) -> u32 {
    input.iter().sorted().rev().take(3).sum()
}

pub fn run() {
    let input = read_input();

    let part1 = part1(input.clone());
    let part2 = part2(input);

    assert_eq!(part1, 68775);
    assert_eq!(part2, 202585);

    println!("--- Day 01 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
