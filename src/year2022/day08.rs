use aoc_lib::util::{to_lines, transpose};
use std::collections::HashSet;

pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    to_lines(input)
        .into_iter()
        .map(|line| line.as_bytes().iter().map(|c| *c - 48).collect())
        .collect()
}

fn visible(row: &Vec<u8>) -> HashSet<usize> {
    let mut visible = HashSet::new();

    // Forward
    visible.insert(0);
    let mut tallest = &row[0];
    for (i, tree) in row.iter().enumerate().skip(1) {
        if tree > tallest {
            tallest = tree;
            visible.insert(i);
        }
    }

    //Backward
    visible.insert(row.len() - 1);
    let mut tallest = &row[row.len() - 1];
    for (i, tree) in row.iter().enumerate().rev().skip(1) {
        if tree > tallest {
            tallest = tree;
            visible.insert(i);
        }
    }

    visible
}

pub fn part1(input: String) -> usize {
    let tree_grid = parse_input(&input);
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    for (y, row) in tree_grid.iter().enumerate() {
        for x in visible(row) {
            set.insert((y, x));
        }
    }
    let tree_grid = transpose(&tree_grid);
    for (x, col) in tree_grid.iter().enumerate() {
        for y in visible(col) {
            set.insert((y, x));
        }
    }
    set.len()
}

#[derive(Copy, Clone)]
struct Tree {
    height: u8,
    scores: [usize; 4],
}

impl Tree {
    fn new(height: u8) -> Self {
        Self {
            height,
            scores: [0; 4],
        }
    }
}

// up = 0
// left = 1
// right = 2
// down = 3
pub fn part2(input: String) -> usize {
    let mut tree_grid: Vec<Vec<Tree>> = parse_input(&input)
        .into_iter()
        .map(|row| row.into_iter().map(Tree::new).collect())
        .collect();
    for row in tree_grid.iter_mut() {
        for x in 0..row.len() {
            let immutable_row = row.clone();
            let (left, right) = immutable_row.split_at(x);
            let mut count = 0;
            for x1 in left.iter().rev() {
                if x1.height < row[x].height {
                    count += 1;
                }
                if x1.height >= row[x].height {
                    count += 1;
                    break;
                }
            }
            row[x].scores[1] = count;
            let mut count = 0;
            for x2 in right.iter().skip(1) {
                if x2.height < row[x].height {
                    count += 1;
                }
                if x2.height >= row[x].height {
                    count += 1;
                    break;
                }
            }
            row[x].scores[3] = count;
        }
    }
    let mut tree_grid = transpose(&tree_grid);
    for row in tree_grid.iter_mut() {
        for x in 0..row.len() {
            let immutable_row = row.clone();
            let (left, right) = immutable_row.split_at(x);
            let mut count = 0;
            for x1 in left.iter().rev() {
                if x1.height < row[x].height {
                    count += 1;
                }
                if x1.height >= row[x].height {
                    count += 1;
                    break;
                }
            }
            row[x].scores[0] = count;
            let mut count = 0;
            for x2 in right.iter().skip(1) {
                if x2.height < row[x].height {
                    count += 1;
                }
                if x2.height >= row[x].height {
                    count += 1;
                    break;
                }
            }
            row[x].scores[2] = count;
        }
    }
    let tree_grid = transpose(&tree_grid);
    tree_grid
        .into_iter()
        .flat_map(|row| {
            row.into_iter()
                .map(|t| t.scores.into_iter().product::<usize>())
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::*;

    #[test]
    fn test_visible() {
        let a = visible(&vec![2, 5, 5, 1, 2]);
        assert!([0, 1, 2].iter().all(|i| a.contains(i)));

        assert!([0, 1, 2].iter().all(|i| a.contains(i)));
        let b = visible(&vec![7, 1, 3, 4, 9]);
        assert!([0, 4].iter().all(|i| b.contains(i)));
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
