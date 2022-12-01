use std::collections::{HashMap, HashSet};

use aoc_lib::util::to_lines;
use scanf::sscanf;

pub fn part1(input: String) -> usize {
    let mut ps: Vec<(usize, usize)> = to_lines(&input)
        .iter()
        .map(|l| {
            let mut a: usize = 0;
            let mut b = 0;
            sscanf!(l, "Player {} starting position: {}", a, b).unwrap();
            b
        })
        .map(|p| (p, 0))
        .collect();
    println!("{:?}", ps);
    let mut die_state: usize = 0;
    let mut roll_c: usize = 0;
    while !ps.iter().any(|(_, score)| *score >= 1000) {
        for (pos, score) in ps.iter_mut() {
            *pos = (*pos + rolls(&mut die_state, &mut roll_c, 3)) % 10;
            *score += *pos;
            if *pos == 0 {
                *score += 10;
            }
            if *score >= 1000 {
                break;
            }
        }
        println!("{:?}", ps);
    }

    roll_c * ps.iter().map(|(_, score)| score).min().unwrap()
}

fn rolls(die_state: &mut usize, roll_c: &mut usize, count: usize) -> usize {
    (0..count).map(|_| roll(die_state, roll_c)).sum()
}

fn roll(die_state: &mut usize, roll_c: &mut usize) -> usize {
    *die_state = (*die_state + 1) % 100;
    *roll_c += 1;
    *die_state
}

type States = [u8; 2];
type Scores = [u8; 2];
type Wins = [u64; 2];

pub fn part2(input: String) -> u64 {
    let ps: Vec<u8> = to_lines(&input)
        .iter()
        .map(|l| {
            let mut a: usize = 0;
            let mut b = 0;
            sscanf!(l, "Player {} starting position: {}", a, b).unwrap();
            b
        })
        .collect();
    let states: States = [ps[0], ps[1]];
    println!("{:?}", states);

    // Caclulates 3 ^ 6 combinations of rolls where a roll is in (1..=3)
    //  returns the sum of the first 3 and the sum of the second 3
    // Yes it's ugly, I know
    let q_rolls: Vec<[u8; 2]> = (1..=3)
        .flat_map(move |a: u8| {
            (1..=3).flat_map(move |b: u8| {
                (1..=3).flat_map(move |c: u8| {
                    (1..=3).flat_map(move |d: u8| {
                        (1..=3).flat_map(move |e: u8| (1..=3).map(move |f| [a + b + c, d + e + f]))
                    })
                })
            })
        })
        .collect();

    // Collect the 729 combinations into 49 unique rolls and their appearance counts
    let flat_rolls: HashMap<[u8; 2], u64> = q_rolls
        .clone()
        .into_iter()
        .collect::<HashSet<[u8; 2]>>()
        .iter()
        .map(|u| (*u, q_rolls.iter().filter(|r| **r == *u).count() as u64))
        .collect();

    println!("{:?}", flat_rolls);
    let wins = dirac(states, [0; 2], &mut HashMap::new(), &flat_rolls);
    println!("{:?}", wins);
    wins.into_iter().max().unwrap()
}

fn dirac(
    states: States,
    scores: Scores,
    reference: &mut HashMap<(States, Scores), Wins>, // The "dynamic programming" storage
    quantum_rolls: &HashMap<[u8; 2], u64>, // All unique roll sums and their counts from 729 (3^6) combinations of rolls
) -> Wins {
    if let Some(r) = reference.get(&(states, scores)) {
        return *r;
    }
    let mut wins = [0, 0];
    for (rolls, count) in quantum_rolls.iter() {
        let mut temp_states = states;
        let mut temp_scores = scores;
        let mut win = false;
        for (i, roll) in rolls.iter().enumerate() {
            temp_states[i] = (temp_states[i] + *roll) % 10;
            temp_scores[i] += temp_states[i];
            if temp_states[i] == 0 {
                temp_scores[i] += 10;
            }
            if temp_scores[i] >= 21 {
                wins[i] += *count;
                win = true;
                break;
            }
        }
        if !win {
            wins = add_wins(
                &wins,
                &scale_wins(
                    &dirac(temp_states, temp_scores, reference, quantum_rolls),
                    count,
                ),
            );
        }
    }
    reference.insert((states, scores), wins);
    wins
}

fn add_wins(s1: &Wins, s2: &Wins) -> Wins {
    [s1[0] + s2[0], s1[1] + s2[1]]
}

fn scale_wins(w: &Wins, c: &u64) -> Wins {
    [w[0] * c, w[1] * c]
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
}
