use std::fs;

use itertools::Itertools;

type Int = u16;
type InputType = Vec<Vec<Int>>;

fn read_input() -> InputType {
    fs::read_to_string("./inputs/day03.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| -> Int {
                    let c = c as Int;

                    c - if (65..=90).contains(&c) {
                        38
                    } else if (97..=122).contains(&c) {
                        96
                    } else {
                        panic!()
                    }
                })
                .collect()
        })
        .collect()
}

fn part1(input: InputType) -> Int {
    input.iter().fold(0, |acc, line| {
        let (first, second) = line.split_at(line.len() / 2);

        acc + first.iter().find(|e| second.contains(e)).unwrap()
    })
}

fn part2(input: InputType) -> Int {
    input.chunks(3).fold(0, |acc, chunk| {
        let (first, second, third) = chunk.iter().collect_tuple().unwrap();

        acc + first
            .iter()
            .find(|e| second.contains(e) && third.contains(e))
            .unwrap()
    })
}

pub fn run() {
    let input = read_input();

    let part1 = part1(input.clone());
    let part2 = part2(input);

    println!("--- Day 03 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 7967);
    assert_eq!(part2, 2716);
}
