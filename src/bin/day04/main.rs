use std::{fs, num::ParseIntError};

use regex::Regex;

type Int = u16;
type Ranges = ((Int, Int), (Int, Int));
type InputType = Vec<Ranges>;

fn read_input() -> InputType {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    fs::read_to_string("./src/bin/day04/input.txt")
        .unwrap()
        .trim()
        .lines()
        .flat_map(|line| re.captures(line))
        .flat_map(|cap| -> Result<Ranges, ParseIntError> {
            Ok((
                (cap[1].parse()?, cap[2].parse()?),
                (cap[3].parse()?, cap[4].parse()?),
            ))
        })
        .collect()
}

fn contains((x, y): &Ranges) -> bool {
    (x.0 >= y.0 && x.1 <= y.1) || (y.0 >= x.0 && y.1 <= x.1)
}

fn overlaps((x, y): &Ranges) -> bool {
    !(x.1 < y.0 || x.0 > y.1)
}

fn part1(input: InputType) -> Int {
    input.into_iter().filter(contains).count() as Int
}

fn part2(input: InputType) -> Int {
    input.into_iter().filter(overlaps).count() as Int
}

pub fn main() {
    let input = read_input();

    let part1 = part1(input.clone());
    let part2 = part2(input);

    println!("--- Day 04 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 542);
    assert_eq!(part2, 900);
}
