use std::collections::{HashMap, HashSet};

use aoc_lib::util::to_lines;

type Map = HashMap<Point, Node>;
type Point = (usize, usize);

#[derive(Copy, Clone, Debug)]
struct Node {
    val: usize,
    pos: Point,
    g: usize,
    parent: Option<Point>,
}

fn a(mut map: Map, w: usize, h: usize, start: Point, end: Point) -> Option<Vec<Node>> {
    let mut open: HashSet<Point> = HashSet::new();
    open.insert(start);

    let s = map.get_mut(&start).unwrap();
    s.g = 0;

    while !open.is_empty() {
        let q = open
            .iter()
            .map(|p| map.get(p).unwrap())
            .reduce(|min, n| {
                if n.g < min.g {
                    return n;
                }
                min
            })
            .unwrap()
            .pos;

        if q == end {
            let mut path = Vec::new();
            path.push(*map.get(&q).unwrap());
            while let Some(parent) = path.last().unwrap().parent {
                path.push(*map.get(&parent).unwrap());
            }
            path.reverse();
            return Some(path);
        }

        open.remove(&q);

        let mut successors: Vec<Point> = Vec::new();
        if q.0 > 0 {
            successors.push((q.0 - 1, q.1));
        }
        if q.1 > 0 {
            successors.push((q.0, q.1 - 1));
        }
        if q.0 < w - 1 {
            successors.push((q.0 + 1, q.1));
        }
        if q.1 < h - 1 {
            successors.push((q.0, q.1 + 1));
        }
        for s in successors {
            let g = map[&q].g + map[&s].val;
            if g < map[&s].g {
                let n = map.get_mut(&s).unwrap();
                n.parent = Some(q);
                n.g = g;
                open.insert(s);
            }
        }
    }
    None
}

pub fn part1(input: String) -> usize {
    let h = input.lines().count();
    let w = input.lines().next().unwrap().len();
    let input: Map = to_lines(&input)
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .enumerate()
                .map(move |(x, v)| {
                    (
                        (x, y),
                        Node {
                            val: v,
                            pos: (x, y),
                            g: usize::MAX,
                            parent: None,
                        },
                    )
                })
        })
        .collect();
    let start_v = input[&(0, 0)].val;
    let path = a(input, w, h, (0, 0), (w - 1, h - 1)).unwrap();
    path.iter().map(|n| n.val).sum::<usize>() - start_v
}

// Not mine, credit goes to the reddit mega thread somewhere (sorry)
pub fn part2(input: String) -> usize {
    let mut new = String::new();
    for y in 0..5 {
        for line in input.lines() {
            for x in 0..5 {
                for v in line.chars() {
                    let v = v.to_digit(10).unwrap() + y + x;
                    let v = if v > 9 { v - 9 } else { v };
                    new.push(char::from_digit(v, 10).unwrap());
                }
            }
            new.push('\n');
        }
    }
    part1(new)
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
}
