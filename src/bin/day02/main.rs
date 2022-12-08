use std::fs;

type Int = u16;
type InputType = Vec<String>;

const R: Int = 1;
const P: Int = 2;
const S: Int = 3;

const W: Int = 6;
const D: Int = 3;
const L: Int = 0;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day02/input.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn part1(input: InputType) -> Int {
    input.iter().fold(0, |acc, game| {
        acc + match game.as_str() {
            "A X" => R + D,
            "A Y" => P + W,
            "A Z" => S + L,
            "B X" => R + L,
            "B Y" => P + D,
            "B Z" => S + W,
            "C X" => R + W,
            "C Y" => P + L,
            "C Z" => S + D,
            _ => panic!(),
        }
    })
}

fn part2(input: InputType) -> Int {
    input.iter().fold(0, |acc, game| {
        acc + match game.as_str() {
            "A X" => S + L,
            "A Y" => R + D,
            "A Z" => P + W,
            "B X" => R + L,
            "B Y" => P + D,
            "B Z" => S + W,
            "C X" => P + L,
            "C Y" => S + D,
            "C Z" => R + W,
            _ => panic!(),
        }
    })
}

pub fn main() {
    let input = read_input();

    let part1 = part1(input.clone());
    let part2 = part2(input);

    println!("--- Day 02 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 13268);
    assert_eq!(part2, 15508);
}
