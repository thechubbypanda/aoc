pub fn part1(input: String) -> usize {
    let input: Vec<(Vec<String>, Vec<String>)> = input
        .lines()
        .map(|l| l.split(" | "))
        .map(|mut s| {
            (
                s.next()
                    .unwrap()
                    .split_whitespace()
                    .map(String::from)
                    .collect(),
                s.next()
                    .unwrap()
                    .split_whitespace()
                    .map(String::from)
                    .collect(),
            )
        })
        .collect();
    // println!("{:?}", input);
    let input: Vec<Vec<String>> = input.into_iter().map(|l| l.1).collect();
    let input: Vec<Vec<usize>> = input
        .iter()
        .map(|outputs| {
            outputs
                .iter()
                .map(digit_from_string_len)
                .filter(|o| o.is_some())
                .map(|o| o.unwrap())
                .collect()
        })
        .collect();
    // println!("{:?}", input);

    input.iter().flatten().count()
}

fn digit_from_string_len(s: &String) -> Option<usize> {
    return match s.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    };
}

pub fn part2(input: String) -> usize {
    let input: Vec<(Vec<String>, Vec<String>)> = input
        .lines()
        .map(|l| l.split(" | "))
        .map(|mut s| {
            (
                s.next()
                    .unwrap()
                    .split_whitespace()
                    .map(String::from)
                    .collect(),
                s.next()
                    .unwrap()
                    .split_whitespace()
                    .map(String::from)
                    .collect(),
            )
        })
        .collect();
    let mut total = 0;
    for (uniques, output) in input {
        let uniques: Vec<(String, Option<usize>)> = uniques
            .into_iter()
            .map(|u| (u.clone(), digit_from_string_len(&u)))
            .collect();
        let mut found: Vec<(String, usize)> = uniques
            .iter()
            .filter(|(_, v)| v.is_some())
            .map(|(s, v)| (s.clone(), v.unwrap()))
            .collect();
        let left: Vec<String> = uniques
            .into_iter()
            .filter(|(_, v)| v.is_none())
            .map(|(s, _)| s)
            .collect();
        for s in left.clone() {
            if get_string_from_val(&found, 1)
                .unwrap()
                .chars()
                .all(|c| s.contains(c))
            {
                match s.len() {
                    5 => safe_push(&mut found, (s, 3)),
                    6 => {
                        if get_string_from_val(&found, 4)
                            .unwrap()
                            .chars()
                            .all(|c| s.contains(c))
                        {
                            safe_push(&mut found, (s, 9));
                        } else {
                            safe_push(&mut found, (s, 0));
                        }
                    }
                    _ => unreachable!(),
                }
            } else if s.len() == 6 {
                safe_push(&mut found, (s, 6));
            }
        }
        let c = get_string_from_val(&found, 8)
            .unwrap()
            .chars()
            .filter(|c| !get_string_from_val(&found, 6).unwrap().contains(*c))
            .next()
            .unwrap();
        for s in left.into_iter().filter(|s| s.len() == 5) {
            if get_string_from_val(&found, 1)
                .unwrap()
                .chars()
                .all(|c| s.contains(c))
            {
                continue;
            }
            if s.contains(c) {
                safe_push(&mut found, (s, 2));
            } else {
                safe_push(&mut found, (s, 5));
            }
        }
        // println!("{:?}", found);
        let mut q = String::new();
        for o in output {
            q.push_str(
                &found
                    .iter()
                    .filter(|(s, _)| o.len() == s.len() && o.chars().all(|u| s.contains(u)))
                    .next()
                    .unwrap()
                    .1
                    .to_string(),
            );
        }
        total += q.parse::<usize>().unwrap();
        // break;
    }
    total
}

fn get_string_from_val(found: &Vec<(String, usize)>, value: usize) -> Option<String> {
    found
        .iter()
        .filter(|(_, v)| *v == value)
        .map(|(s, _)| s.clone())
        .next()
}

fn safe_push(found: &mut Vec<(String, usize)>, i: (String, usize)) {
    if get_string_from_val(found, i.1).is_none() {
        found.push(i);
    }
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
