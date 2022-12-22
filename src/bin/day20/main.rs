use std::cmp::Ordering;
use std::fs;

use itertools::Itertools;

type Int = i64;
type InputType = Vec<Number>;

type Number = (usize, Int);

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day20/input.txt")
        .unwrap()
        .lines()
        .flat_map(|s| s.parse())
        .enumerate()
        .collect()
}

fn calc_dest(pos: usize, mut ele: Int, mut n: usize) -> usize {
    n -= 1;

    if ele.is_negative() {
        let n = n as Int;
        ele += (ele.abs() / n + 1) * n;
    }

    (pos + ele as usize + n - 1) % n + 1
}

fn calc(mut input: InputType, part1: bool) -> Int {
    if !part1 {
        for (_, v) in input.iter_mut() {
            *v *= 811589153
        }
    }

    let n = input.len();

    for _ in 0..(if part1 { 1 } else { 10 }) {
        for i in 0..n {
            let (current_position, &(_, ele)) = input
                .iter()
                .find_position(|(order, _)| order == &i)
                .unwrap();

            let dest = calc_dest(current_position, ele, n);

            match current_position.cmp(&dest) {
                Ordering::Less => {
                    for j in current_position..dest {
                        input.swap(j, j + 1);
                    }
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    for j in (dest..current_position).rev() {
                        input.swap(j, j + 1);
                    }
                }
            }
        }
    }

    let zero_pos = input.iter().position(|&ele| ele.1 == 0).unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|i| input[(i + zero_pos) % n].1)
        .sum()
}

pub fn main() {
    let input = read_input();

    let part1 = calc(input.clone(), true);
    let part2 = calc(input, false);

    println!("--- Day 20 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 13522);
    assert_eq!(part2, 17113168880158);
}
