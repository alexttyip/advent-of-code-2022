use std::{collections::HashSet, fs};

use itertools::Itertools;

type Int = usize;
type InputType = String;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day06/input.txt")
        .unwrap()
        .trim()
        .to_string()
}

fn find_unique_seq(input: &InputType, chunk_size: Int) -> Int {
    input
        .chars()
        .collect_vec()
        .windows(chunk_size)
        // Should've done this, feels nicer
        // .position(|window| window.iter().collect::<HashSet<_>>().len() == chunk_size)
        .position(|window| {
            let mut unique = HashSet::with_capacity(chunk_size);

            window.iter().all(move |x| unique.insert(x))
        })
        .unwrap()
        + chunk_size
}

fn part1(input: &InputType) -> Int {
    find_unique_seq(input, 4)
}

fn part2(input: &InputType) -> Int {
    find_unique_seq(input, 14)
}

pub fn main() {
    let input = read_input();

    let part1 = part1(&input);
    let part2 = part2(&input);

    println!("--- Day 06 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 1598);
    assert_eq!(part2, 2414);
}
