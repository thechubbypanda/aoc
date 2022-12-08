use aoc_lib::util::{to_lines, transpose};
use std::collections::HashSet;

pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    to_lines(input)
        .into_iter()
        .map(|line| line.as_bytes().iter().map(|c| *c - 48).collect())
        .collect()
}

fn visible(row: &Vec<u8>, reverse: bool) -> Vec<usize> {
    if reverse {
        let mut visible = vec![row.len() - 1];
        let mut tallest = &row[row.len() - 1];
        for (i, tree) in row.iter().enumerate().rev().skip(1) {
            if tree > tallest {
                tallest = tree;
                visible.push(row.len() - 1 - i);
            }
        }
        visible
    } else {
        let mut visible = vec![0];
        let mut tallest = &row[0];
        for (i, tree) in row.iter().enumerate().skip(1) {
            if tree > tallest {
                tallest = tree;
                visible.push(i);
            }
        }
        visible
    }
}

pub fn part1(input: String) -> usize {
    let tree_grid = parse_input(&input);
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    for (y, row) in tree_grid.iter().enumerate() {
        let mut xs: HashSet<usize> = HashSet::new();
        xs.extend(visible(row, false).iter());
        xs.extend(visible(row, true).iter());
        println!("y: {y} - {:?}", xs);
        for x in xs {
            set.insert((y, x));
        }
    }
    let tree_grid = transpose(&tree_grid);
    for (x, col) in tree_grid.iter().enumerate() {
        let mut ys: HashSet<usize> = HashSet::new();
        ys.extend(visible(col, false).iter());
        ys.extend(visible(col, true).iter());
        println!("x: {x} - {:?}", ys);
        for y in ys {
            set.insert((y, x));
        }
    }
    set.len()
}

pub fn part2(input: String) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::*;

    #[test]
    fn test_visible() {
        assert!(matches!(visible(&vec![2, 5, 5, 1, 2], false)[..], [0, 1]));
        assert!(matches!(visible(&vec![2, 5, 5, 1, 2], true)[..], [0, 2]));
        assert!(matches!(visible(&vec![7, 1, 3, 4, 9], false)[..], [0, 4]));
        assert!(matches!(visible(&vec![7, 1, 3, 4, 9], true)[..], [0]));
    }

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
