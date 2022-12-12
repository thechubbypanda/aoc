use std::collections::{HashMap, HashSet};

type Point = (usize, usize);

fn neighbours((y, x): Point) -> Vec<Point> {
    let mut ns = vec![(y, x + 1), (y + 1, x)];
    if x != 0 {
        ns.push((y, x - 1));
    }
    if y != 0 {
        ns.push((y - 1, x));
    }
    ns
}

fn dijkstra_to_goal(map: HashMap<Point, u8>, start: Point, goal: Point) -> usize {
    let mut dist: HashMap<Point, usize> = map.keys().map(|p| (*p, usize::MAX)).collect();
    // let mut prev: HashMap<Point, Option<Point>> = map.keys().map(|p| (*p, None)).collect();
    let mut q: HashSet<Point> = map.keys().copied().collect();

    dist.insert(start, 0);

    while !q.is_empty() {
        let u = dist
            .iter()
            .filter(|(p, _v)| q.contains(p))
            .min_by(|(_, v1), (_, v2)| v1.cmp(v2))
            .map(|(p, _)| *p)
            .unwrap();
        if u == goal {
            return dist[&u];
        }
        q.remove(&u);

        for v in neighbours(u).into_iter() {
            if !q.contains(&v) {
                continue;
            }
            if *map.get(&v).unwrap() as isize - *map.get(&u).unwrap() as isize > 1 {
                continue;
            }
            let alt = dist[&u] + 1;
            if alt < dist[&v] {
                dist.insert(v, alt);
                // prev.insert(v, Some(u));
            }
        }
    }
    0
}

pub fn part1(input: String) -> usize {
    let mut map: HashMap<Point, u8> = input
        .lines()
        .map(|line| line.as_bytes().iter().enumerate().map(|(x, b)| (x, *b)))
        .enumerate()
        .flat_map(|(y, row)| row.map(move |(x, b)| ((y, x), b)))
        .collect();
    let start = map
        .iter()
        .find(|(_, v)| **v == b'S')
        .map(|(p, _)| *p)
        .unwrap();
    let goal = map
        .iter()
        .find(|(_, v)| **v == b'E')
        .map(|(p, _)| *p)
        .unwrap();

    map.insert(start, b'a');
    map.insert(goal, b'z');

    dijkstra_to_goal(map, start, goal)
}

pub fn part2(input: String) -> usize {
    let mut map: HashMap<Point, u8> = input
        .lines()
        .map(|line| line.as_bytes().iter().enumerate().map(|(x, b)| (x, *b)))
        .enumerate()
        .flat_map(|(y, row)| row.map(move |(x, b)| ((y, x), b)))
        .collect();
    map.insert(
        map.iter()
            .find(|(_, v)| **v == b'S')
            .map(|(p, _)| *p)
            .unwrap(),
        b'a',
    );
    let goal = map
        .iter()
        .find(|(_, v)| **v == b'E')
        .map(|(p, _)| *p)
        .unwrap();
    map.insert(goal, b'z');

    let mut dist: HashMap<Point, usize> = map.keys().map(|p| (*p, 99999999)).collect();
    // let mut prev: HashMap<Point, Option<Point>> = map.keys().map(|p| (*p, None)).collect();
    let mut q: HashSet<Point> = map.keys().copied().collect();

    dist.insert(goal, 0);

    while !q.is_empty() {
        let u = dist
            .iter()
            .filter(|(p, _v)| q.contains(p))
            .min_by(|(_, v1), (_, v2)| v1.cmp(v2))
            .map(|(p, _)| *p)
            .unwrap();
        q.remove(&u);

        for v in neighbours(u).into_iter() {
            if !q.contains(&v) {
                continue;
            }
            if *map.get(&u).unwrap() as isize - *map.get(&v).unwrap() as isize > 1 {
                continue;
            }
            let alt = dist[&u] + 1;
            if alt < dist[&v] {
                dist.insert(v, alt);
                // prev.insert(v, Some(u));
            }
        }
    }

    map.iter()
        .filter(|(p, v)| **v == b'a')
        .map(|(p, _)| dist[p])
        .min()
        .unwrap()
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
