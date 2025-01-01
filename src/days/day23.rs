use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    iter, str,
};

/// Represents a node using it's ASCII-encoded name
type Node = [u8; 2];
/// Represents a graph as an adjacency list
type Graph = BTreeMap<Node, BTreeSet<Node>>;

fn parse_input(input: &str) -> Graph {
    let mut graph = Graph::new();
    for line in input.lines() {
        let (a_part, b_part) = line.split_once('-').unwrap();
        let a = parse_node(a_part);
        let b = parse_node(b_part);
        graph.entry(a).or_insert_with(BTreeSet::new).insert(b);
        graph.entry(b).or_insert_with(BTreeSet::new).insert(a);
    }
    graph
}

fn parse_node(input: &str) -> Node {
    input.as_bytes().try_into().unwrap()
}

fn stringify_set<'a>(nodes: impl IntoIterator<Item = &'a Node>) -> String {
    nodes
        .into_iter()
        .map(stringify_node)
        .collect::<Vec<_>>()
        .join(",")
}

fn stringify_node(node: &Node) -> &str {
    str::from_utf8(node).unwrap()
}

pub fn part1(input: &str) -> u64 {
    let graph = parse_input(input);

    let cliques = three_cliques(&graph);

    cliques
        .iter()
        .filter(|&c| c.iter().any(|n| n[0] == b't'))
        .count() as u64
}

pub fn part2(input: &str) -> String {
    let graph = parse_input(input);

    let largest_clique = all_cliques(&graph).max_by_key(|c| c.len()).unwrap();
    stringify_set(&largest_clique)
}

fn three_cliques(graph: &Graph) -> BTreeSet<[Node; 3]> {
    let mut cliques = BTreeSet::new();
    for (&a, a_neighbors) in graph {
        for &b in a_neighbors {
            let b_neighbors = graph.get(&b).unwrap();
            for &c in b_neighbors {
                if a_neighbors.contains(&c) {
                    let mut clique = [a, b, c];
                    clique.sort();
                    cliques.insert(clique);
                }
            }
        }
    }
    cliques
}

fn all_cliques(graph: &Graph) -> impl Iterator<Item = BTreeSet<Node>> + '_ {
    let mut ungrouped: BTreeSet<Node> = graph.keys().copied().collect();
    iter::from_fn(move || {
        let Some(seed) = ungrouped.pop_first() else {
            return None;
        };

        let mut inside = BTreeSet::new();
        let mut checked = BTreeSet::new();
        let mut candidates = VecDeque::new();
        candidates.push_back(seed);
        while let Some(candidate) = candidates.pop_front() {
            if checked.contains(&candidate) {
                continue;
            }

            let neighbors = &graph[&candidate];
            if neighbors.is_superset(&inside) {
                inside.insert(candidate);
                candidates.extend(neighbors);
            }
            checked.insert(candidate);
        }

        Some(inside)
    })
}
