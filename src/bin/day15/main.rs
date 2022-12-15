use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use itertools::Itertools;

type Int = i64;
type Point = (Int, Int);
type InputType = Vec<(Point, Point)>;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day15/input.txt")
        .unwrap()
        .trim()
        .lines()
        .filter_map(|line| {
            let split = line.split(['=', ',', ':']).collect_vec();

            let sx = split.get(1)?.parse().ok()?;
            let sy = split.get(3)?.parse().ok()?;
            let bx = split.get(5)?.parse().ok()?;
            let by = split.get(7)?.parse().ok()?;

            Some(((sx, sy), (bx, by)))
        })
        .collect()
}

fn get_distance(&(sx, sy): &Point, &(bx, by): &Point) -> Int {
    (sx.abs_diff(bx) + sy.abs_diff(by)) as Int
}

fn part1(input: InputType) -> usize {
    let target_y = 2000000;
    let mut covered = HashSet::<Int>::new();

    for (sensor, beacon) in input {
        let radius = get_distance(&sensor, &beacon);
        let vert_distance = sensor.1.abs_diff(target_y) as Int;

        if vert_distance > radius {
            continue;
        }

        covered
            .extend((sensor.0 - (radius - vert_distance))..=(sensor.0 + (radius - vert_distance)));

        if sensor.1 == target_y {
            covered.remove(&sensor.0);
        }

        if beacon.1 == target_y {
            covered.remove(&beacon.0);
        }
    }

    covered.len()
}

fn part2(input: InputType) -> Int {
    let radii = input
        .iter()
        .map(|&(sensor, beacon)| (sensor, get_distance(&sensor, &beacon)))
        .collect_vec();

    let mut x = 0;
    let mut y = 0;

    loop {
        let detecting_sensor = radii
            .iter()
            .find(|&&(sensor, radius)| get_distance(&(x, y), &sensor) <= radius);

        let Some((sensor, radius)) = detecting_sensor else {
            return x * 4000000 + y;
        };

        let vert_distance = sensor.1.abs_diff(y) as Int;
        x = sensor.0 + radius - vert_distance + 1;

        if x > 4000000 {
            x = 0;
            y += 1;
        }
    }
}

pub fn main() {
    let input = read_input();

    println!("--- Day 15 ---");

    let before = Instant::now();
    let part1 = part1(input.clone());
    println!("Part 1: {}, took {:.2?}", part1, before.elapsed());

    let before = Instant::now();
    let part2 = part2(input);
    println!("Part 2: {}, took {:.2?}", part2, before.elapsed());

    assert_eq!(part1, 4582667);
    assert_eq!(part2, 10961118625406);
}
