use std::cmp::{max, min};

pub fn part1(input: String) -> usize {
    let mut input: Vec<usize> = input
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    input.sort();
    // let mut seen = Vec::new();
    // for c in input.iter() {
    //     if !seen.iter().any(|v| *v == *c) {
    //         seen.push(*c);
    //     }
    // }
    // let mut crab = 0;
    // let mut count = 0;
    // for s in seen.iter() {
    //     let new = input.iter().filter(|c| **c == *s).count();
    //     if count < new {
    //         count = new;
    //         crab = *s;
    //     }
    // }
    // let mut small_diff = 9999999;
    // let mut crab = 0;
    // for i in 0..(input.len() - 1) {
    //     let diff = input[i + 1] - in459588put[i];
    //     if diff < small_diff {
    //         println!("{}, {}",input[i], diff);
    //         small_diff = diff;
    //         crab = input[i];
    //     }
    // }
    let crab = input[input.len() / 2];
    println!("{}", crab);
    input
        .iter()
        .map(|c| max(*c, crab) - min(*c, crab))
        .sum()
}

pub fn part2(input: String) -> usize {
    let mut input: Vec<usize> = input
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    input.sort();
    let crab = input.iter().sum::<usize>() / input.len();
    let crab2 = crab - 1;
    min(input
        .iter()
        .map(|c| max(*c, crab) - min(*c, crab))
        .map(|c| (1..=c).sum::<usize>())
        .sum(),
    input
        .iter()
        .map(|c| max(*c, crab2) - min(*c, crab2))
        .map(|c| (1..=c).sum::<usize>())
        .sum())
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
}
