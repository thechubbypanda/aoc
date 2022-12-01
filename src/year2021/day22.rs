// use std::{
//     cmp::{max, min},
//     collections::HashSet,
//     ops::{Range, RangeInclusive},
// };

// use aoc_lib::util::to_lines;
// use scanf::sscanf;

// type Map = HashSet<(i32, i32, i32)>;

// pub fn part1(input: String) -> usize {
//     let instructions: Vec<(
//         bool,
//         RangeInclusive<i32>,
//         RangeInclusive<i32>,
//         RangeInclusive<i32>,
//     )> = to_lines(&input)
//         .iter()
//         .map(|l| {
//             let mut ls = l.split(' ');
//             let on = if ls.next().unwrap() == "on" {
//                 true
//             } else {
//                 false
//             };
//             let mut xmin = 0;
//             let mut xmax = 0;
//             let mut ymin = 0;
//             let mut ymax = 0;
//             let mut zmin = 0;
//             let mut zmax = 0;
//             sscanf!(
//                 &ls.next().unwrap(),
//                 "x={}..{},y={}..{},z={}..{}",
//                 xmin,
//                 xmax,
//                 ymin,
//                 ymax,
//                 zmin,
//                 zmax
//             )
//             .unwrap();
//             let xr = min(xmin, xmax)..=max(xmin, xmax);
//             let yr = min(ymin, ymax)..=max(ymin, ymax);
//             let zr = min(zmin, zmax)..=max(zmin, zmax);
//             (on, xr, yr, zr)
//         })
//         .collect();
//     println!("{:?}", &instructions);
//     let mut map = Map::new();
//     for (on, xr, yr, zr) in instructions {
//         if in_range(xr.clone().next().unwrap())
//             && in_range(xr.clone().last().unwrap())
//             && in_range(yr.clone().next().unwrap())
//             && in_range(yr.clone().last().unwrap())
//             && in_range(zr.clone().next().unwrap())
//             && in_range(zr.clone().last().unwrap())
//         {
//             for x in xr {
//                 for y in yr.clone() {
//                     for z in zr.clone() {
//                         if on {
//                             map.insert((x, y, z));
//                         } else {
//                             map.remove(&(x, y, z));
//                         }
//                     }
//                 }
//             }
//         }
//         println!("{}", map.len());
//     }
//     map.len()
// }

// fn in_range(v: i32) -> bool {
//     v <= 50 && v >= -50
// }

// #[derive(Clone, Debug, Eq, PartialEq, Hash)]
// struct Point {
//     x: i32,
//     y: i32,
//     z: i32,
// }

// impl From<(i32, i32, i32)> for Point {
//     fn from((x, y, z): (i32, i32, i32)) -> Self {
//         Point { x, y, z }
//     }
// }

// #[derive(Clone, Debug, Eq, PartialEq, Hash)]
// struct Cube {
//     p1: Point,
//     p2: Point,
// }

// impl From<(Point, Point)> for Cube {
//     fn from((p1, p2): (Point, Point)) -> Self {
//         Cube { p1, p2 }
//     }
// }

// impl Cube {
//     fn intersect(&self, o: &Cube) -> Option<Cube> {
//         None
//     }

//     fn contains(&self, p: Point) -> bool {
//         // Assumed all p1 are smaller than p2
//         self.p1.x <= p.x
//             && p.x <= self.p2.x
//             && self.p1.y <= p.y
//             && p.y <= self.p2.y
//             && self.p1.z <= p.z
//             && p.z <= self.p2.z
//     }

//     fn vol(&self) -> i32 {
//         let w = (self.p1.x - self.p2.x).abs();
//         let h = (self.p1.y - self.p2.y).abs();
//         let d = (self.p1.z - self.p2.z).abs();
//         w * h * d
//     }
// }

// pub fn part2(input: String) -> usize {
//     let instructions: Vec<(bool, Cube)> = to_lines(&input)
//         .iter()
//         .map(|l| {
//             let mut ls = l.split(' ');
//             let on = if ls.next().unwrap() == "on" {
//                 true
//             } else {
//                 false
//             };
//             let mut xmin = 0;
//             let mut xmax = 0;
//             let mut ymin = 0;
//             let mut ymax = 0;
//             let mut zmin = 0;
//             let mut zmax = 0;
//             sscanf!(
//                 &ls.next().unwrap(),
//                 "x={}..{},y={}..{},z={}..{}",
//                 xmin,
//                 xmax,
//                 ymin,
//                 ymax,
//                 zmin,
//                 zmax
//             )
//             .unwrap();
//             (
//                 on,
//                 Cube::from((
//                     Point::from((xmin, ymin, zmin)),
//                     Point::from((xmax, ymax, zmax)),
//                 )),
//             )
//         })
//         .collect();
//     println!("{:?}", &instructions);
//     let mut cubes: HashSet<Cube> = HashSet::new();
//     for (on, i_cube) in instructions.iter() {
//         for e_cube in cubes.clone() {
//             if let Some(intersect) = e_cube.intersect(i_cube) {
//                 cubes.remove(&e_cube);

//                 cubes.insert()
//             }
//         }
//     }
//     0
// }
