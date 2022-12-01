use aoc_lib::util::to_lines;

#[derive(Clone)]
enum Node {
    Value(usize),
    Pair(Box<(Node, Node)>),
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::Value(v) => v.to_string(),
            Self::Pair(p) => format!("[{:?},{:?}]", p.0, p.1),
        })
    }
}

impl Node {
    fn from_chars<T>(l: &mut T) -> Self
    where
        T: Iterator<Item = char>,
    {
        let mut stack = Vec::new();
        while let Some(c) = l.next() {
            if c.is_numeric() {
                stack.push(Self::Value(c.to_digit(10).unwrap() as usize))
            } else if c == ']' {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(Self::pair(left, right));
            }
        }
        stack.pop().unwrap()
    }

    fn pair(a: Node, b: Node) -> Self {
        Self::Pair(Box::new((a, b)))
    }

    fn reduce(mut self) -> Self {
        while self.explode(0).is_some() || self.split() {}
        self
    }

    /// Inspired by [chaosteil](https://github.com/chaosteil/aoc2021/blob/main/aoc18/src/main.rs)
    fn explode(&mut self, depth: usize) -> Option<(Option<usize>, Option<usize>)> {
        match self {
            Self::Value(_) => None,
            Self::Pair(p) => {
                if let Self::Value(l) = p.0 {
                    if let Self::Value(r) = p.1 {
                        if depth >= 4 {
                            *self = Self::Value(0);
                            return Some((Some(l), Some(r)));
                        }
                    }
                }
                if let Some((l, mut r)) = p.0.explode(depth + 1) {
                    if r.is_some() {
                        p.1.insert_left(r.unwrap());
                        r = None;
                    }
                    return Some((l, r));
                }
                if let Some((mut l, r)) = p.1.explode(depth + 1) {
                    if l.is_some() {
                        p.0.insert_right(l.unwrap());
                        l = None;
                    }
                    return Some((l, r));
                }
                None
            }
        }
    }

    fn insert_left(&mut self, l: usize) {
        match self {
            Self::Value(ref mut v) => *v += l,
            Self::Pair(ref mut p) => p.0.insert_left(l),
        }
    }

    fn insert_right(&mut self, r: usize) {
        match self {
            Self::Value(ref mut v) => *v += r,
            Self::Pair(ref mut p) => p.1.insert_right(r),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Value(v) if *v > 9 => {
                *self = Self::pair(Self::Value(*v / 2), Self::Value(*v / 2 + *v % 2));
                true
            }
            Self::Pair(p) => p.0.split() || p.1.split(),
            _ => false,
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Self::Value(v) => *v,
            Self::Pair(p) => p.0.magnitude() * 3 + p.1.magnitude() * 2,
        }
    }
}

pub fn part1(input: String) -> usize {
    to_lines(&input)
        .iter()
        .map(|l| Node::from_chars(&mut l.chars()))
        .map(|num| num.reduce())
        .reduce(|acc, num| Node::pair(acc, num).reduce())
        .unwrap()
        .magnitude()
}

pub fn part2(input: String) -> usize {
    let nums = to_lines(&input)
        .iter()
        .map(|l| Node::from_chars(&mut l.chars()))
        .collect::<Vec<Node>>();
    nums.clone().into_iter()
        .map(|l| nums.clone().into_iter().map(move |r| Node::pair(l.clone(), r)))
        .flatten()
        .map(|sum| sum.reduce().magnitude())
        .max()
        .unwrap()
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
}
