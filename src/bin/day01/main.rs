use std::fs;

use itertools::Itertools;

type Int = u32;
type InputType = Vec<Int>;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day01/input.txt")
        .unwrap()
        .trim()
        .split("\n\n")
        .map(|elf| elf.lines().flat_map(|cal| cal.parse::<Int>()).sum::<Int>())
        .collect_vec()
}

fn part1(input: &InputType) -> Int {
    *input.iter().max().expect("Surely there's a max")
}

fn part2(input: &InputType) -> Int {
    input.iter().sorted_unstable().rev().take(3).sum()
}

pub fn main() {
    let input = read_input();

    let part1 = part1(&input);
    let part2 = part2(&input);

    println!("--- Day 01 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 68775);
    assert_eq!(part2, 202585);
}
