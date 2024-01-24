use std::{
    fmt::{Debug, Display},
    time::Instant,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Node {
    Floor,
    Empty,
    Occupied,
}

impl Node {
    fn to_char(self) -> char {
        match self {
            Node::Floor => '.',
            Node::Empty => 'L',
            Node::Occupied => '#',
        }
    }
}

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    seats: Vec<Node>,
}

impl Grid {
    fn from(input: String) -> Self {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let seats = input
            .lines()
            .map(|l| {
                l.chars().map(|c| match c {
                    '.' => Node::Floor,
                    'L' => Node::Empty,
                    '#' => Node::Occupied,
                    _ => unreachable!(),
                })
            })
            .flatten()
            .collect();
        Self {
            width,
            height,
            seats,
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        x + y * &self.width
    }

    fn get_adjacent_indices(&self, _x: usize, _y: usize) -> Vec<usize> {
        let mut adjacents = vec![];
        for y in (-(1 as i32))..=1 {
            let y = _y as i32 + y;
            if y < 0 || y >= self.height as i32 {
                continue;
            }
            for x in (-(1 as i32))..=1 {
                let x = _x as i32 + x;
                if x < 0 || x >= self.width as i32 {
                    continue;
                }
                if x == _x as i32 && y == _y as i32 {
                    continue;
                }
                adjacents.push(self.get_index(x as usize, y as usize));
            }
        }
        adjacents
    }

    fn simulate(&self) -> Self {
        let mut grid = self.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index(x, y);
                let adjs = self.get_adjacent_indices(x, y);
                match self.seats[index] {
                    Node::Floor => continue,
                    Node::Empty => {
                        if adjs.iter().all(|i| self.seats[*i] != Node::Occupied) {
                            grid.seats[index] = Node::Occupied;
                        }
                    }
                    Node::Occupied => {
                        if adjs
                            .iter()
                            .filter(|i| self.seats[**i] == Node::Occupied)
                            .count()
                            >= 4
                        {
                            grid.seats[index] = Node::Empty;
                        }
                    }
                }
            }
        }
        return grid;
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.seats[self.get_index(x, y)].to_char()).unwrap();
            }
            writeln!(f, "").unwrap();
        }
        Ok(())
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid = Grid::from(input);

    let timer = Instant::now();
    println!("Part 1 output: {}", part1(grid.clone()));
    println!("Part 1 time: {:?}", timer.elapsed());
}

fn part1(grid: Grid) -> usize {
    let mut grid = grid;
    loop {
        let temp = grid.simulate();
        if temp.seats == grid.seats {
            return grid.seats.iter().filter(|n| **n == Node::Occupied).count();
        }
        grid = temp;
    }
}
