use std::fs;

use num::Integer;

type Int = u64;
type InputType = Vec<String>;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day25/input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}

fn decode(string: &str) -> Int {
    let mut num = 0i64;
    for c in string.chars() {
        num *= 5;

        num += match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => panic!(),
        }
    }

    num as Int
}

const CONVERSION: [char; 5] = ['0', '1', '2', '=', '-'];

fn encode(mut num: Int) -> String {
    let mut encoded = String::new();

    while num >= 5 {
        let (div, rem) = num.div_rem(&5);
        encoded.push(CONVERSION[rem as usize]);
        num = div + u64::from(rem > 2);
    }

    if num != 0 {
        encoded.push(CONVERSION[num as usize])
    }

    encoded.chars().rev().collect()
}

fn part1(input: InputType) -> String {
    encode(input.iter().map(|line| decode(line)).sum())
}

pub fn main() {
    let input = read_input();

    let part1 = part1(input);

    println!("--- Day 25 ---");
    println!("Part 1: {}", part1);

    assert_eq!(part1, "2=0=02-0----2-=02-10");
}
