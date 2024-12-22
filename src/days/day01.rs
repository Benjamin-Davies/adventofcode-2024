use std::collections::BTreeMap;

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut a = Vec::new();
    let mut b = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        a.push(parts.next().unwrap().parse::<u64>().unwrap());
        b.push(parts.next().unwrap().parse::<u64>().unwrap());
    }

    (a, b)
}

pub fn part1(input: &str) -> u64 {
    let (mut a, mut b) = parse_input(input);

    a.sort();
    b.sort();

    a.into_iter().zip(b).map(|(x, y)| u64::abs_diff(x, y)).sum()
}

pub fn part2(input: &str) -> u64 {
    let (a, b) = parse_input(input);

    let mut counts = BTreeMap::new();
    for x in b {
        *counts.entry(x).or_insert(0) += 1;
    }

    a.into_iter()
        .map(|x| x * counts.get(&x).copied().unwrap_or(0))
        .sum()
}
