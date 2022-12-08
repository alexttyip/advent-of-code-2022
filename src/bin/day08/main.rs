use std::{
    collections::{BTreeMap, HashSet},
    fs,
};

use itertools::Itertools;

type Int = usize;
type InputType = BTreeMap<(Int, Int), Int>;

const SIZE: Int = 99;

fn read_input() -> InputType {
    let input = fs::read_to_string("./src/bin/day08/input.txt").unwrap();

    let mut map = BTreeMap::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x, y), c.to_digit(10).unwrap() as usize);
        }
    }

    map
}

fn get_visible_trees(trees: [&Int; SIZE]) -> HashSet<Int> {
    let mut visibles = HashSet::with_capacity(SIZE * SIZE);

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
    let mut visibles = HashSet::with_capacity(SIZE * SIZE);

    for (i, j) in (0..SIZE).cartesian_product([0, SIZE - 1]) {
        visibles.insert((i, j));
        visibles.insert((j, i));
    }

    let mut row = [&0usize; SIZE];
    let mut col = [&0usize; SIZE];

    for i in 1..(SIZE - 1) {
        input
            .iter()
            .filter(|&(&(_, yy), _)| yy == i)
            .enumerate()
            .for_each(|(idx, pair)| row[idx] = pair.1);

        input
            .range((i, 0)..(i, SIZE))
            .enumerate()
            .for_each(|(idx, pair)| col[idx] = pair.1);

        visibles.extend(get_visible_trees(row).iter().map(|&x| (x, i)));
        visibles.extend(get_visible_trees(col).iter().map(|&y| (i, y)));
    }

    visibles.len()
}

fn part2(input: InputType) -> isize {
    let deltas = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut max = 0;

    let get_input = |x: isize, y: isize| -> Option<&usize> { input.get(&(x as usize, y as usize)) };
    let range_without_edge = 1..(SIZE as isize - 1);

    for (x, y) in range_without_edge
        .clone()
        .cartesian_product(range_without_edge)
    {
        let tree = get_input(x, y).unwrap();
        let mut count = 1;

        for (dx, dy) in deltas.iter() {
            let mut multiplier = 1;

            let mut other = get_input(x + dx, y + dy);

            while let Some(other_tree) = other {
                multiplier += 1;

                if tree <= other_tree {
                    break;
                }

                other = get_input(x + dx * multiplier, y + dy * multiplier);
            }

            count *= multiplier - 1;
        }

        if count > max {
            max = count;
        }
    }

    max
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
