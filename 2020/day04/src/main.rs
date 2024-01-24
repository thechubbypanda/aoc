use std::time::Instant;

const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap();
	let passports: Vec<&str> = input.split("\n\n").collect();

	let timer = Instant::now();
	println!("Part 1 output: {}", part1(&passports));
	println!("Part 1 time: {:?}", timer.elapsed());
	let timer = Instant::now();
	println!("Part 2 output: {}", part2(&passports));
	println!("Part 2 time: {:?}", timer.elapsed());
}

fn part1(input: &Vec<&str>) -> usize {
	input.iter()
		.map(|passport| FIELDS.iter()
			.map(|field| passport.contains(field))
			.fold(true, |a, v| a && v)
		)
		.filter(|valid| *valid)
		.count()
}

fn part2(input: &Vec<&str>) -> usize {
	input.iter()
		.filter(|passport| FIELDS.iter()
			.map(|field| passport.contains(field))
			.fold(true, |a, v| a && v)
		)
		.filter(|passport| validate(passport))
		.count()
}

fn validate(passport: &str) -> bool {
	let pairs: Vec<(&str, &str)> = passport.split_whitespace().map(|pair| {
		let mut split = pair.split(':');
		return (split.nth(0).unwrap(), split.nth(0).unwrap());
	}).collect();

	pairs.iter()
		.map(|(key, val)| -> bool {
			match *key {
				"byr" => in_range(val.parse().unwrap(), 1920, 2002),
				"iyr" => in_range(val.parse().unwrap(), 2010, 2020),
				"eyr" => in_range(val.parse().unwrap(), 2020, 2030),
				"hgt" => match val.strip_suffix("cm") {
					Some(val) => in_range(val.parse().unwrap(), 150, 193),
					None => match val.strip_suffix("in") {
						Some(val) => in_range(val.parse().unwrap(), 59, 76),
						None => false,
					},
				}
				"hcl" => match val.strip_prefix("#") {
					Some(col) => col.chars()
						.map(|c| (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'))
						.fold(true, |a, v| a && v),
					None => false,
				},
				"ecl" => EYE_COLORS.contains(&val),
				"pid" => val.len() == 9 && val.parse::<u32>().is_ok(),
				"cid" => true,
				_ => panic!("Wtf is {}?", key),
			}
		})
		.fold(true, |a, v| a && v)
}

fn in_range(num: u32, min: u32, max: u32) -> bool {
	num >= min && num <= max
}
