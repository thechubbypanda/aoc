type Item = usize;

type Atom = Option<Item>;

#[derive(Debug, Clone)]
enum Operation {
    Add(Atom, Atom),
    Mul(Atom, Atom),
}

#[derive(Debug, Clone)]
struct Test {
    divisor: Item,
    true_monkey: usize,
    false_monkey: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test: Test,
    inspections: usize,
}

// pub fn product_of_factors(item: Item) -> Item {
//     (1..item + 1).into_iter().filter(|&x| item % x == 0).product()
// }

impl Monkey {
    fn parse(value: &str) -> Self {
        let mut lines = value.lines().skip(1);
        let items = lines
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();
        let mut operands = lines
            .next()
            .unwrap()
            .strip_prefix("  Operation: new = ")
            .unwrap()
            .split_whitespace();
        let num1 = operands.next().unwrap().parse().ok();
        let op = operands.next().unwrap();
        let num2 = operands.next().unwrap().parse().ok();
        let operation = match op {
            "+" => Operation::Add(num1, num2),
            "*" => Operation::Mul(num1, num2),
            _ => panic!("Unknown operator: {op}"),
        };
        let test = Test {
            divisor: lines
                .next()
                .unwrap()
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap(),
            true_monkey: lines
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),
            false_monkey: lines
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),
        };
        Monkey {
            items,
            operation,
            test,
            inspections: 0,
        }
    }

    fn inspect_1(&self, item: Item) -> Item {
        let item = match self.operation {
            Operation::Add(a, b) => a.unwrap_or(item) + b.unwrap_or(item),
            Operation::Mul(a, b) => a.unwrap_or(item) * b.unwrap_or(item),
        };
        item / 3
    }

    fn inspect_2(&self, item: Item) -> Item {
        // match self.operation {
        //     Operation::Add(a, b) => a.unwrap_or(item) + b.unwrap_or(item),
        //     Operation::Mul(a, b) => product_of_factors(a.unwrap_or(item) * b.unwrap_or(item)),
        // }
        item
    }

    fn throw_to_monkey(&self, item: Item) -> usize {
        if item % self.test.divisor == 0 {
            self.test.true_monkey
        } else {
            self.test.false_monkey
        }
    }
}

pub fn part1(input: String) -> usize {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::parse).collect();

    for round in 0..20 {
        println!("After round {}:", round + 1);
        for i in 0..monkeys.len() {
            let items: Vec<Item> = monkeys[i].items.drain(0..).collect();
            monkeys[i].inspections += items.len();
            for item in items.into_iter() {
                let item = monkeys[i].inspect_1(item);
                let new_monkey = monkeys[i].throw_to_monkey(item);
                monkeys[new_monkey].items.push(item);
            }
        }
        for (i, monkey) in monkeys.iter().enumerate() {
            println!("Monkey {i}: {:?}", monkey.items);
        }
    }

    let mut inspections: Vec<usize> = monkeys.into_iter().map(|m| m.inspections).collect();
    inspections.sort();
    inspections.into_iter().rev().take(2).product()
}

pub fn part2(input: String) -> usize {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::parse).collect();

    for _round in 0..10_000 {
        for i in 0..monkeys.len() {
            let items: Vec<Item> = monkeys[i].items.drain(0..).collect();
            monkeys[i].inspections += items.len();
            for item in items.into_iter() {
                let item = monkeys[i].inspect_2(item);
                let new_monkey = monkeys[i].throw_to_monkey(item);
                monkeys[new_monkey].items.push(item);
            }
        }
    }

    let mut inspections: Vec<usize> = monkeys.into_iter().map(|m| m.inspections).collect();
    inspections.sort();
    inspections.into_iter().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use aoc_lib::*;

    use super::*;

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
