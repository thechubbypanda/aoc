use std::collections::HashSet;

use aoc_lib::util::to_lines;

type Image = HashSet<Point>;
type Point = (i32, i32);

fn get_input(input: String) -> (Vec<bool>, Image) {
    let mut input = input.split("\n\n").map(|block| block.trim());
    let algorithm: Vec<bool> = input
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => unreachable!(),
        })
        .collect();
    let image = &input.next().unwrap().to_string();
    let image: Image = to_lines(&image)
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .flatten()
        .collect();
    (algorithm, image)
}

fn get_adjacents((x, y): Point) -> Vec<Point> {
    vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .into_iter()
    .collect()
}

fn print(image: &Image) {
    let x_min = image.iter().map(|p| p.0).min().unwrap();
    let x_max = image.iter().map(|p| p.0).max().unwrap();
    let y_min = image.iter().map(|p| p.1).min().unwrap();
    let y_max = image.iter().map(|p| p.1).max().unwrap();
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            print!("{}", if image.contains(&(x, y)) { '#' } else { '.' });
        }
        println!("")
    }
    println!("")
}

fn solve(algorithm: Vec<bool>, mut image: Image, iterations: usize) -> usize {
    print(&image);
    let mut void: bool = false;
    for _ in 0..iterations {
        let mut new_image = Image::new();
        let x_min = image.iter().map(|p| p.0).min().unwrap();
        let x_max = image.iter().map(|p| p.0).max().unwrap();
        let y_min = image.iter().map(|p| p.1).min().unwrap();
        let y_max = image.iter().map(|p| p.1).max().unwrap();
        for x in x_min - 1..=x_max + 1 {
            for y in y_min - 1..=y_max + 1 {
                let around = get_adjacents((x, y))
                    .iter()
                    .map(|(p, q)| {
                        if *p <= x_max && *p >= x_min && *q <= y_max && *q >= y_min {
                            image.contains(&(*p, *q))
                        } else {
                            void
                        }
                    })
                    .map(|c| match c {
                        false => '0',
                        true => '1',
                    })
                    .collect::<String>();
                let index = usize::from_str_radix(&around, 2).unwrap();
                if algorithm[index] {
                    new_image.insert((x, y));
                }
            }
        }
        image = new_image;
        void = if void { algorithm[511] } else { algorithm[0] }
    }
    print(&image);
    image.len()
}

pub fn part1(input: String) -> usize {
    let (algorithm, image) = get_input(input);
    solve(algorithm, image, 2)
}

pub fn part2(input: String) -> usize {
    let (algorithm, image) = get_input(input);
    solve(algorithm, image, 50)
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
