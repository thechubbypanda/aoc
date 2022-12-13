use aoc_lib::run_test;
use std::{cmp::Ordering, fmt::format};

#[derive(Debug, Clone)]
enum Obj {
    List(ObjList),
    Num(u32),
}

type ObjList = Vec<Obj>;

fn parse(packet: &Vec<char>, pointer: &mut usize) -> ObjList {
    let mut list = ObjList::new();
    while let Some(c) = packet.get(*pointer) {
        *pointer += 1;
        match *c {
            ']' => return list,
            '[' => list.push(Obj::List(parse(packet, pointer))),
            ',' => {}
            _ => {
                let mut num = String::with_capacity(1);
                for i in (*pointer - 1).. {
                    if let Some(c) = packet.get(i) {
                        if c.is_ascii_digit() {
                            num.push(*c);
                        } else {
                            break;
                        }
                    } else {
                        panic!("Reached the end of packet without finishing num")
                    }
                }
                list.push(Obj::Num(num.parse::<u32>().unwrap()));
                *pointer += num.len() - 1;
            }
        };
    }
    list
}

fn cmp(left: &ObjList, right: &ObjList) -> Ordering {
    let mut orderings = vec![];
    for pair in left.iter().zip(right.iter()) {
        let o = match pair {
            (Obj::Num(l), Obj::Num(r)) => l.cmp(&r),
            (Obj::List(l), Obj::List(r)) => cmp(&l, &r),
            (Obj::Num(l), Obj::List(r)) => cmp(&vec![Obj::Num(*l)], &r),
            (Obj::List(l), Obj::Num(r)) => cmp(&l, &vec![Obj::Num(*r)]),
        };
        orderings.push(o);
    }
    orderings
        .into_iter()
        .find(|o| *o != Ordering::Equal)
        .unwrap_or(left.len().cmp(&right.len()))
}

pub fn part1(input: String) -> usize {
    let packet_pairs: Vec<(ObjList, ObjList)> = input
        .split("\n\n")
        .map(|lines| {
            let mut pair = lines.lines();
            (pair.next().unwrap(), pair.next().unwrap())
        })
        .map(|(left, right)| {
            (
                parse(&left.chars().skip(1).collect(), &mut 0),
                parse(&right.chars().skip(1).collect(), &mut 0),
            )
        })
        .collect();

    // for (left, right) in packet_pairs.iter() {
    //     println!("{:?}", left);
    //     println!("{:?}", right);
    //     println!("{:?}", in_order(left, right));
    //     println!();
    // }

    packet_pairs
        .iter()
        .map(|(left, right)| cmp(left, right))
        .enumerate()
        .filter(|(_, v)| *v != Ordering::Greater)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2(input: String) -> usize {
    let mut packets: Vec<ObjList> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse(&line.chars().skip(1).collect(), &mut 0))
        .collect();
    let divider1 = vec![Obj::List(vec![Obj::Num(2)])];
    let divider2 = vec![Obj::List(vec![Obj::Num(6)])];
    packets.push(divider1.clone());
    packets.push(divider2.clone());
    packets.sort_by(cmp);
    for packet in packets.iter() {
        println!("{:?}", packet);
    }
    let index1 = packets
        .iter()
        .enumerate()
        .find(|(_, p)| format!("{:?}", p) == format!("{:?}", divider1))
        .unwrap()
        .0
        + 1;
    let index2 = packets
        .iter()
        .enumerate()
        .find(|(_, p)| format!("{:?}", p) == format!("{:?}", divider2))
        .unwrap()
        .0
        + 1;
    index1 * index2
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

fn main() {
    run_test!(part2);
}
