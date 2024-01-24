use aoc_lib::util::to_lines;
use std::ops::RangeInclusive;

fn parse_input(input: String) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    to_lines(&input)
        .iter()
        .map(|pair| pair.split(','))
        .map(|ranges| {
            ranges
                .map(|range| range.split('-').map(|v| v.parse().unwrap()))
                .map(|mut range| range.next().unwrap()..=range.next().unwrap())
        })
        .map(|mut ranges| (ranges.next().unwrap(), ranges.next().unwrap()))
        .collect()
}

pub fn part1(input: String) -> usize {
    parse_input(input)
        .iter()
        .filter(|pair| {
            pair.0.clone().all(|v| pair.1.contains(&v))
                || pair.1.clone().all(|v| pair.0.contains(&v))
        })
        .count()
}

pub fn part2(input: String) -> usize {
    parse_input(input)
        .iter()
        .filter(|pair| {
            pair.0.clone().any(|v| pair.1.contains(&v))
                || pair.1.clone().any(|v| pair.0.contains(&v))
        })
        .count()
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
