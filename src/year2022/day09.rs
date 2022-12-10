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
        .map(|line| {
            let mut split = line.split_whitespace();
            let dir = split.next().unwrap().chars().next().unwrap();
            let count = split.next().unwrap().parse::<usize>().unwrap();
            Move {
                dir: dir.into(),
                count,
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

fn iterate(head: &Pos, tail: &mut Pos) {
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

pub fn part1(input: String) -> usize {
    let moves = parse_input(input);

    let mut tail_visited: HashSet<Pos> = HashSet::new();
    let mut nodes = [Pos::new(); 2];
    tail_visited.insert(nodes[1]);

    for dir in moves
        .into_iter()
        .flat_map(|m| (0..m.count).map(move |_| m.dir))
    {
        nodes[0].translate(dir);
        let mut iter = nodes.iter_mut();
        let mut head = iter.next().unwrap();
        for tail in iter {
            iterate(head, tail);
            head = tail;
        }
        tail_visited.insert(nodes[1]);
    }
    tail_visited.len()
}

pub fn part2(input: String) -> usize {
    let moves = parse_input(input);

    let mut tail_visited: HashSet<Pos> = HashSet::new();
    let mut nodes = [Pos::new(); 10];
    tail_visited.insert(*nodes.last().unwrap());

    for dir in moves
        .into_iter()
        .flat_map(|m| (0..m.count).map(move |_| m.dir))
    {
        nodes[0].translate(dir);
        let mut iter = nodes.iter_mut();
        let mut head = iter.next().unwrap();
        for tail in iter {
            iterate(head, tail);
            head = tail;
        }
        tail_visited.insert(*nodes.last().unwrap());
    }
    tail_visited.len()
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
