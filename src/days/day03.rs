use regex::Regex;

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|caps| (caps[1].parse().unwrap(), caps[2].parse().unwrap()))
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let products = parse_input(input);

    products.into_iter().map(|(a, b)| a * b).sum()
}
