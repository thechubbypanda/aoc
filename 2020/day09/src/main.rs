use std::time::Instant;
use itertools::Itertools;

const PREAMBLE: usize = 25;

fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap();
	let sequence: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();

	let timer = Instant::now();
	println!("Part 1 output: {}", part1(sequence.clone()));
	println!("Part 1 time: {:?}", timer.elapsed());
	let timer = Instant::now();
	println!("Part 2 output: {}", part2(sequence.clone()));
	println!("Part 2 time: {:?}", timer.elapsed());
}

fn part1(input: Vec<u64>) -> u64 {
	input.as_slice()
		.windows(PREAMBLE + 1)
		.find(|win| win[0..PREAMBLE].iter()
			.tuple_combinations()
			.all(|(a,b)| a + b != win[PREAMBLE])
		)
		.map(|win| win[PREAMBLE])
		.unwrap()
}

fn part2(input: Vec<u64>) -> u64 {
	let num = part1(input.clone());
	let set: Vec<u64> = input.iter().take_while(|v| **v != num).map(|u| *u).collect();
	let size = set.clone().len();
	for start in 0..size {
		for end in (start + 1)..size {
			if input[start..=end].iter().sum::<u64>() == num {
				let largest = input[start..=end].iter()
					.fold(u64::min_value(), |a, v| if *v > a { *v } else { a });
				let smallest = input[start..=end].iter()
					.fold(u64::max_value(), |a, v| if *v < a { *v } else { a });
				return smallest + largest;
			}
		}
	}
	0
}
