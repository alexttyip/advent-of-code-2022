use std::{collections::HashSet, fs};

use itertools::Itertools;
use num::signum;

type Int = i16;
type InputType = Vec<(Knot, Int)>;
type Knot = (Int, Int);

fn get_deltas(dir: &str) -> Knot {
    match dir {
        "R" => (1, 0),
        "D" => (0, -1),
        "L" => (-1, 0),
        "U" => (0, 1),
        _ => panic!(),
    }
}

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day09/input.txt")
        .unwrap()
        .trim()
        .lines()
        .flat_map(|line| line.split_whitespace().collect_tuple())
        .flat_map(|(x, y)| Some((get_deltas(x), y.parse::<Int>().ok()?)))
        .collect()
}

fn update_tail(head: Knot, tail: &mut Knot) {
    if head.0.abs_diff(tail.0) < 2 && head.1.abs_diff(tail.1) < 2 {
        return;
    }

    tail.0 += signum(head.0 - tail.0);
    tail.1 += signum(head.1 - tail.1);
}

fn part1(input: InputType) -> usize {
    let mut head: Knot = (0, 0);
    let mut tail: Knot = (0, 0);
    let mut history = HashSet::<Knot>::from([tail]);

    for ((dx, dy), mag) in input {
        for _ in 0..mag {
            head.0 += dx;
            head.1 += dy;
            update_tail(head, &mut tail);

            history.insert(tail);
        }
    }

    history.len()
}

fn part2(input: InputType) -> usize {
    let mut rope: [Knot; 10] = [(0, 0); 10];

    let mut history = HashSet::<Knot>::from([(0, 0)]);

    for ((dx, dy), mag) in input {
        for _ in 0..mag {
            rope[0].0 += dx;
            rope[0].1 += dy;

            for i in 1..10 {
                update_tail(rope[i - 1], &mut rope[i]);
            }

            history.insert(rope[9]);
        }
    }

    history.len()
}

pub fn main() {
    let input = read_input();

    let part1 = part1(input.clone());
    let part2 = part2(input);

    println!("--- Day 09 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 6470);
    assert_eq!(part2, 2658);
}
