use std::collections::{HashMap, HashSet};
use std::fs;

fn extract_nodes<'a>(lines: &Vec<&'a str>) -> HashSet<&'a str> {
    let mut nodes: HashSet<&str> = HashSet::new();
    for i in lines.iter() {
        nodes.extend(i.split('-'));
    }
    nodes
}

fn build_graph<'a>(
    nodes: &HashSet<&'a str>,
    lines: &Vec<&str>,
) -> (Vec<Vec<bool>>, HashMap<&'a str, usize>) {
    let mut graph: Vec<Vec<bool>> = vec![vec![false; nodes.len()]; nodes.len()];
    let mut node_names: HashMap<&str, usize> = HashMap::new();
    for (i, item) in nodes.iter().enumerate() {
        node_names.insert(item, i as usize);
    }
    for i in lines.iter() {
        let caves: Vec<&str> = i.split('-').collect();
        let index_1 = node_names.get(caves[0]).unwrap();
        let index_2 = node_names.get(caves[1]).unwrap();
        graph[*index_1][*index_2] = true;
        graph[*index_2][*index_1] = true;
    }
    (graph, node_names)
}

fn bfs(
    graph: &Vec<Vec<bool>>,
    visited: &mut Vec<bool>,
    current_node: usize,
    end: usize,
    big_caves: &HashSet<usize>,
) -> i32 {
    if current_node == end {
        return 1;
    }
    if !big_caves.contains(&current_node) {
        if visited[current_node] {
            return 0;
        } else {
            visited[current_node] = true;
        }
    }
    let next_nodes: Vec<usize> = (0..graph.len())
        .filter(|x| graph[current_node][*x])
        .collect();
    let mut sum: i32 = 0;
    for i in next_nodes.iter() {
        sum += bfs(graph, &mut visited.clone(), *i, end, big_caves);
    }
    sum
}

fn part_1(graph: &Vec<Vec<bool>>, node_names: &HashMap<&str, usize>) -> i32 {
    let start: usize = *node_names.get("start").unwrap();
    let end: usize = *node_names.get("end").unwrap();
    let big_caves: HashSet<usize> = node_names
        .iter()
        .filter(|(s, _)| s.chars().next().unwrap().is_uppercase())
        .map(|(_, i)| *i)
        .collect();
    let mut visited = vec![false; graph.len()];
    let sum = bfs(graph, &mut visited, start, end, &big_caves);
    return sum;
}

fn bfs2(
    graph: &Vec<Vec<bool>>,
    visited: &mut Vec<bool>,
    current_node: usize,
    start: usize,
    end: usize,
    big_caves: &HashSet<usize>,
    twice: bool,
) -> i32 {
    if current_node == end {
        return 1;
    }
    if current_node == start && visited[start] {
        return 0;
    }
    let mut flag = twice;
    if !big_caves.contains(&current_node) {
        if visited[current_node] {
            if flag {
                return 0;
            } else {
                flag = true;
            }
        } else {
            visited[current_node] = true;
        }
    }
    let next_nodes: Vec<usize> = (0..graph.len())
        .filter(|x| graph[current_node][*x])
        .collect();
    let mut sum: i32 = 0;
    for i in next_nodes.iter() {
        sum += bfs2(graph, &mut visited.clone(), *i, start, end, big_caves, flag);
    }
    sum
}

fn part_2(graph: &Vec<Vec<bool>>, node_names: &HashMap<&str, usize>) -> i32 {
    let start: usize = *node_names.get("start").unwrap();
    let end: usize = *node_names.get("end").unwrap();
    let big_caves: HashSet<usize> = node_names
        .iter()
        .filter(|(s, _)| s.chars().next().unwrap().is_uppercase())
        .map(|(_, i)| *i)
        .collect();
    let mut visited = vec![false; graph.len()];
    let sum = bfs2(graph, &mut visited, start, start, end, &big_caves, false);
    return sum;
}

fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let content_lines: Vec<&str> = contents.split('\n').collect();

    let nodes = extract_nodes(&content_lines);
    let (graph, node_names) = build_graph(&nodes, &content_lines);
    println!("{}", part_1(&graph, &node_names));
    println!("{}", part_2(&graph, &node_names));
}
