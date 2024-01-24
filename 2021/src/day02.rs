use aoc_lib::util::to_lines;

enum Direction {
    Down,
    Up,
    Forward,
}

impl From<&String> for Direction {
    fn from(s: &String) -> Self {
        match s.chars().next().unwrap() {
            'd' => Direction::Down,
            'u' => Direction::Up,
            'f' => Direction::Forward,
            x => panic!("{}", x),
        }
    }
}

fn get_x(s: &String) -> i32 {
    String::from(s)
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as i32
}

fn get_dir_x(input: Vec<String>) -> Vec<(Direction, i32)> {
    input
        .iter()
        .map(|s| (Direction::from(s), get_x(s)))
        .collect()
}

pub fn part1(input: String) -> i32 {
    let mut depth = 0;
    let mut horizontal = 0;

    for (dir, x) in get_dir_x(to_lines(&input)) {
        match dir {
            Direction::Down => depth += x,
            Direction::Up => depth -= x,
            Direction::Forward => horizontal += x,
        }
    }
    horizontal * depth
}

pub fn part2(input: String) -> i32 {
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal = 0;

    for (dir, x) in get_dir_x(to_lines(&input)) {
        match dir {
            Direction::Down => aim += x,
            Direction::Up => aim -= x,
            Direction::Forward => {
                horizontal += x;
                depth += x * aim;
            }
        }
    }
    horizontal * depth
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
