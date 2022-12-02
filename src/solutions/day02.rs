use std::fs;

use itertools::Itertools;

type Int = u16;
type Game = (Int, Int);
type InputType = Vec<Game>;

fn parse_char(s: &str) -> Int {
    match s {
        "A" | "X" => 0, // Rock / Lose
        "B" | "Y" => 1, // Paper / Draw
        "C" | "Z" => 2, // Scissors / Win
        _ => panic!("Not a valid shape"),
    }
}

fn read_input() -> InputType {
    fs::read_to_string("./inputs/day02.txt")
        .unwrap()
        .lines()
        .map(|s| s.split_ascii_whitespace())
        .map(|s| s.map(parse_char).collect_tuple().unwrap())
        .collect()
}

fn part1(input: InputType) -> Int {
    input.iter().fold(0, |acc, (elf, me)| {
        acc + if elf == &((me + 1) % 3) {
            0
        } else if me == elf {
            1
        } else {
            2
        } * 3 // Score for result
            + (me + 1) // Score for shape
    })
}

fn part2(input: InputType) -> Int {
    input.iter().fold(0, |acc, (elf, result)| {
        acc + *result * 3 // Score for result
            + match *result {
                0 => (elf + 3 - 1) % 3,
                1 => *elf,
                2 => (elf + 1) % 3,
                _ => panic!(),
            }
            + 1 // Score for shape
    })
}

pub fn run() {
    let input = read_input();

    let part1 = part1(input.clone());
    let part2 = part2(input);

    assert_eq!(part1, 13268);
    assert_eq!(part2, 15508);

    println!("--- Day 02 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
