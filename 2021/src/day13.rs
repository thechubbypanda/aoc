use std::collections::HashSet;

use aoc_lib::util::to_lines;

type Point = (usize, usize);
type Fold = (char, usize);

fn read_input(input: &String) -> (HashSet<Point>, Vec<Fold>) {
    let mut input = input.split("\n\n").into_iter();
    let (paper, folds) = (input.next().unwrap(), input.next().unwrap());
    let paper: HashSet<Point> = to_lines(&paper.to_string())
        .iter()
        .map(|l| l.split(','))
        .map(|mut c| (c.next().unwrap(), c.next().unwrap()))
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    let folds: Vec<Fold> = to_lines(&folds.to_string())
        .iter()
        .map(|l| l.split_whitespace().skip(2).next().unwrap().split("="))
        .map(|mut s| {
            (
                s.next().unwrap().to_string().chars().next().unwrap(),
                s.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    // println!("{:?}", paper);
    // println!("{:?}", folds);

    (paper, folds)
}

fn print(paper: &HashSet<Point>) {
    for y in 0..6 {
        for x in 0..39 {
            if paper.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("")
    }
    println!("")
}

pub fn part1(input: String) -> usize {
    let (mut paper, folds) = read_input(&input);

    let (axis, v) = folds[0];
    if axis == 'x' {
        for (x, y) in paper.clone().iter().filter(|(x, _)| *x > v) {
            paper.insert((x - (x - v) * 2, *y));
            paper.remove(&(*x, *y));
        }
    } else {
        for (x, y) in paper.clone().iter().filter(|(_, y)| *y > v) {
            paper.insert((*x, y - (y - v) * 2));
            paper.remove(&(*x, *y));
        }
    }

    paper.len()
}

pub fn part2(input: String) -> usize {
    let (mut paper, folds) = read_input(&input);

    for (axis, v) in folds {
        if axis == 'x' {
            for (x, y) in paper.clone().iter().filter(|(x, _)| *x > v) {
                paper.remove(&(*x, *y));
                paper.insert((x - (x - v) * 2, *y));
            }
        } else {
            for (x, y) in paper.clone().iter().filter(|(_, y)| *y > v) {
                paper.remove(&(*x, *y));
                paper.insert((*x, y - (y - v) * 2));
            }
        }
    }

    print(&paper);

    0
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
