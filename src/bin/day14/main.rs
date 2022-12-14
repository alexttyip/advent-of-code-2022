use std::cmp::Ordering::{Greater, Less};
use std::collections::{HashSet, VecDeque};
use std::fs;

use itertools::Itertools;

type Int = u16;
type Point = (Int, Int);
type InputType = HashSet<Point>;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day14/input.txt")
        .unwrap()
        .trim()
        .lines()
        .flat_map(|s| {
            s.split(" -> ")
                .filter_map(|point| {
                    point
                        .split(',')
                        .filter_map(|s| s.parse::<Int>().ok())
                        .collect_tuple::<(_, _)>()
                })
                .tuple_windows::<(_, _)>()
                .flat_map(|((x, y), (xx, yy))| match (x.cmp(&xx), y.cmp(&yy)) {
                    (Less, _) => (x..=xx).map(|i| (i, y)).collect_vec(),
                    (Greater, _) => (xx..=x).map(|i| (i, y)).collect_vec(),
                    (_, Less) => (y..=yy).map(|i| (x, i)).collect_vec(),
                    (_, Greater) => (yy..=y).map(|i| (x, i)).collect_vec(),
                    _ => panic!(),
                })
        })
        .collect()
}

fn simulate(mut occupied: InputType) -> (Int, Int) {
    let floor = occupied.iter().map(|(_, y)| y).max().unwrap() + 2;

    let mut part1 = None;
    let mut sand_count = 0;
    let mut history = VecDeque::<Point>::from([(500, 0)]);

    while let Some(&(x, y)) = history.back() {
        if y >= floor - 1 {
            // Reached floor
            if part1.is_none() {
                part1 = Some(sand_count);
            }
        } else if !occupied.contains(&(x, y + 1)) {
            // Straight down
            history.push_back((x, y + 1));
            continue;
        } else if !occupied.contains(&(x - 1, y + 1)) {
            // Down left
            history.push_back((x - 1, y + 1));
            continue;
        } else if !occupied.contains(&(x + 1, y + 1)) {
            // Down right
            history.push_back((x + 1, y + 1));
            continue;
        }

        // Settled
        sand_count += 1;
        occupied.insert((x, y));
        history.pop_back();
    }

    (part1.unwrap(), sand_count)
}

pub fn main() {
    let input = read_input();

    let (part1, part2) = simulate(input);

    println!("--- Day 14 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 832);
    assert_eq!(part2, 27601);
}
