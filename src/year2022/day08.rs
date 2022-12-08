use aoc_lib::util::{to_lines, transpose};
use std::collections::HashSet;

pub fn parse_input(input: String) -> Vec<Vec<u8>> {
    to_lines(&input)
        .into_iter()
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

fn visible<'a>(
    row: impl Iterator<Item = (usize, &'a u8)> + 'a,
) -> impl Iterator<Item = usize> + 'a {
    let mut tallest: u8 = 0;
    row.filter_map(move |(index, tree_height)| {
        if *tree_height > tallest {
            tallest = *tree_height;
            Some(index)
        } else {
            None
        }
    })
}

pub fn part1(input: String) -> usize {
    let mut set = HashSet::new();

    let grid_rows = parse_input(input);
    for (y, row) in grid_rows.iter().enumerate() {
        set.extend(
            visible(row.iter().enumerate())
                .chain(visible(row.iter().enumerate().rev()))
                .map(|x| (y, x)),
        );
    }

    let grid_cols = transpose(&grid_rows);
    for (x, col) in grid_cols.iter().enumerate() {
        set.extend(
            visible(col.iter().enumerate())
                .chain(visible(col.iter().enumerate().rev()))
                .map(|y| (y, x)),
        );
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
    let mut tree_grid: Vec<Vec<Tree>> = parse_input(input)
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
        let a: Vec<usize> = visible([2, 5, 5, 1, 2].iter().enumerate()).collect();
        assert!([0, 1].iter().all(|i| a.contains(i)));
        let b: Vec<usize> = visible([7, 1, 3, 4, 9].iter().enumerate()).collect();
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
