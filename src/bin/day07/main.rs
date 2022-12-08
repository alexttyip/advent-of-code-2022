extern crate core;

use std::collections::{HashMap, VecDeque};
use std::fs;

use itertools::Itertools;

type Int = u32;
type InputType = Vec<String>;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day07/input.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn calc_path_map(input: InputType) -> HashMap<String, Vec<(String, String)>> {
    let mut map = HashMap::<String, Vec<_>>::new();
    let mut path = VecDeque::new();

    for line in input {
        let line = line.split_whitespace().collect_vec();
        let curr_dir = path.iter().join("");

        match line.get(2) {
            Some(&"..") => {
                path.pop_back();
            }
            Some(&dir) => {
                path.push_back(dir.to_string());
            }
            None if line[0] == "dir" => {
                map.entry(curr_dir.to_string())
                    .or_default()
                    .push(("dir".to_string(), format!("{}{}", curr_dir, line[1])));
            }
            None if line[0] != "$" => {
                map.entry(curr_dir)
                    .or_default()
                    .push((line[0].to_string(), line[1].to_string()));
            }
            _ => {}
        }
    }

    map
}

fn calc_sizes(input: InputType) -> HashMap<String, Int> {
    let map = calc_path_map(input);
    let mut queue = map.keys().collect::<VecDeque<&String>>();
    let mut sizes = HashMap::with_capacity(queue.len());

    'outer: while let Some(dir) = queue.pop_front() {
        let mut size = 0;
        let children = map.get(dir).unwrap();
        for child in children {
            if child.0 == "dir" {
                if let Some(child_size) = sizes.get(&child.1) {
                    size += child_size;
                } else {
                    queue.push_back(dir);
                    continue 'outer;
                }
            } else {
                size += child.0.parse::<Int>().unwrap();
            }
        }

        sizes.insert(dir.to_string(), size);
    }

    sizes
}

fn part1(input: InputType) -> Int {
    let sizes = calc_sizes(input);
    sizes.values().filter(|&&size| size <= 100_000).sum()
}

fn part2(input: InputType) -> Int {
    let sizes = calc_sizes(input);
    let root_size = sizes.get(&"/".to_string()).unwrap();
    let min_removed = root_size - 40_000_000;

    *sizes
        .values()
        .filter(|&&size| size >= min_removed)
        .min()
        .unwrap()
}

pub fn main() {
    let input = read_input();

    let part1 = part1(input.clone());
    let part2 = part2(input);

    println!("--- Day 07 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 1447046);
    assert_eq!(part2, 578710);
}
