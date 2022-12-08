use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

type Int = usize;
type InputType = HashMap<(Int, Int), Int>;

const SIZE: Int = 99;

fn read_input() -> InputType {
    let input = fs::read_to_string("./src/bin/day08/input.txt").unwrap();

    let mut map = HashMap::<(Int, Int), Int>::with_capacity(SIZE * SIZE);

    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x, y), c.to_digit(10).unwrap() as usize);
        }
    }

    map
}

fn get_visible_trees(trees: Vec<&Int>) -> HashSet<Int> {
    let mut visibles = HashSet::<Int>::with_capacity(SIZE * SIZE);

    let mut forward_min = trees[0];
    let mut backward_min = trees[SIZE - 1];

    for i in 0..SIZE {
        let forward_tree = trees[i];

        if forward_tree > forward_min {
            forward_min = forward_tree;
            visibles.insert(i);
        }

        let i = SIZE - i - 1;
        let backward_tree = trees[i];

        if backward_tree > backward_min {
            backward_min = backward_tree;
            visibles.insert(i);
        }
    }

    visibles
}

fn part1(input: InputType) -> Int {
    let mut visibles = HashSet::<(Int, Int)>::with_capacity(SIZE * SIZE);

    for (x, y) in (0..SIZE).cartesian_product(0..SIZE) {
        if x == 0 || x == SIZE - 1 || y == 0 || y == SIZE - 1 {
            visibles.insert((x, y));
        }
    }

    for y in 0..SIZE {
        let row = input
            .iter()
            .filter(|((_, yy), _)| yy == &y)
            .sorted_by(|((xx, _), _), ((xxx, _), _)| xx.cmp(xxx))
            .map(|(_, value)| value)
            .collect_vec();

        visibles.extend(get_visible_trees(row).iter().map(|&x| (x, y)));
    }

    for x in 0..SIZE {
        let col = input
            .iter()
            .filter(|((xx, _), _)| xx == &x)
            .sorted_by(|((_, yy), _), ((_, yyy), _)| yy.cmp(yyy))
            .map(|(_, value)| value)
            .collect_vec();

        visibles.extend(get_visible_trees(col).iter().map(|&y| (x, y)));
    }

    visibles.len()
}

fn part2(input: InputType) -> Int {
    let deltas = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut max = 0;

    for (x, y) in (1..(SIZE as isize - 1)).cartesian_product(1..(SIZE as isize - 1)) {
        let tree = input.get(&(x as usize, y as usize)).unwrap();
        let mut count = 1;

        for (dx, dy) in deltas.iter() {
            let mut multiplier = 1;

            let mut other = input.get(&((x + dx) as usize, (y + dy) as usize));

            while let Some(other_tree) = other {
                multiplier += 1;

                if tree <= other_tree {
                    break;
                }

                other = input.get(&(
                    (x + dx * multiplier) as usize,
                    (y + dy * multiplier) as usize,
                ));
            }

            count *= multiplier - 1;
        }

        if count > max {
            max = count;
        }
    }

    max as usize
}

pub fn main() {
    let input = read_input();

    let part1 = part1(input.clone());
    let part2 = part2(input);

    println!("--- Day 08 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 1719);
    assert_eq!(part2, 590824);
}
