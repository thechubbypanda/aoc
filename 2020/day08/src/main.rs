use std::str::Lines;
use crate::Instruction::{JMP, ACC, NOP};
use std::time::Instant;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Instruction {
	ACC(i32),
	JMP(i32),
	NOP(i32),
}

fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap();
	let program = parse_program(input.lines());

	let timer = Instant::now();
	println!("Part 1 output: {}", part1(program.clone()).err().unwrap());
	println!("Part 1 time: {:?}", timer.elapsed());
	let timer = Instant::now();
	println!("Part 2 output: {}", part2(program.clone()));
	println!("Part 2 time: {:?}", timer.elapsed());
}

fn part1(program: Vec<Instruction>) -> Result<i32, i32> {
	let mut seen: HashSet<i32> = HashSet::new();

	let mut current = 0;
	let mut accumulator = 0;

	while seen.get(&current).is_none() {
		seen.insert(current);

		match program[current as usize] {
			ACC(num) => {
				accumulator += num;
				current += 1;
			}
			JMP(num) => {
				current += num;
			}
			NOP(_) => {
				current += 1;
			}
		}

		if current == program.len() as i32 {
			return Ok(accumulator);
		} else if current > program.len() as i32 {
			return Err(accumulator);
		}
	}
	Err(accumulator)
}

fn part2(program: Vec<Instruction>) -> i32 {
	for i in 0..program.len() {
		let mut altered: Vec<Instruction> = program.clone();
		match altered[i] {
			ACC(_) => (),
			NOP(num) => altered[i] = JMP(num),
			JMP(num) => altered[i] = NOP(num),
		}
		match part1(altered) {
			Ok(num) => return num,
			Err(_) => ()
		}
	}
	panic!("Couldn't find one that works :/")
}

fn parse_program(input: Lines) -> Vec<Instruction> {
	input
		.map(|l| l.split_whitespace().collect::<Vec<&str>>())
		.map(|split| (split[0], split[1].parse().unwrap()))
		.map(|(instruction, num)| match instruction {
			"acc" => ACC(num),
			"jmp" => JMP(num),
			"nop" => NOP(num),
			_ => panic!("wtf is this"),
		})
		.collect()
}
