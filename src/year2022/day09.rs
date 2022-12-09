use aoc_lib::util::to_lines;
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
    to_lines(&input)
        .into_iter()
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

    if head.x - tail.x == 2 {
        tail.translate(Dir::Right);
        if head.y > tail.y {
            tail.translate(Dir::Up);
        } else if head.y < tail.y {
            tail.translate(Dir::Down);
        }
    } else if tail.x - head.x == 2 {
        tail.translate(Dir::Left);
        if head.y > tail.y {
            tail.translate(Dir::Up);
        } else if head.y < tail.y {
            tail.translate(Dir::Down);
        }
    } else if head.y - tail.y == 2 {
        tail.translate(Dir::Up);
        if head.x > tail.x {
            tail.translate(Dir::Right);
        } else if head.x < tail.x {
            tail.translate(Dir::Left);
        }
    } else if tail.y - head.y == 2 {
        tail.translate(Dir::Down);
        tail.translate(Dir::Up);
        if head.x > tail.x {
            tail.translate(Dir::Right);
        } else if head.x < tail.x {
            tail.translate(Dir::Left);
        }
    }
}

pub fn part1(input: String) -> usize {
    let moves = parse_input(input);

    let mut tail_visited: HashSet<Pos> = HashSet::new();
    let mut head = Pos::new();
    let mut tail = Pos::new();
    tail_visited.insert(tail);

    for m in moves.into_iter() {
        for _ in 0..(m.count) {
            iterate(&mut head, &mut tail, m.dir);
            // println!("{:?}", tail);
            tail_visited.insert(tail);
        }
    }
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
