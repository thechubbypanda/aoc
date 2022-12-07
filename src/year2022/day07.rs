use aoc_lib::util::to_lines;

struct Directory {
    name: String,
    contents: Vec<Structure>,
    size: usize,
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            contents: vec![],
            size: 0,
        }
    }
}

struct File {
    size: usize,
}

enum Structure {
    File(File),
    Dir(Directory),
}

impl Structure {
    fn dir_mut(&mut self) -> Option<&mut Directory> {
        match self {
            Self::File(_) => None,
            Self::Dir(d) => Some(d),
        }
    }

    fn dir(&self) -> Option<&Directory> {
        match self {
            Self::File(_) => None,
            Self::Dir(d) => Some(d),
        }
    }

    fn size(&self) -> usize {
        match self {
            Self::File(f) => f.size,
            Self::Dir(d) => d.size,
        }
    }
}

fn populate(lines: &mut Vec<Vec<String>>, current_dir: &mut Directory) {
    while !lines.is_empty() {
        match lines
            .pop()
            .unwrap()
            .iter()
            .map(|part| part.as_str())
            .collect::<Vec<&str>>()[..]
        {
            ["$", "ls"] => {}
            ["$", "cd", ".."] => {
                break;
            }
            ["$", "cd", dir_name] => populate(
                lines,
                current_dir
                    .contents
                    .iter_mut()
                    .filter_map(|structure| structure.dir_mut())
                    .find(|structure| structure.name == dir_name)
                    .unwrap(),
            ),
            ["dir", dir_name] => {
                current_dir
                    .contents
                    .push(Structure::Dir(Directory::new(dir_name)));
            }
            [size, _] => {
                current_dir.contents.push(Structure::File(File {
                    size: size.parse().unwrap(),
                }));
            }
            [..] => {}
        }
    }
    current_dir.size = current_dir.contents.iter().map(|s| s.size()).sum()
}

fn parse_input(input: String) -> Directory {
    let mut root = Directory::new("/");
    populate(
        &mut to_lines(&input)
            .into_iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .skip(1)
            .rev()
            .collect::<Vec<Vec<String>>>(),
        &mut root,
    );
    root
}

fn collect_dirs(directory: &Directory) -> Vec<&Directory> {
    let mut dirs = vec![directory];
    dirs.extend(
        directory
            .contents
            .iter()
            .filter_map(|structure| structure.dir())
            .flat_map(|dir| collect_dirs(dir).into_iter()),
    );
    dirs
}

pub fn part1(input: String) -> usize {
    collect_dirs(&parse_input(input))
        .into_iter()
        .map(|dir| dir.size)
        .filter(|size| *size <= 100_000)
        .sum()
}

pub fn part2(input: String) -> usize {
    let root = parse_input(input);
    collect_dirs(&root)
        .into_iter()
        .map(|dir| dir.size)
        .filter(|size| 70000000 - (root.size - *size) > 30000000)
        .min()
        .unwrap()
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
