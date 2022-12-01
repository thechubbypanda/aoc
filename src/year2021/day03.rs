use aoc_lib::util::{to_lines, transpose};

fn e_and_g(input: &Vec<Vec<char>>) -> (Vec<char>, Vec<char>) {
    let mut gamma = Vec::new();
    let mut epsilon = Vec::new();
    for col in input.iter() {
        let ones = col.iter().filter(|c| **c == '1').count();
        let zeros = col.len() - ones;
        if ones >= zeros {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    (epsilon, gamma)
}

fn usize_from_bin_chars(chars: &Vec<char>) -> usize {
    usize::from_str_radix(&chars.into_iter().collect::<String>(), 2).unwrap()
}

pub fn part1(input: String) -> usize {
    let input = to_lines(&input);
    let input: Vec<Vec<char>> = transpose(&input.iter().map(|x| x.chars().collect()).collect());
    let (epsilon, gamma) = e_and_g(&input);
    usize_from_bin_chars(&epsilon) * usize_from_bin_chars(&gamma)
}

pub fn part2(input: String) -> usize {
    let input = to_lines(&input);
    let input: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();
    let mut o2 = input.clone();
    for i in 0..o2.len() {
        let (_, gamma) = e_and_g(&transpose(&o2));
        o2.retain(|cs| cs[i] == gamma[i]);
        if o2.len() == 1 {
            break;
        }
    }

    let mut co2 = input.clone();
    for i in 0..co2.len() {
        let (epsilon, _) = e_and_g(&transpose(&co2));
        co2.retain(|cs| cs[i] == epsilon[i]);
        if co2.len() == 1 {
            break;
        }
    }
    usize_from_bin_chars(&o2[0]) * usize_from_bin_chars(&co2[0])
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
