use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Debug)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            'U' => Dir::Up,
            'L' => Dir::Left,
            'D' => Dir::Down,
            'R' => Dir::Right,
            _ => panic!("Unknown direction: {}", value),
        }
    }
}

struct Move {
    dir: Dir,
    count: usize,
}

impl Debug for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self.dir, self.count)
    }
}

fn parse_input(input: String) -> Vec<Move> {
    input
        .lines()
        .filter_map(|line| {
            if let [d, _, c] = line.as_bytes() {
                Some(Move {
                    dir: (*d as char).into(),
                    count: (*c - b'0') as usize,
                })
            } else {
                None
            }
        })
        .collect()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new() -> Self {
        Pos { x: 0, y: 0 }
    }

    fn translate(&mut self, dir: Dir) {
        match dir {
            Dir::Up => self.y += 1,
            Dir::Left => self.x -= 1,
            Dir::Down => self.y -= 1,
            Dir::Right => self.x += 1,
        }
    }
}

fn iterate(head: &mut Pos, tail: &mut Pos, dir: Dir) {
    head.translate(dir);

    if head.x.abs_diff(tail.x) >= 2 {
        if head.x > tail.x {
            tail.translate(Dir::Right);
        } else {
            tail.translate(Dir::Left);
        }
        if head.y > tail.y {
            tail.translate(Dir::Up);
        } else if head.y < tail.y {
            tail.translate(Dir::Down);
        }
    } else if head.y.abs_diff(tail.y) >= 2 {
        if head.y > tail.y {
            tail.translate(Dir::Up);
        } else {
            tail.translate(Dir::Down);
        }
        if head.x > tail.x {
            tail.translate(Dir::Right);
        } else if head.x < tail.x {
            tail.translate(Dir::Left);
        }
    }
}

fn visualise(iter: impl Iterator<Item = (i32, i32)>) {
    let points = iter.collect::<Vec<_>>();
    let (min_x, max_x, min_y, max_y) =
        points
            .iter()
            .copied()
            .fold((0, 0, 0, 0), |(min_x, max_x, min_y, max_y), (x, y)| {
                (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y))
            });

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if x == 0 && y == 0 {
                print!("s");
            } else {
                match points
                    .iter()
                    .enumerate()
                    .find(|(_, p)| p.0 == x && p.1 == y)
                    .map(|(i, _)| i)
                {
                    None => print!("."),
                    Some(ix) => print!("#"),
                }
            }
        }
        println!();
    }
}

pub fn part1(input: String) -> usize {
    let moves = parse_input(input);

    let mut tail_visited: HashSet<Pos> = HashSet::new();
    let mut head = Pos::new();
    let mut tail = Pos::new();
    tail_visited.insert(tail);

    for dir in moves
        .into_iter()
        .flat_map(|m| (0..m.count).map(move |_| m.dir))
    {
        iterate(&mut head, &mut tail, dir);
        tail_visited.insert(tail);
    }
    visualise(tail_visited.iter().map(|pos| (pos.x as i32, pos.y as i32)));
    tail_visited.len()
}

pub fn part2(input: String) -> usize {
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
