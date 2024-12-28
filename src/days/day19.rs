fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (available_part, desired_part) = input.split_once("\n\n").unwrap();

    let available = available_part.split(", ").collect();
    let desired = desired_part.lines().collect();
    (available, desired)
}

pub fn part1(input: &str) -> u64 {
    let (available, desired) = parse_input(input);

    desired
        .iter()
        .filter(|&s| count_ways(s, &available) > 0)
        .count() as u64
}

pub fn part2(input: &str) -> u64 {
    let (available, desired) = parse_input(input);

    desired.iter().map(|&s| count_ways(s, &available)).sum()
}

fn count_ways(s: &str, available: &[&str]) -> u64 {
    let mut results = vec![0; s.len() + 1];
    results[s.len()] = 1;

    for i in (0..s.len()).rev() {
        let substring = &s[i..];
        for &prefix in available {
            if substring.starts_with(prefix) {
                results[i] += results[i + prefix.len()];
            }
        }
    }

    results[0]
}
