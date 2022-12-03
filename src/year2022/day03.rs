use std::collections::{HashMap, HashSet};

use aoc_lib::util::to_lines;

fn to_priority(item: char) -> usize {
    match item {
        _ if ('a'..='z').contains(&item) => item as usize - 'a' as usize + 1,
        _ if ('A'..='Z').contains(&item) => item as usize - 'A' as usize + 27,
        _ => unreachable!(),
    }
}

fn parse_input(input: String) -> Vec<String> {
    to_lines(&input)
}

pub fn part1(input: String) -> usize {
    parse_input(input)
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

pub fn part2(input: String) -> usize {
    parse_input(input)
        .windows(3)
        .step_by(3)
        .map(|group| {
            let mut map: HashMap<char, usize> =
                HashMap::from_iter(group[0].chars().zip((0..).map(|_| 1)));
            let r2: HashSet<char> = HashSet::from_iter(group[1].chars());
            for i in r2 {
                let _ = match map.get(&i) {
                    None => map.insert(i, 1),
                    Some(_) => map.insert(i, 2),
                };
            }
            let common = group[2]
                .chars()
                .find(|i| map.get(i).is_some() && *map.get(i).unwrap() == 2)
                .unwrap();
            to_priority(common)
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
