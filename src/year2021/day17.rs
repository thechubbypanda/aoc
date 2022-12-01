use std::cmp::max;

// x1, y1, x2, y2
type Rect = (Point, Point);
type Point = (i32, i32);

fn get_rect(input: String) -> Rect {
    let mut input = input.split(", y=");
    let xs: Vec<i32> = input
        .next()
        .unwrap()
        .split("=")
        .skip(1)
        .next()
        .unwrap()
        .split("..")
        .map(|x| x.parse().unwrap())
        .collect();
    let ys: Vec<i32> = input
        .next()
        .unwrap()
        .split("..")
        .map(|y| y.parse().unwrap())
        .collect();
    ((xs[0], ys[0]), (xs[1], ys[1]))
}

fn max_h(u: Point, rect: Rect) -> Option<i32> {
    let mut v = u.clone();
    let mut max_y = 0;
    let mut p = (0, 0);
    while p.1 >= rect.0 .1 {
        if in_rect(&p, rect) {
            return Some(max_y);
        }
        p.0 += v.0;
        p.1 += v.1;
        max_y = max(max_y, p.1);
        v.0 = max(0, v.0 - 1);
        v.1 -= 1;
    }
    None
}

fn in_rect(p: &Point, r: Rect) -> bool {
    p.0 >= (r.0 .0) && p.0 <= (r.1 .0) && p.1 >= (r.0 .1) && p.1 <= (r.1 .1)
}

pub fn part1(input: String) -> i32 {
    let rect = get_rect(input);

    let mut hs: Vec<i32> = Vec::new();
    for vx in 0..=rect.1 .0 {
        for vy in rect.0 .1..=-rect.0 .1 {
            if let Some(h) = max_h((vx, vy), rect) {
                hs.push(h);
            }
        }
    }

    *hs.iter().max().unwrap()
}

pub fn part2(input: String) -> usize {
    let rect = get_rect(input);

    let mut hs: Vec<i32> = Vec::new();
    for vx in 0..=rect.1 .0 {
        for vy in rect.0 .1..=-rect.0 .1 {
            if let Some(h) = max_h((vx, vy), rect) {
                hs.push(h);
            }
        }
    }

    hs.iter().count()
}

#[cfg(test)]
mod tests {
    use aoc_lib::*;
    use super::*;

    #[test]
    fn test_part1() {
        run_test!(part1);
    }

    #[test]
    fn run_part1() {
        run_real!(part1);
    }
}
