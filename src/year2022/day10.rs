enum Instruction {
    Noop,
    AddX(i32),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut split = value.split_whitespace();
        match split.next().unwrap() {
            "noop" => Instruction::Noop,
            "addx" => Instruction::AddX(split.next().unwrap().parse().unwrap()),
            command => panic!("Unrecognised command: {}", command),
        }
    }
}

fn update_strengths(cycle: usize, x: i32, strengths: &mut Vec<i32>) {
    if [20, 60, 100, 140, 180, 220].contains(&cycle) {
        strengths.push(cycle as i32 * x)
    }
}

pub fn part1(input: String) -> i32 {
    let mut cycle: usize = 0;
    let mut strengths = vec![];
    input
        .lines()
        .map(Instruction::from)
        .fold(1, |x, instruction| match instruction {
            Instruction::Noop => {
                cycle += 1;
                update_strengths(cycle, x, &mut strengths);
                x
            }
            Instruction::AddX(num) => {
                cycle += 1;
                update_strengths(cycle, x, &mut strengths);
                cycle += 1;
                update_strengths(cycle, x, &mut strengths);
                x + num
            }
        });
    strengths.into_iter().sum()
}

fn update_display(cycle: usize, x: i32, screen: &mut [[char; 40]; 6]) {
    let cycle_x = ((cycle - 1) % 40) as i32;
    if x.abs_diff(cycle_x) <= 1 {
        screen[(cycle - 1) / 40][(cycle - 1) % 40] = '#';
    } else {
        screen[(cycle - 1) / 40][(cycle - 1) % 40] = '.';
    }
}

pub fn part2(input: String) -> String {
    let mut cycle = 0;
    let mut x = 1;
    let mut screen = [[' '; 40]; 6];
    input
        .lines()
        .map(Instruction::from)
        .for_each(|instruction| match instruction {
            Instruction::Noop => {
                cycle += 1;
                update_display(cycle, x, &mut screen);
            }
            Instruction::AddX(num) => {
                cycle += 1;
                update_display(cycle, x, &mut screen);
                cycle += 1;
                update_display(cycle, x, &mut screen);
                x += num
            }
        });
    screen
        .iter()
        .map(|line| line.iter().collect::<String>())
        .fold(String::from("\n"), |out, line| out + &line + "\n")
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
