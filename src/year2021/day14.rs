use std::collections::{HashMap, HashSet};

use aoc_lib::util::to_lines;

type Rules = HashMap<(char, char), char>;

pub fn part1(input: String) -> usize {
    let mut input = input.split("\n\n");
    let mut s: Vec<char> = input.next().unwrap().chars().collect();
    let rules: Rules = to_lines(&input.next().unwrap().to_string())
        .iter()
        .map(|l| l.split(" -> "))
        .map(|mut s| {
            (
                s.next().unwrap().chars(),
                s.next().unwrap().chars().next().unwrap(),
            )
        })
        .map(|(mut cs, c)| ((cs.next().unwrap(), cs.next().unwrap()), c))
        .collect();

    for _ in 0..5 {
        let mut new = Vec::with_capacity(s.len() * 2);
        for i in 0..(s.len() - 1) {
            new.push(s[i]);
            new.push(*rules.get(&(s[i], s[i + 1])).unwrap());
        }
        new.push(s[s.len() - 1]);
        s = new;
    }

    let types: HashSet<char> = s.clone().into_iter().collect();
    let counts: Vec<usize> = types
        .iter()
        .map(|t| s.iter().filter(|c| **c == *t).count())
        .collect();
    let max = counts.iter().max().unwrap();
    let min = counts.iter().min().unwrap();
    max - min
}

pub fn part2(input: String) -> usize {
    let mut input = input.split("\n\n");
    let s: Vec<char> = input.next().unwrap().chars().collect();
    let rules: Rules = to_lines(&input.next().unwrap().to_string())
        .iter()
        .map(|l| l.split(" -> "))
        .map(|mut s| {
            (
                s.next().unwrap().chars(),
                s.next().unwrap().chars().next().unwrap(),
            )
        })
        .map(|(mut cs, c)| ((cs.next().unwrap(), cs.next().unwrap()), c))
        .collect();

    let mut char_c: HashMap<char, usize> = HashMap::new();
    let mut pair_c: HashMap<(char, char), usize> = HashMap::new();

    for w in s.windows(2) {
        pair_c.insert((w[0], w[1]), *pair_c.get(&(w[0], w[1])).unwrap_or(&0) + 1);
    }
    for c in &s {
        char_c.insert(*c, *char_c.get(c).unwrap_or(&0) + 1);
    }

    for _ in 0..40 {
        for (pair, count) in pair_c.clone().iter() {
            let c = *rules.get(pair).unwrap();
            char_c.insert(c, *char_c.get(&c).unwrap_or(&0) + count);
            let p1 = (pair.0, c);
            let p2 = (c, pair.1);
            pair_c.insert(p1, *pair_c.get(&p1).unwrap_or(&0) + count);
            pair_c.insert(p2, *pair_c.get(&p2).unwrap_or(&0) + count);
            pair_c.insert(*pair, *pair_c.get(pair).unwrap() - count);
        }
    }
    let max = char_c.values().max().unwrap();
    let min = char_c.values().min().unwrap();
    max - min
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
