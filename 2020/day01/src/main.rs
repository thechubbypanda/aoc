use std::time::Instant;

fn main() {
	let input: Vec<u32> = std::fs::read_to_string("input.txt").unwrap().lines().map(|l| l.parse().unwrap()).collect();

	let timer = Instant::now();
	println!("Part 1 output: {}", part1(&input).unwrap());
	println!("Part 1 time: {:?}", timer.elapsed());

	let timer = Instant::now();
	println!("Part 2 output: {}", part2(&input).unwrap());
	println!("Part 2 time: {:?}", timer.elapsed());
}

fn part1(input: &Vec<u32>) -> Result<u32, &'static str> {
	for i in input.iter() {
		for j in input.iter() {
			let test = i + j;
			if test == 2020 {
				return Ok(i * j);
			}
		}
	}
	return Err("No pair");
}

fn part2(input: &Vec<u32>) -> Result<u32, &'static str> {
	for i in input.iter() {
		for j in input.iter() {
			for k in input.iter() {
				if i + j + k == 2020 {
					return Ok(i * j * k);
				}
			}
		}
	}
	return Err("No pair");
}