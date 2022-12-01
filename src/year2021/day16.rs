fn take_next(packet: &String, index: &mut usize, len: usize) -> String {
    *index += len;
    let s = packet.chars().skip(*index - len).take(len).collect();
    // println!("{}", s);
    s
}

fn from_bin(s: String) -> usize {
    usize::from_str_radix(&s, 2).unwrap()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => unreachable!(),
    }
}

pub fn part1(input: String) -> usize {
    let input: String = input
        .split_whitespace()
        .next()
        .unwrap()
        .chars()
        .map(|c| to_binary(c))
        .collect();
    // let input = "00111000000000000110111101000101001010010001001000000000".to_string();
    let mut versions = Vec::new();
    parse_packet_1(&input, &mut 0, &mut versions);
    versions.iter().sum()
}

fn parse_packet_1(packet: &String, index: &mut usize, versions: &mut Vec<usize>) {
    let v = from_bin(take_next(&packet, index, 3));
    let t = from_bin(take_next(&packet, index, 3));
    println!("{}", t);

    versions.push(v);

    // println!("{}",  t);
    if t != 4 {
        let i = from_bin(take_next(&packet, index, 1));
        if i == 0 {
            let l = from_bin(take_next(&packet, index, 15));
            let q = *index + l;
            while *index < q {
                parse_packet_1(&packet, index, versions);
            }
        } else if i == 1 {
            let l = from_bin(take_next(&packet, index, 11));
            for _ in 0..l {
                parse_packet_1(&packet, index, versions);
            }
        }
    } else {
        let mut v = String::new();
        loop {
            let g = take_next(&packet, index, 5);
            g.chars().skip(1).for_each(|c| v.push(c));
            if g.chars().next().unwrap() == '0' {
                break;
            }
        }
    }
}

pub fn part2(input: String) -> usize {
    let input: String = input
        .split_whitespace()
        .next()
        .unwrap()
        .chars()
        .map(|c| to_binary(c))
        .collect();
    // let input = "00111000000000000110111101000101001010010001001000000000".to_string();
    parse_packet_2(&input, &mut 0)
}

fn parse_packet_2(packet: &String, index: &mut usize) -> usize {
    let v = from_bin(take_next(&packet, index, 3));
    let t = from_bin(take_next(&packet, index, 3));

    if t == 4 {
        let mut v = String::new();
        loop {
            let g = take_next(&packet, index, 5);
            g.chars().skip(1).for_each(|c| v.push(c));
            if g.chars().next().unwrap() == '0' {
                break;
            }
        }
        return from_bin(v);
    }

    let mut vs = Vec::new();
    let i = from_bin(take_next(&packet, index, 1));
    if i == 0 {
        let l = from_bin(take_next(&packet, index, 15));
        let q = *index + l;
        while *index < q {
            vs.push(parse_packet_2(&packet, index));
        }
    } else if i == 1 {
        let l = from_bin(take_next(&packet, index, 11));
        for _ in 0..l {
            vs.push(parse_packet_2(&packet, index));
        }
    }
    return match t {
        0 => vs.iter().sum(),
        1 => vs.iter().product(),
        2 => *vs.iter().min().unwrap(),
        3 => *vs.iter().max().unwrap(),
        5 => {
            if vs[0] > vs[1] {
                return 1;
            }
            0
        }
        6 => {
            if vs[0] < vs[1] {
                return 1;
            }
            0
        }
        7 => {
            if vs[0] == vs[1] {
                return 1;
            }
            0
        }
        _ => unreachable!(),
    };
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
