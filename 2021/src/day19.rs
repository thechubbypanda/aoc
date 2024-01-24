use std::{collections::HashSet, ops::Mul};

use aoc_lib::util::to_lines;
use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vec3(i32, i32, i32);

impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({}, {}, {})", self.0, self.1, self.2))
    }
}

impl Vec3 {
    // fn cross(&self, v: Vec3) -> Self {
    //     Self(
    //         self.1 * v.2 - self.2 * v.1,
    //         self.2 * v.0 - self.0 * v.2,
    //         self.0 * v.1 - self.1 * v.0,
    //     )
    // }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

// const XY: [Vec3; 4] = [
//     Vec3(1, 1, 1),
//     Vec3(1, -1, 1),
//     Vec3(-1, 1, 1),
//     Vec3(-1, -1, 1),
// ];
// const YZ: [Vec3; 4] = [
//     Vec3(1, 1, 1),
//     Vec3(1, 1, -1),
//     Vec3(1, -1, 1),
//     Vec3(1, -1, -1),
// ];
// const XZ: [Vec3; 4] = [
//     Vec3(1, 1, 1),
//     Vec3(1, 1, -1),
//     Vec3(-1, 1, 1),
//     Vec3(-1, 1, -1),
// ];

// fn rotations(beacons: Vec<Vec3>) -> Vec<Vec<Vec3>> {
//     let out = XY
//         .iter()
//         .flat_map(|xy| {
//             YZ.iter()
//                 .flat_map(|yz| {
//                     XZ.iter()
//                         .map(|xz| {
//                             beacons
//                                 .iter()
//                                 .map(|b| *b * *xy * *yz * *xz)
//                                 .collect::<Vec<Vec3>>()
//                         })
//                         .collect::<Vec<Vec<Vec3>>>()
//                 })
//                 .collect::<Vec<Vec<Vec3>>>()
//         })
//         .collect();
//     out
// }

pub fn part1(input: String) -> usize {
    let scans: Vec<Vec<Vec3>> = Regex::new(r"--- scanner \d+ ---")
        .unwrap()
        .split(&input)
        .skip(1)
        .map(|sv| sv.trim())
        .map(|sv| {
            to_lines(sv)
                .into_iter()
                .map(|b| {
                    b.split(',')
                        .map(|b_c| b_c.parse().unwrap())
                        .collect::<Vec<i32>>()
                })
                .map(|b| Vec3(b[0], b[1], b[2]))
                .collect()
        })
        .collect();
    // for sv in scanner_views.iter() {
    //     println!("{:?}", sv)
    // }

    let beacons: HashSet<Vec3> = HashSet::new();

    // for s0 in scans.iter() {
    //     for s1 in scans.iter() {
    //         if s0 == s1 {
    //             continue;
    //         }
    //         // rotations(*s1).iter().filter(||)
    //         break; // t
    //     }
    //     break; // t
    // }

    println!("{:?}", beacons);

    println!("Here is where I gave up");
    scans.len()
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

    // #[test]
    // fn test_part2() {
    //     run_test!(part2);
    // }
    //
    // #[test]
    // fn run_part2() {
    //     run_real!(part2);
    // }
}
