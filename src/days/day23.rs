use std::collections::{BTreeMap, BTreeSet};

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

pub fn part1(input: &str) -> u64 {
    let graph = parse_input(input);

    let cliques = three_cliques(&graph);

    cliques
        .iter()
        .filter(|&c| c.iter().any(|n| n[0] == b't'))
        .count() as u64
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
