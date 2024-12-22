pub fn solution(input: &str) -> u64 {
    let mut a = Vec::new();
    let mut b = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        a.push(parts.next().unwrap().parse::<u64>().unwrap());
        b.push(parts.next().unwrap().parse::<u64>().unwrap());
    }

    a.sort();
    b.sort();

    a.into_iter().zip(b).map(|(x, y)| u64::abs_diff(x, y)).sum()
}
