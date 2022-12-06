use std::collections::HashSet;

fn all_different(w: &[char]) -> bool {
    let mut set = HashSet::new();
    for c in w {
        if !set.insert(c) {
            return false;
        }
    }
    true
}

fn common(input: String, marker_size: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(marker_size)
        .map(all_different)
        .enumerate()
        .find(|(_, different)| *different)
        .unwrap()
        .0
        + marker_size
}

pub fn part1(input: String) -> usize {
    common(input, 4)
}

pub fn part2(input: String) -> usize {
    common(input, 14)
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
