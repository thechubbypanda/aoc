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
            (
                line.chars().take(line.len() / 2).collect::<Vec<char>>(),
                line.chars().skip(line.len() / 2).collect::<Vec<char>>(),
            )
        })
        .map(|(l, r)| {
            let set: HashSet<char> = HashSet::from_iter(l.into_iter());
            let common = r.iter().find(|i| set.contains(i)).unwrap();
            to_priority(*common)
        })
        .sum()
}

fn in_all(item: char, rucksacks: &[String]) -> bool {
    rucksacks
        .iter()
        .map(|rucksack| rucksack.chars())
        .all(|mut items| items.any(|v| v == item))
}

fn find_common_item(rucksacks: &[String]) -> Option<char> {
    rucksacks
        .iter()
        .flat_map(|rucksack| rucksack.chars())
        .collect::<HashSet<char>>()
        .into_iter()
        .find(|item| in_all(*item, rucksacks))
}

pub fn part2(input: String) -> usize {
    to_lines(&input)
        .windows(3)
        .step_by(3)
        .map(|rucksacks| {
            to_priority(find_common_item(rucksacks).expect("Failed to find common character"))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::*;

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
