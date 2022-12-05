use std::{collections::VecDeque, fs};

use itertools::Itertools;
use regex::Regex;

const NUM_COL: usize = 9;

type Move = (usize, usize, usize);
type InputType = ([VecDeque<char>; NUM_COL], Vec<Move>);

fn read_input() -> Option<InputType> {
    let input = fs::read_to_string("./src/bin/day05/input.txt").ok()?;
    let (stacks_input, moves_input) = input.split_once("\n\n")?;

    let mut stacks: [VecDeque<char>; NUM_COL] = Default::default();

    for line in stacks_input.lines() {
        for (col, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let ele = chunk.nth(1)?;

            if ele.is_whitespace() {
                continue;
            }

            if ele.is_numeric() {
                break;
            }

            stacks[col].push_front(ele);
        }
    }

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").ok()?;
    let moves = re
        .captures_iter(moves_input)
        .filter_map(|cap| {
            cap.iter()
                .skip(1)
                .take(3)
                .filter_map(|grp| grp?.as_str().parse::<usize>().ok())
                .collect_tuple::<Move>()
        })
        .map(|(mag, from, to)| (mag, from - 1, to - 1))
        .collect_vec();

    Some((stacks, moves))
}

fn part1((mut stacks, moves): InputType) -> String {
    for (mag, from, to) in moves {
        let drain_from = stacks[from].len() - mag;
        let drained = stacks[from].drain(drain_from..).rev().collect_vec();

        stacks[to].extend(drained);
    }

    stacks
        .into_iter()
        .flat_map(|stack| stack.back().copied())
        .join("")
}

fn part2((mut stacks, moves): InputType) -> String {
    for (mag, from, to) in moves {
        let drain_from = stacks[from].len() - mag;
        let drained = stacks[from].drain(drain_from..).collect_vec();

        stacks[to].extend(drained);
    }

    stacks
        .into_iter()
        .flat_map(|stack| stack.back().copied())
        .join("")
}

pub fn main() {
    let input = read_input().unwrap();

    let part1 = part1(input.clone());
    let part2 = part2(input);

    println!("--- Day 05 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    assert_eq!(part1, "BSDMQFLSP");
    assert_eq!(part2, "PGSQBFLDP");
}
