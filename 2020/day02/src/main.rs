use std::time::Instant;

fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap();

	let start = Instant::now();
	println!("Part 1 output: {}", part1(&input));
	println!("Part 1 time: {:?}", start.elapsed());
	let start = Instant::now();
	println!("Part 2 output: {}", part2(&input));
	println!("Part 2 time: {:?}", start.elapsed());
}

fn part1(input: &String) -> usize {
	return input
		.lines()
		.filter(|l| {
			let (min, max, character, pass) = split(l);

			let count = pass.chars().filter(|c| *c == character).count();

			return count >= min && count <= max;
		})
		.count();
}

fn part2(input: &String) -> usize {
	return input
		.lines()
		.filter(|l| {
			let (i0, i1, character, pass) = split(l);

			let characters: Vec<char> = pass.chars().collect();

			if characters[i0 - 1] == character {
				return characters[i1 - 1] != character;
			}
			return characters[i1 - 1] == character;
		})
		.count();
}

fn split(line: &str) -> (usize, usize, char, &str) {
	// Format of the given line: {min}-{max} {character}: {value}
	let parts: Vec<&str> = line
		.split_whitespace()
		.collect();

	let range: Vec<usize> = parts[0]
		.split('-')
		.map(|c| c.parse().unwrap())
		.collect();
	let min = range[0];
	let max = range[1];

	let character = parts[1]
		.chars()
		.nth(0)
		.unwrap();

	let value = parts[2];

	return (min, max, character, value);
}
