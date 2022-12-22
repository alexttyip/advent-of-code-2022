use std::collections::BTreeMap;
use std::fs;

use itertools::Itertools;

use crate::Move::{Anticlockwise, Clockwise, Steps};
use crate::PointType::{Open, Wall};

type Int = i32;
type Point = (Int, Int);
type InputType = (BTreeMap<Point, PointType>, Vec<Move>);

enum PointType {
    Open,
    Wall,
}

enum Move {
    Steps(Int),
    Clockwise,
    Anticlockwise,
}

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day22/input.txt").unwrap();

    let (map, moves) = file.split("\n\n").collect_tuple::<(_, _)>().unwrap();

    let map = map
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, square)| {
                if square == ' ' {
                    return None;
                }

                Some((
                    ((x + 1) as Int, (y + 1) as Int),
                    match square {
                        '.' => Open,
                        '#' => Wall,
                        _ => unreachable!(),
                    },
                ))
            })
        })
        .collect();

    let moves = moves
        .trim()
        .chars()
        .group_by(|c| c.is_numeric())
        .into_iter()
        .map(|(is_steps, mut group)| {
            if is_steps {
                Steps(group.collect_vec().iter().join("").parse().unwrap())
            } else {
                match group.next() {
                    Some('R') => Clockwise,
                    Some('L') => Anticlockwise,
                    _ => unreachable!(),
                }
            }
        })
        .collect();

    (map, moves)
}

const DELTAS: [Point; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn wrap_part1(map: &BTreeMap<Point, PointType>, orientation: &mut usize, point: &mut Point) {
    // Off the map
    let (x, y) = &point;

    let new_point = match orientation {
        0 => map.iter().find(|((_, yy), _)| yy == y),
        1 => map.iter().find(|((xx, _), _)| xx == x),
        2 => map.iter().rfind(|((_, yy), _)| yy == y),
        3 => map.iter().rfind(|((xx, _), _)| xx == x),
        _ => unreachable!(),
    };

    if let Some((&new_point, Open)) = new_point {
        *point = new_point;
    }
}

fn wrap_part2(map: &BTreeMap<Point, PointType>, orientation: &mut usize, point: &mut Point) {
    // Off the map
    let (x, y) = *point;

    let (new_x, new_y, new_o) = match orientation {
        0 => {
            if y <= 50 {
                // Edge 3
                (100, 151 - y, 2)
            } else if y <= 100 {
                // Edge 4
                (y + 50, 50, 3)
            } else if y <= 150 {
                // Edge 3
                (150, 151 - y, 2)
            } else if y <= 200 {
                // Edge 7
                (y - 100, 150, 3)
            } else {
                unreachable!();
            }
        }

        1 => {
            if x <= 50 {
                // Edge 2
                (x + 100, 1, 1)
            } else if x <= 100 {
                // Edge 7
                (50, x + 100, 2)
            } else if x <= 150 {
                // Edge 4
                (100, x - 50, 2)
            } else {
                unreachable!();
            }
        }

        2 => {
            if y <= 50 {
                // Edge 5
                (1, 151 - y, 0)
            } else if y <= 100 {
                // Edge 6
                (y - 50, 101, 1)
            } else if y <= 150 {
                // Edge 5
                (51, 151 - y, 0)
            } else if y <= 200 {
                // Edge 1
                (y - 100, 1, 1)
            } else {
                unreachable!();
            }
        }

        3 => {
            if x <= 50 {
                // Edge 6
                (51, x + 50, 0)
            } else if x <= 100 {
                // Edge 1
                (1, x + 100, 0)
            } else if x <= 150 {
                // Edge 2
                (x - 100, 200, 3)
            } else {
                unreachable!();
            }
        }
        _ => unreachable!(),
    };

    if let Some(Open) = map.get(&(new_x, new_y)) {
        *point = (new_x, new_y);
        *orientation = new_o;
    }
}

fn do_move(
    map: &BTreeMap<Point, PointType>,
    orientation: &mut usize,
    point: &mut Point,
    part1: bool,
) {
    let (dx, dy) = DELTAS[*orientation];

    let (x, y) = point;

    match map.get(&(*x + dx, *y + dy)) {
        Some(Open) => {
            *x += dx;
            *y += dy;
        }
        None => {
            if part1 {
                wrap_part1(map, orientation, point)
            } else {
                wrap_part2(map, orientation, point)
            }
        }
        _ => {}
    }
}

fn simulate((map, moves): InputType) -> (Int, Int) {
    let mut orientation1 = 0;
    let mut point1 = *map.keys().find(|(_, y)| y == &1).unwrap();

    let mut orientation2 = 0;
    let mut point2 = point1;

    for movement in moves {
        match movement {
            Steps(steps) => {
                for _ in 0..steps {
                    do_move(&map, &mut orientation1, &mut point1, true);
                    do_move(&map, &mut orientation2, &mut point2, false);
                }
            }
            Clockwise => {
                orientation1 += 1;
                orientation1 %= 4;
                orientation2 += 1;
                orientation2 %= 4;
            }
            Anticlockwise => {
                orientation1 += 3;
                orientation1 %= 4;
                orientation2 += 3;
                orientation2 %= 4;
            }
        }
    }

    (
        point1.1 * 1000 + point1.0 * 4 + orientation1 as Int,
        point2.1 * 1000 + point2.0 * 4 + orientation2 as Int,
    )
}

pub fn main() {
    let input = read_input();

    let (part1, part2) = simulate(input);

    println!("--- Day 22 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 67390);
    assert_eq!(part2, 95291);
}
