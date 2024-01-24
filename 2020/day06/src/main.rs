use std::time::Instant;
use std::collections::HashSet;

fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap();
	let input = input.split("\n\n").collect();

	let timer = Instant::now();
	println!("Part 1 output: {}", part1(&input));
	println!("Part 1 time: {:?}", timer.elapsed());
	let timer = Instant::now();
	println!("Part 2 output: {}", part2(&input));
	println!("Part 2 time: {:?}", timer.elapsed());
}

fn part1(input: &Vec<&str>) -> usize {
	input.iter()
		.map(|group| group.chars()
			.filter(|c| !c.is_whitespace())
			.collect::<HashSet<char>>()
			.len()
		)
		.sum()
}

fn part2(input: &Vec<&str>) -> usize {
	input.iter().map(|group| {
		group.chars()
			.filter(|c| !c.is_whitespace())
			.collect::<HashSet<char>>().iter()
			.map(|c| {
				group.lines()
					.map(|l| l.contains(*c))
					.fold(true, |a, v| a && v)
			})
			.filter(|v| *v)
			.count()
	}).sum()
}
