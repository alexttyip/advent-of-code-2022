use std::collections::{HashSet, VecDeque};
use std::fs;

use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

type Int = i8;
type Cube = (Int, Int, Int);
type InputType = HashSet<Cube>;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day18/input.txt")
        .unwrap()
        .trim()
        .lines()
        .filter_map(|line| {
            line.trim()
                .split(',')
                .filter_map(|s| s.parse::<Int>().ok())
                .collect_tuple()
        })
        .collect()
}

fn get_all_neighbours((x, y, z): Cube, lower: Int, upper: Int) -> HashSet<Cube> {
    let bounds = lower..=upper;

    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
    .into_iter()
    .filter(|&(xx, yy, zz)| bounds.contains(&xx) && bounds.contains(&yy) && bounds.contains(&zz))
    .dedup()
    .collect()
}

fn count(input: InputType, part2: bool) -> usize {
    let MinMax(lower_bound, upper_bound) = input.iter().flat_map(|&(x, y, z)| [x, y, z]).minmax() else {
        panic!();
    };

    let mut visited = HashSet::<Cube>::from([(lower_bound, lower_bound, lower_bound)]);
    let mut queue = VecDeque::<Cube>::from([(lower_bound, lower_bound, lower_bound)]);

    let mut count = 0;

    while let Some(cube) = queue.pop_front() {
        let mut all_neighbours = get_all_neighbours(cube, lower_bound - 1, upper_bound + 1);

        if part2 || !input.contains(&cube) {
            count += (&all_neighbours & &input).len()
        }

        if part2 {
            all_neighbours = &all_neighbours - &input;
        }

        for space in all_neighbours {
            if !visited.contains(&space) {
                visited.insert(space);
                queue.push_back(space);
            }
        }
    }

    count
}

pub fn main() {
    let input = read_input();

    let part1 = count(input.clone(), false);
    let part2 = count(input, true);

    println!("--- Day 18 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 3466);
    assert_eq!(part2, 2012);
}
