extern crate core;

use std::collections::{HashMap, VecDeque};
use std::fs;

use itertools::Itertools;

type Int = u32;
type InputType = Vec<String>;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day07/input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.trim().to_string())
        .collect()
}

fn calc_path_map(input: InputType) -> HashMap<String, Vec<(String, String)>> {
    let mut map = HashMap::<String, Vec<_>>::new();
    let mut path = VecDeque::new();

    for line in input {
        if line.starts_with("$ cd") {
            if line.ends_with("..") {
                path.pop_back();
            } else {
                path.push_back(line.replace("$ cd ", ""));
            }
        } else if !line.starts_with('$') {
            let (first, mut second) = line
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect_tuple::<(String, String)>()
                .unwrap();

            let curr_dir = path.iter().join("");

            if first == "dir" {
                second = format!("{}{}", curr_dir, second);
            }

            map.entry(curr_dir).or_default().push((first, second));
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
            match child {
                (first, child_dir) if first == "dir" => {
                    if let Some(child_size) = sizes.get(&child_dir.to_string()) {
                        size += child_size;
                    } else {
                        queue.push_back(dir);
                        continue 'outer;
                    }
                }
                (file_size, _) => {
                    size += file_size.parse::<Int>().unwrap();
                }
            }
        }

        sizes.insert(dir.to_string(), size);
    }

    sizes
}

fn part1(input: InputType) -> Int {
    let sizes = calc_sizes(input);
    sizes.values().filter(|&size| size <= &100_000).sum()
}

fn part2(input: InputType) -> Int {
    let sizes = calc_sizes(input);
    let root_size = sizes.get(&"/".to_string()).unwrap();
    let need_to_be_removed = root_size - 40_000_000;

    *sizes
        .values()
        .sorted_unstable()
        .find(|&size| size >= &need_to_be_removed)
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
