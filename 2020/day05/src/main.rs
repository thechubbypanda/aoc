use std::io;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq)]
struct Seat {
    row: usize,
    column: usize,
}

impl Seat {
    fn id(&self) -> usize {
        self.row * 8 + self.column
    }
}

fn main() -> io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let timer = Instant::now();
    println!("Part 1 output: {}", part1(input.lines().collect()));
    println!("Part 1 time: {:?}", timer.elapsed());
    let timer = Instant::now();
    println!("Part 2 output: {}", part2(input.lines().collect()));
    println!("Part 2 time: {:?}", timer.elapsed());
    Ok(())
}

fn part1(input: Vec<&str>) -> usize {
    input
        .iter()
        .map(|v| calc_seat(v))
        .map(|v| v.id())
        .fold(0, |a, v| if v > a { v } else { a })
}

fn char_to_bit(c: &char) -> usize {
    match c {
        'F' | 'L' => 0,
        'B' | 'R' => 1,
        _ => panic!("WTF2"),
    }
}

fn calc_seat(input: &str) -> Seat {
    let input: Vec<char> = input.chars().collect();

    let row: usize = input.iter()
        .take(7)
        .map(char_to_bit)
        .rev()
        .enumerate()
        .rev()
        .map(|(i, v)| v << i)
        .fold(0, |a, v| a + v);

    let column: usize = input.iter()
        .skip(7)
        .take(3)
        .map(char_to_bit)
        .rev()
        .enumerate()
        .rev()
        .map(|(i, v)| v << i)
        .fold(0, |a, v| a + v);

    Seat {row, column}
}

#[test]
fn test_input_one() {
    assert_eq!(calc_seat("FBFBBFFRLR"), Seat { row: 44, column: 5 });
}

fn part2(input: Vec<&str>) -> usize {
    let mut ids: Vec<usize> = input.iter()
        .map(|v| calc_seat(v))
        .map(|v| v.id())
        .collect();

    ids.sort();

    let mut ids = ids.iter().peekable();

    while ids.len() > 1 {
        let a = ids.next().unwrap();
        let b = ids.peek().unwrap();
        if *a + 1 != **b {
            return a+1;
        }
    }
    panic!("LIES");
}