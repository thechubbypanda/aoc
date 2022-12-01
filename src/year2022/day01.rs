pub fn part1(input: String) -> usize {
    parse_input(input).into_iter().map(|inv| inv.iter().sum::<usize>()).max().unwrap()
}

pub fn part2(input: String) -> usize {
    let mut elves = parse_input(input).into_iter().map(|inv| inv.iter().sum::<usize>()).collect::<Vec<usize>>();
    elves.sort();
    elves.reverse();
    elves.iter().take(3).sum()
}

fn parse_input(input: String) -> Vec<Vec<usize>> {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.parse().unwrap()).collect())
        .collect()
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

    #[test]
    fn test_part2() {
        run_test!(part2);
    }

    #[test]
    fn run_part2() {
        run_real!(part2);
    }
}
