use std::collections::HashMap;

use aoc_lib::util::to_lines;

type Map = HashMap<(usize, usize), usize>;
type Key = (usize, usize);

fn get_map(input: &String) -> (Map, usize, usize) {
    let input: Vec<Vec<usize>> = to_lines(&input)
        .into_iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let h = input.len();
    let w = input[0].len();
    let mut map = Map::new();
    for (y, l) in input.clone().into_iter().enumerate() {
        for (x, v) in l.into_iter().enumerate() {
            map.insert((y, x), v);
        }
    }
    (map, h, w)
}

fn step(map: &mut Map, h: usize, w: usize, flashes: &mut usize) {
    for (_, v) in map.iter_mut() {
        *v += 1;
    }
    let mut tens: Vec<Key> = map
        .clone()
        .into_iter()
        .filter(|(_, v)| *v == 10)
        .map(|n| n.0)
        .collect();
    while tens.len() > 0 {
        let mut new = Vec::new();
        for k in tens.iter() {
            for q in get_adjacents(*k, h, w) {
                let v = map.get_mut(&q).unwrap();
                if *v == 10 {
                    continue;
                }
                *v += 1;
                if *v == 10 {
                    new.push(q);
                }
            }
        }
        tens.clear();
        tens.append(&mut new);
    }
    for (_, v) in map.iter_mut().filter(|(_, v)| **v == 10) {
        *flashes += 1;
        *v = 0;
    }
}

fn get_adjacents((y, x): Key, h: usize, w: usize) -> Vec<Key> {
    let y = y as i32;
    let x = x as i32;
    vec![
        (y - 1, x - 1),
        (y, x - 1),
        (y + 1, x - 1),
        (y - 1, x),
        (y + 1, x),
        (y - 1, x + 1),
        (y, x + 1),
        (y + 1, x + 1),
    ]
    .into_iter()
    .filter(|(y, x)| *y >= 0 && *x >= 0)
    .map(|k| (k.0 as usize, k.1 as usize))
    .filter(|(y, x)| *y < h && *x < w)
    .collect()
}

pub fn part1(input: String) -> usize {
    let (mut map, h, w) = get_map(&input);
    let mut flashes = 0;
    for _ in 0..100 {
        step(&mut map, h, w, &mut flashes);
    }
    flashes
}

pub fn part2(input: String) -> usize {
    let (mut map, h, w) = get_map(&input);
    let mut i = 0;
    loop {
        step(&mut map, h, w, &mut 0);
        i += 1;
        if map.iter().all(|(_, v)| *v == 0) {
            return i + 1;
        }
    }
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
