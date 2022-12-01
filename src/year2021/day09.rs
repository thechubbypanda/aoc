type Map = Vec<Vec<usize>>;
type Node = (usize, usize, usize);

fn flood_fill(map: &Map, w: usize, h: usize, fill: &mut Vec<Node>, (y, x, v): Node) {
    if v == 9 || fill.contains(&(y, x, v)) {
        return;
    };
    fill.push((y, x, v));
    if x != 0 {
        flood_fill(map, w, h, fill, (y, x - 1, map[y][x - 1]));
    }
    if x != w - 1 {
        flood_fill(map, w, h, fill, (y, x + 1, map[y][x + 1]));
    }
    if y != 0 {
        flood_fill(map, w, h, fill, (y - 1, x, map[y - 1][x]));
    }
    if y != h - 1 {
        flood_fill(map, w, h, fill, (y + 1, x, map[y + 1][x]));
    }
}

pub fn part1(input: String) -> usize {
    let input: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|s| s.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let w = input[0].len();
    let h = input.len();
    // println!("{:?}", input);
    let mut lowest: Vec<(usize, usize, usize)> = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let v = input[y][x];
            if x != 0 {
                if input[y][x - 1] <= v {
                    continue;
                }
            }
            if x != w - 1 {
                if input[y][x + 1] <= v {
                    continue;
                }
            }
            if y != 0 {
                if input[y - 1][x] <= v {
                    continue;
                }
            }
            if y != h - 1 {
                if input[y + 1][x] <= v {
                    continue;
                }
            }
            lowest.push((y, x, v));
        }
    }
    lowest.iter().map(|l| l.2 + 1).sum()
}

pub fn part2(input: String) -> usize {
    let input: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|s| s.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let w = input[0].len();
    let h = input.len();
    let mut lowest: Vec<Node> = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let v = input[y][x];
            if x != 0 {
                if input[y][x - 1] <= v {
                    continue;
                }
            }
            if x != w - 1 {
                if input[y][x + 1] <= v {
                    continue;
                }
            }
            if y != 0 {
                if input[y - 1][x] <= v {
                    continue;
                }
            }
            if y != h - 1 {
                if input[y + 1][x] <= v {
                    continue;
                }
            }
            lowest.push((y, x, v));
        }
    }

    let mut basins: Vec<Vec<Node>> = Vec::new();
    for (y, x, v) in lowest {
        let mut b = Vec::new();
        flood_fill(&input, w, h, &mut b, (y, x, v));
        basins.push(b);
    }
    let mut basins: Vec<usize> = basins.iter().map(|b| b.len()).collect();
    basins.sort();
    basins.iter().rev().take(3).product()
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
