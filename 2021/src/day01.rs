use aoc_lib::util::{parse_strings, to_lines};

pub fn part1(input: String) -> usize {
    common(to_lines(&input), 1)
}

pub fn part2(input: String) -> usize {
    common(to_lines(&input), 3)
}

fn common(input: Vec<String>, skip: usize) -> usize {
    let input: Vec<usize> = parse_strings(&input).unwrap();
    input
        .iter()
        .zip(input.iter().skip(skip))
        .filter(|(a, b)| b > a)
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
