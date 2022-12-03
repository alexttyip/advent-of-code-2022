use std::fs;

type Int = u16;
type InputType = Vec<Int>;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day00/input.txt")
        .unwrap()
        .trim()
        .lines()
        .flat_map(|s| s.parse::<Int>())
        .collect()
}

fn part1(_input: InputType) -> Int {
    0
}

fn part2(_input: InputType) -> Int {
    0
}

pub fn main() {
    let input = read_input();

    let part1 = part1(input.clone());
    let part2 = part2(input);

    println!("--- Day 00 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    // assert_eq!(part1, 0);
    // assert_eq!(part2, 0);
}
