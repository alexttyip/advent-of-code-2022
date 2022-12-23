extern crate core;

use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

type Int = i32;
type Point = (Int, Int);
type InputType = HashSet<Point>;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day23/input.txt")
        .unwrap()
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim().chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as Int, y as Int))
                } else {
                    None
                }
            })
        })
        .collect()
}

const NUM_DIRECTIONS: usize = 4;
const DELTAS: [[Point; 3]; NUM_DIRECTIONS] = [
    [(0, -1), (-1, -1), (1, -1)],
    [(0, 1), (-1, 1), (1, 1)],
    [(-1, 0), (-1, -1), (-1, 1)],
    [(1, 0), (1, -1), (1, 1)],
];

fn calc_proposal(elves: &InputType, (x, y): &Point, first_direction: usize) -> Option<Point> {
    let mut proposal = None;
    let mut has_neighbour = false;

    for i in 0..NUM_DIRECTIONS {
        if has_neighbour && proposal.is_some() {
            return proposal;
        }

        let neighbours =
            DELTAS[(first_direction + i) % NUM_DIRECTIONS].map(|(dx, dy)| (x + dx, y + dy));

        if neighbours
            .iter()
            .all(|neighbour| !elves.contains(neighbour))
        {
            if proposal.is_none() {
                proposal = Some(neighbours[0]);
            }
        } else {
            has_neighbour = true;
        }
    }

    if has_neighbour {
        proposal
    } else {
        None
    }
}

fn simulate(mut elves: InputType) -> (Int, Int) {
    let mut direction = 0;
    let mut part1 = HashSet::new();
    let mut part2 = 0;

    loop {
        if part2 == 10 {
            part1 = elves.clone();
        }

        let mut moves = HashMap::<Point, Point>::new();

        for elf in &elves {
            let proposal = calc_proposal(&elves, elf, direction);

            if let Some(new_point) = proposal {
                if moves.remove(&new_point).is_none() {
                    moves.insert(new_point, *elf);
                }
            }
        }

        part2 += 1;

        if moves.is_empty() {
            break;
        }

        for (new_point, old_point) in moves {
            elves.remove(&old_point);
            elves.insert(new_point);
        }

        direction = (direction + 1) % NUM_DIRECTIONS;
    }

    let MinMax(&min_x, &max_x) = part1.iter().map(|(x, _)| x).minmax() else {
        panic!();
    };

    let MinMax(&min_y, &max_y) = part1.iter().map(|(_, y)| y).minmax() else {
        panic!();
    };

    let count = (max_x - min_x + 1) * (max_y - min_y + 1) - part1.len() as Int;

    (count, part2)
}

pub fn main() {
    let input = read_input();

    let before = Instant::now();
    let (part1, part2) = simulate(input);
    let took = before.elapsed();

    println!("--- Day 23 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Took: {:.2?}", took);

    assert_eq!(part1, 4195);
    assert_eq!(part2, 1069);
}
