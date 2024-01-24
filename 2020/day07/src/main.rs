use std::time::Instant;
use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
	color: String,
	contents: Vec<(usize, String)>,
}

impl Rule {
	pub fn can_contain_r(&self, color: &str, set: &HashMap<String, Rule>) -> bool {
		self.contents.iter()
			.any(|(_, c)|
				c.as_str() == color ||
					set.get(c.as_str()).unwrap().can_contain_r(color, set)
			)
	}
}

fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap();
	let lines = input.lines().collect();

	let timer = Instant::now();
	println!("Part 1 output: {}", part1(&lines));
	println!("Part 1 time: {:?}", timer.elapsed());
	let timer = Instant::now();
	println!("Part 2 output: {}", part2(&lines));
	println!("Part 2 time: {:?}", timer.elapsed());
}

fn part1(input: &Vec<&str>) -> usize {
	let mut set = HashMap::new();:wq
	for r in input.iter().map(|l| str_to_rule(l)) {
		set.insert(r.color.clone(), r);
	}
	set.values()
		.filter(|r| r.can_contain_r("shiny gold", &set))
		.count()
}

fn part2(input: &Vec<&str>) -> usize {
	let rules = input.iter().map(|l| str_to_rule(l)).collect();
	count(&rules, rule_by_color(&rules, "shiny gold"))
}

fn count(rules: &Vec<Rule>, start: &Rule) -> usize {
	if start.contents.is_empty() {
		return 0;
	}
	start.contents.iter()
		.map(|(c, color)| {
			c * (1 + count(rules, rule_by_color(rules, color)))
		})
		.sum()
}

fn rule_by_color<'a>(rules: &'a Vec<Rule>, color: &str) -> &'a Rule {
	rules.iter().find(|r| r.color == color).unwrap()
}

fn str_to_rule(line: &str) -> Rule {
	let color_contents: Vec<&str> = line.split(" bags contain ").collect();
	let color: String = color_contents[0].to_string();

	if color_contents[1].contains("no other") {
		return Rule { color, contents: vec!() };
	}

	let contents: Vec<(usize, String)> = color_contents[1].split(", ")
		.map(|content| content.split_whitespace().collect())
		.map(|content: Vec<&str>| {
			let mut iter = content.iter();
			let count = iter.next().unwrap().parse().unwrap();
			let color = iter.next().unwrap().to_string() + " " + iter.next().unwrap();
			(count, color)
		})
		.collect();
	Rule { color, contents }
}
