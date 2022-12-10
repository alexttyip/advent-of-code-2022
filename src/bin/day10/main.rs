use std::fs;

type Int = i16;
type InputType = Vec<String>;
const SIZE: usize = 240;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day10/input.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn calc_cycles(input: &InputType) -> [Int; SIZE] {
    let mut cycles = [1; SIZE];

    let mut idx = 1;
    for line in input {
        if idx >= SIZE {
            break;
        }

        cycles[idx] = cycles[idx - 1];
        idx += 1;

        if line == "noop" {
            continue;
        }

        let mag = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<Int>()
            .unwrap();

        cycles[idx] = cycles[idx - 1] + mag;
        idx += 1;
    }

    cycles
}

fn part1(input: &InputType) -> Int {
    let cycles = calc_cycles(input);

    [20, 60, 100, 140, 180, 220]
        .iter()
        .fold(0, |acc, curr| acc + cycles[*curr - 1] * (*curr) as Int)
}

fn part2(input: &InputType) -> String {
    let cycles = calc_cycles(input);
    let mut output = "\n".to_string();

    cycles.chunks_exact(40).for_each(|chunk| {
        chunk
            .iter()
            .enumerate()
            .map(|(i, x)| x.abs_diff(i as Int) <= 1)
            .for_each(|b| output.push(if b { '#' } else { ' ' }));

        output.push('\n');
    });

    output
}

pub fn main() {
    let input = read_input();

    let part1 = part1(&input);
    let part2 = part2(&input);

    println!("--- Day 10 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 13060);
    // part2 == FJUBULRZ
}
