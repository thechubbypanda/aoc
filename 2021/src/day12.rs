use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use aoc_lib::util::to_lines;

#[derive(Debug)]
struct Node {
    pub big: bool,
    pub conns: Vec<String>,
}

type Graph = HashMap<String, Node>;
type Path = Vec<String>;

fn get_graph(input: String) -> Graph {
    let rules: Vec<(String, String)> = to_lines(&input)
        .iter()
        .map(|l| l.split('-'))
        .map(|mut s| (s.next().unwrap().to_string(), s.next().unwrap().to_string()))
        .collect();
    let mut nodes = HashSet::new();
    for (n1, n2) in rules.iter() {
        nodes.insert(n1.clone());
        nodes.insert(n2.clone());
    }
    let mut nodes: Graph = nodes
        .into_iter()
        .map(|name| {
            (
                name.clone(),
                Node {
                    big: name.chars().next().unwrap() < 'a',
                    conns: Vec::new(),
                },
            )
        })
        .collect();
    for (from, to) in rules {
        nodes.get_mut(&from).unwrap().conns.push(to.clone());
        nodes.get_mut(&to).unwrap().conns.push(from);
    }
    for n in nodes.iter() {
        println!("{:?}", n);
    }
    println!();
    nodes
}

pub fn part1(input: String) -> usize {
    let graph = get_graph(input);
    let mut paths = Vec::new();
    traverse1(
        &graph,
        &"start".to_string(),
        &mut HashSet::new(),
        &mut paths,
        &mut Vec::new(),
    );
    // for path in paths.iter() {
    //     println!("{:?}", path);
    // }
    paths.len()
}

fn traverse1(
    graph: &Graph,
    node: &String,
    visited: &mut HashSet<String>,
    paths: &mut Vec<Path>,
    path: &mut Path,
) {
    let n = graph.get(node).unwrap();
    if !n.big {
        if visited.contains(node) {
            return;
        }
        visited.insert(node.clone());
    }
    path.push(node.clone());
    if node == "end" {
        paths.push(path.clone());
        visited.remove(node);
        path.pop();
        return;
    }
    for s in n.conns.iter() {
        traverse1(graph, s, visited, paths, path);
    }
    path.pop();
    visited.remove(node);
}

pub fn part2(input: String) -> usize {
    let graph = get_graph(input);
    let mut paths = Vec::new();
    traverse2(
        &graph,
        &"start".to_string(),
        &mut graph.keys().map(|k| (k.clone(), 0)).collect(),
        &mut paths,
        &mut Vec::new(),
    );
    // for path in paths.iter() {
    //     println!("{:?}", path);
    // }
    paths.len()
}

fn traverse2(
    graph: &Graph,
    node: &String,
    visited: &mut HashMap<String, usize>,
    paths: &mut Vec<Path>,
    path: &mut Path,
) {
    let n = graph.get(node).unwrap();
    if !n.big {
        if visited[node] > 0 {
            if visited.values().any(|v| *v == 2) {
                return;
            }
            if node == "start" || node == "end" {
                return;
            }
        }
        *visited.get_mut(node).unwrap() += 1;
    }
    path.push(node.clone());
    if node == "end" {
        paths.push(path.clone());
        *visited.get_mut(node).unwrap() -= 1;
        path.pop();
        return;
    }
    for s in n.conns.iter() {
        traverse2(graph, s, visited, paths, path);
    }
    path.pop();
    if !n.big {
        *visited.get_mut(node).unwrap() -= 1;
    }
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
