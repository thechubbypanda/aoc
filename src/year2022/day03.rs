use std::collections::HashSet;

use aoc_lib::util::to_lines;

fn to_priority(item: char) -> usize {
    match item {
        _ if ('a'..='z').contains(&item) => item as usize - 'a' as usize + 1,
        _ if ('A'..='Z').contains(&item) => item as usize - 'A' as usize + 27,
        _ => panic!("Unexpected character: {}", item),
    }
}

pub fn part1(input: String) -> usize {
    to_lines(&input)
        .into_iter()
        .map(|line| {
            let (l, r) = line.split_at(line.len() / 2);
            let l = l.chars().collect::<HashSet<char>>();
            let r = r.chars().collect::<HashSet<char>>();
            to_priority((&l & &r).into_iter().next().unwrap())
        })
        .sum()
}

pub fn part2(input: String) -> usize {
    to_lines(&input)
        .chunks(3)
        .map(|rucksacks| {
            let mut rucksacks = rucksacks
                .iter()
                .map(|rucksack| rucksack.chars().collect::<HashSet<char>>());
            let common = rucksacks
                .next()
                .map(|rucksack| rucksacks.fold(rucksack, |acc, r| &acc & &r))
                .unwrap()
                .into_iter()
                .next()
                .unwrap();
            to_priority(common)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use aoc_lib::*;

    use super::*;

    #[test]
    fn test_to_priority() {
        assert_eq!(to_priority('p'), 16);
        assert_eq!(to_priority('L'), 38);
        assert_eq!(to_priority('a'), 1);
        assert_eq!(to_priority('z'), 26);
        assert_eq!(to_priority('A'), 27);
        assert_eq!(to_priority('Z'), 52);
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
