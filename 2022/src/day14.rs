use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::iter::repeat;

type Point = (usize, usize);

fn parse_input(input: String) -> HashSet<Point> {
    let mut rocks = HashSet::new();
    for line in input.lines() {
        for (a, b) in line
            .split(" -> ")
            .map(|split| {
                let mut pair = split.split(',').map(|v| v.parse::<usize>().unwrap());
                (pair.next().unwrap(), pair.next().unwrap())
            })
            .tuple_windows()
        {
            match a.0.cmp(&b.0) {
                Ordering::Less => rocks.extend(((a.0)..=(b.0)).zip(repeat(a.1))),
                Ordering::Greater => rocks.extend(((b.0)..=(a.0)).zip(repeat(a.1))),
                Ordering::Equal => match a.1.cmp(&b.1) {
                    Ordering::Less => rocks.extend(repeat(a.0).zip((a.1)..=(b.1))),
                    Ordering::Greater => rocks.extend(repeat(a.0).zip((b.1)..=(a.1))),
                    Ordering::Equal => {}
                },
            };
        }
    }
    rocks
}

fn find_new_pos(s: Point, filled: &HashSet<Point>) -> Option<Point> {
    [(s.0, s.1 + 1), (s.0 - 1, s.1 + 1), (s.0 + 1, s.1 + 1)]
        .into_iter()
        .find(|possibility| !filled.contains(possibility))
}

pub fn part1(input: String) -> usize {
    let mut filled = parse_input(input);
    let initial_filled = filled.len();
    let max_y = filled.iter().map(|(_, y)| *y).max().unwrap();
    'sands: loop {
        let mut s = (500, 0);
        while let Some(new_pos) = find_new_pos(s, &filled) {
            s = new_pos;
            if s.1 > max_y {
                break 'sands;
            }
        }
        filled.insert(s);
    }
    filled.len() - initial_filled
}

pub fn part2(input: String) -> usize {
    let mut filled = parse_input(input);
    let floor_y = filled.iter().map(|(_, y)| *y).max().unwrap() + 2;
    let initial_filled = filled.len();
    while !filled.contains(&(500, 0)) {
        let mut s = (500, 0);
        while let Some(new_pos) = find_new_pos(s, &filled) {
            if new_pos.1 < floor_y {
                s = new_pos;
            } else {
                break;
            }
        }
        filled.insert(s);
    }
    filled.len() - initial_filled
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::*;

    #[test]
    fn test_part1() {
        run_test!(part1);
    }

    #[test]
    fn run_part1() {
        run_real!(part1);
    }

    #[test]
    fn test_part2() {
        run_test!(part2);
    }

    #[test]
    fn run_part2() {
        run_real!(part2);
    }
}
