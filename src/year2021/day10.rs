use aoc_lib::util::to_lines;

const AO: char = '(';
const AC: char = ')';
const BO: char = '[';
const BC: char = ']';
const CO: char = '{';
const CC: char = '}';
const DO: char = '<';
const DC: char = '>';

const OPENS: [char; 4] = [AO, BO, CO, DO];
const CLOSES: [char; 4] = [AC, BC, CC, DC];

fn get_closing(c: char) -> char {
    match c {
        AO => AC,
        BO => BC,
        CO => CC,
        DO => DC,
        _ => unreachable!(),
    }
}

pub fn part1(input: String) -> usize {
    let input = to_lines(&input);
    let mut stack: Vec<char> = Vec::new();
    let mut points = 0;
    for l in input.iter() {
        for c in l.chars() {
            if OPENS.contains(&c) {
                stack.push(c);
                continue;
            }
            let q = stack.pop();
            if q.is_none() {
                break;
            }
            let q = q.unwrap();
            let p = match c {
                t if t == get_closing(q) => 0,
                AC => 3,
                BC => 57,
                CC => 1197,
                DC => 25137,
                _ => unreachable!(),
            };
            if p > 0 {
                points += p;
                // println!("{:?}, {}", c, q);
                break;
            }
        }
        // println!("{:?}, {}", l, points);

        stack.clear();
    }
    points
}

pub fn part2(input: String) -> usize {
    let input = to_lines(&input);
    let mut stack: Vec<char> = Vec::new();
    let mut scores = Vec::new();
    for l in input.iter() {
        let mut bad = false;
        for c in l.chars() {
            if OPENS.contains(&c) {
                stack.push(c);
                continue;
            }
            let q = stack.pop();
            if q.is_none() {
                break;
            }
            let q = q.unwrap();
            if c != get_closing(q) {
                bad = true;
                break;
            }
        }
        if !bad {
            let completion = stack
                .clone()
                .into_iter()
                .rev()
                .map(get_closing)
                .collect::<Vec<char>>();
            let mut p = 0;
            for c in completion {
                p *= 5;
                p += match c {
                    AC => 1,
                    BC => 2,
                    CC => 3,
                    DC => 4,
                    _ => unreachable!(),
                }
            }
            scores.push(p);
            // println!("{:?}", p);
        }

        stack.clear();
    }

    scores.sort();
    scores[scores.len() / 2]
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
