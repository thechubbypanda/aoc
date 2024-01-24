type Item = usize;

type Atom = Option<Item>;

#[derive(Debug, Clone)]
enum Operation {
    Add(Atom, Atom),
    Mul(Atom, Atom),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    divisor: Item,
    true_monkey: usize,
    false_monkey: usize,
    inspections: usize,
}

impl Monkey {
    fn parse(value: &str) -> Option<Self> {
        let mut lines = value.lines().skip(1);
        let items = lines
            .next()?
            .strip_prefix("  Starting items: ")?
            .split(", ")
            .map(|item| item.parse().ok())
            .collect::<Option<Vec<Item>>>()?;
        let mut operands = lines
            .next()?
            .strip_prefix("  Operation: new = ")?
            .split_whitespace();
        let num1 = operands.next()?.parse().ok();
        let op = operands.next().unwrap();
        let num2 = operands.next()?.parse().ok();
        let operation = match op {
            "+" => Operation::Add(num1, num2),
            "*" => Operation::Mul(num1, num2),
            _ => panic!("Unknown operator: {op}"),
        };
        let divisor = lines
            .next()?
            .strip_prefix("  Test: divisible by ")?
            .parse()
            .ok()?;
        let true_monkey = lines.next()?.split_whitespace().last()?.parse().ok()?;
        let false_monkey = lines.next()?.split_whitespace().last()?.parse().ok()?;
        Some(Monkey {
            items,
            operation,
            divisor,
            true_monkey,
            false_monkey,
            inspections: 0,
        })
    }

    fn inspect(&self, item: Item) -> Item {
        match self.operation {
            Operation::Add(a, b) => a.unwrap_or(item) + b.unwrap_or(item),
            Operation::Mul(a, b) => a.unwrap_or(item) * b.unwrap_or(item),
        }
    }

    fn throw_to_monkey(&self, item: Item) -> usize {
        if item % self.divisor == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }
}

fn round(monkeys: &mut Vec<Monkey>, calm: impl Fn(Item) -> Item) {
    for i in 0..monkeys.len() {
        let items: Vec<Item> = monkeys[i].items.drain(0..).collect();
        monkeys[i].inspections += items.len();
        for item in items.into_iter() {
            let item = calm(monkeys[i].inspect(item));
            let new_monkey = monkeys[i].throw_to_monkey(item);
            monkeys[new_monkey].items.push(item);
        }
    }
}

pub fn part1(input: String) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(Monkey::parse)
        .collect::<Option<Vec<Monkey>>>()
        .unwrap();

    for _round in 0..20 {
        round(&mut monkeys, |item| item / 3);
    }

    let mut inspections: Vec<usize> = monkeys.into_iter().map(|m| m.inspections).collect();
    inspections.sort();
    inspections.into_iter().rev().take(2).product()
}

pub fn part2(input: String) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(Monkey::parse)
        .collect::<Option<Vec<Monkey>>>()
        .unwrap();
    let lcm: Item = monkeys.iter().map(|m| m.divisor).product();

    for _round in 0..10_000 {
        round(&mut monkeys, |item| item % lcm);
    }

    let mut inspections: Vec<usize> = monkeys.into_iter().map(|m| m.inspections).collect();
    inspections.sort();
    inspections.into_iter().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::*;

    #[test]
    fn test_part1() {
        run_test!(part1);
    }

    #[test]
    fn run_part1() {
        run_real!(part1);
    }

    #[test]
    fn test_part2() {
        run_test!(part2);
    }

    #[test]
    fn run_part2() {
        run_real!(part2);
    }
}
