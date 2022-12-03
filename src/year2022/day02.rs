use aoc_lib::util::to_lines;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    fn beats(&self) -> Self {
        match self {
            Throw::Rock => Self::Scissors,
            Throw::Paper => Self::Rock,
            Throw::Scissors => Self::Paper,
        }
    }

    fn loses_to(&self) -> Self {
        match self {
            Throw::Rock => Self::Paper,
            Throw::Paper => Self::Scissors,
            Throw::Scissors => Self::Rock,
        }
    }

    fn play(&self, other: &Throw) -> Outcome {
        if self.beats() == *other {
            Outcome::Win
        } else if self.loses_to() == *other {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl From<char> for Throw {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

impl From<char> for Outcome {
    fn from(value: char) -> Self {
        match value {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unreachable!(),
        }
    }
}

trait Points {
    fn points(&self) -> usize;
}

impl Points for Throw {
    fn points(&self) -> usize {
        match self {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        }
    }
}

impl Points for Outcome {
    fn points(&self) -> usize {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

fn parse_input<T, U>(input: String) -> Vec<(T, U)>
where
    T: From<char>,
    U: From<char>,
{
    to_lines(&input)
        .into_iter()
        .map(|r| {
            let mut r = r.chars();
            (r.next().unwrap().into(), r.nth(1).unwrap().into())
        })
        .collect()
}

pub fn part1(input: String) -> usize {
    parse_input::<Throw, Throw>(input)
        .into_iter()
        .map(|(other, own)| own.play(&other).points() + own.points())
        .sum()
}

pub fn part2(input: String) -> usize {
    parse_input::<Throw, Outcome>(input)
        .into_iter()
        .map(|(other, desired_outcome)| {
            let throw = match desired_outcome {
                Outcome::Loss => other.beats(),
                Outcome::Draw => other,
                Outcome::Win => other.loses_to(),
            };
            desired_outcome.points() + throw.points()
        })
        .sum()
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
