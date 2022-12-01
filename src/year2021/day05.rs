use std::{
    cmp::{max, min},
    collections::HashMap,
};

use aoc_lib::util::to_lines;

type Point = (usize, usize);
type Line = (Point, Point);

fn collect_lines(input: &String) -> Vec<Line> {
    to_lines(&input)
        .iter()
        .map(|l| l.split(" -> "))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
        .map(|(xs, ys)| (xs.split(','), ys.split(',')))
        .map(|(xs, ys)| {
            (
                xs.map(|x| x.parse().unwrap()).collect::<Vec<usize>>(),
                ys.map(|y| y.parse().unwrap()).collect::<Vec<usize>>(),
            )
        })
        .map(|(a, b)| ((a[0], a[1]), (b[0], b[1])))
        // .filter(|(a, b)| a.0 == b.0 || a.1 == b.1)
        .collect()
}

fn points_on_line(a: &Point, b: &Point) -> Vec<Point> {
    let mut ps = Vec::new();
    if a.0 == b.0 {
        return (min(a.1, b.1)..=max(a.1, b.1)).map(|y| (a.0, y)).collect();
    } else if a.1 == b.1 {
        return (min(a.0, b.0)..=max(a.0, b.0)).map(|x| (x, a.1)).collect();
    } else {
        let mut iter_x: Vec<usize> = (min(a.0, b.0)..=max(a.0, b.0)).collect();
        if b.0 > a.0 {
            iter_x = iter_x.into_iter().rev().collect();
        }
        let mut iter_y: Vec<usize> = (min(a.1, b.1)..=max(a.1, b.1)).collect();
        if b.1 > a.1 {
            iter_y = iter_y.into_iter().rev().collect();
        }
        for (x, y) in iter_x.iter().zip(iter_y.iter()) {
            ps.push((*x, *y));
        }
    }
    ps
}

pub fn part1(input: String) -> usize {
    let input: Vec<Line> = collect_lines(&input)
        .into_iter()
        .filter(|(a, b)| a.0 == b.0 || a.1 == b.1)
        .collect();
    let mut intersections: HashMap<Point, i32> = HashMap::new();
    for p in input.iter().map(|(a, b)| points_on_line(a, b)).flatten() {
        intersections.insert(p, intersections.get(&p).unwrap_or(&0) + 1);
    }
    intersections.values().filter(|v| **v >= 2).count()
}

pub fn part2(input: String) -> usize {
    let input: Vec<Line> = collect_lines(&input);
    let mut intersections: HashMap<Point, i32> = HashMap::new();
    for p in input.iter().map(|(a, b)| points_on_line(a, b)).flatten() {
        intersections.insert(p, intersections.get(&p).unwrap_or(&0) + 1);
    }
    intersections.values().filter(|v| **v >= 2).count()
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
