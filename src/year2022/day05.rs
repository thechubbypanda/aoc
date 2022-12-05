use aoc_lib::util::{to_lines, transpose};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Action {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_input(input: String) -> (Vec<Vec<char>>, Vec<Action>) {
    let mut split = input.split("\n\n");

    let mut lines: Vec<Vec<char>> = to_lines(split.next().unwrap())
        .into_iter()
        .map(|line| {
            let row: Vec<char> = line
                .chars()
                .chunks(4)
                .into_iter()
                .map(|mut stack| stack.nth(1).unwrap())
                .collect();
            row
        })
        .collect();
    lines.pop();
    let stack_count = lines.iter().map(|line| line.len()).max().unwrap();
    let lines: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| {
            let len = line.len();
            (line, stack_count - len)
        })
        .map(|(mut line, to_add)| {
            (0..to_add).for_each(|_| line.push(' '));
            line
        })
        .collect();
    let stacks = transpose(&lines);
    let stacks = stacks
        .into_iter()
        .map(|stack| stack.into_iter().rev().filter(|c| *c != ' ').collect_vec())
        .collect_vec();

    let action_regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let actions: Vec<Action> = to_lines(split.next().unwrap())
        .into_iter()
        .map(|line| {
            let captures = action_regex.captures_iter(&line).next().unwrap();
            let mut values = captures
                .iter()
                .skip(1)
                .map(|capture| capture.unwrap().as_str().parse().unwrap());
            Action {
                count: values.next().unwrap(),
                from: values.next().unwrap() - 1usize,
                to: values.next().unwrap() - 1usize,
            }
        })
        .collect();

    (stacks, actions)
}

pub fn part1(input: String) -> String {
    let (mut stacks, actions) = parse_input(input);
    for action in actions {
        for _ in 0..action.count {
            let c = stacks[action.from].pop().unwrap();
            stacks[action.to].push(c);
        }
    }
    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

pub fn part2(input: String) -> String {
    let (mut stacks, actions) = parse_input(input);
    for action in actions {
        let mut popped = vec![];
        for _ in 0..action.count {
            popped.push(stacks[action.from].pop().unwrap());
        }
        for c in popped.into_iter().rev() {
            stacks[action.to].push(c);
        }
    }
    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
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
