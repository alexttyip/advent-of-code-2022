use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs;

use itertools::Itertools;

type InputType = Vec<(String, String)>;

fn read_input() -> InputType {
    let input = fs::read_to_string("./src/bin/day13/input.txt").unwrap();

    input
        .split("\n\n")
        .filter_map(|pair| {
            pair.lines()
                .map(|line| line.to_string())
                .collect_tuple::<(_, _)>()
        })
        .collect_vec()
}

fn surround_with_brackets(string: &str) -> String {
    format!("[{string}]")
}

fn take_first(line: &String) -> (String, String) {
    if line.len() < 3 {
        return (String::new(), String::new());
    }

    let line = &line[1..line.len() - 1];

    let mut split_idx = 0;
    let mut bracket_level = 0;
    for c in line.chars() {
        if bracket_level == 0 && c == ',' {
            break;
        }

        bracket_level += match c {
            '[' => 1,
            ']' => -1,
            _ => 0,
        };

        split_idx += 1;
    }

    let (first, mut second) = line.split_at(split_idx);
    second = match second.split_once(',') {
        Some((_, second)) => second,
        None => "",
    };

    (first.to_string(), surround_with_brackets(second))
}

fn compare(left: &String, right: &String) -> Ordering {
    if left == right {
        return Equal;
    }

    let (left_head, left_tail) = &take_first(left);
    let (right_head, right_tail) = &take_first(right);

    if left_head == right_head {
        return compare(left_tail, right_tail);
    }

    if left_head.is_empty() {
        return Less;
    }

    if right_head.is_empty() {
        return Greater;
    }

    let head_result = match (left_head.parse::<u8>(), right_head.parse::<u8>()) {
        (Ok(left), Ok(right)) => left.cmp(&right),
        (Ok(_), _) => compare(&surround_with_brackets(left_head), right_head),
        (_, Ok(_)) => compare(left_head, &surround_with_brackets(right_head)),
        _ => compare(left_head, right_head),
    };

    if head_result == Equal {
        return compare(left_tail, right_tail);
    }

    head_result
}

fn part1(input: InputType) -> usize {
    input
        .iter()
        .positions(|(left, right)| compare(left, right) == Less)
        .map(|idx| idx + 1)
        .sum()
}

fn part2(input: InputType) -> usize {
    let targets = ["[[2]]", "[[6]]"].map(|s| s.to_string());

    input
        .into_iter()
        .flat_map(|(left, right)| [left, right])
        .chain(targets.clone())
        .sorted_unstable_by(compare)
        .positions(|v| targets.contains(&v))
        .map(|idx| idx + 1)
        .product()
}

pub fn main() {
    let input = read_input();

    let part1 = part1(input.clone());
    let part2 = part2(input);

    println!("--- Day 13 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 6656);
    assert_eq!(part2, 19716);
}
