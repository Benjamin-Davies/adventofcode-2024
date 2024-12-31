const MODULUS: u64 = 16_777_216;

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|n| n.parse().unwrap()).collect()
}

pub fn part1(input: &str) -> u64 {
    let initial_secrets = parse_input(input);

    initial_secrets
        .iter()
        .map(|&n| evolve_repeat(n, 2000))
        .sum()
}

fn evolve_repeat(mut n: u64, count: u64) -> u64 {
    for _ in 0..count {
        n = evolve(n);
    }
    n
}

fn evolve(n: u64) -> u64 {
    let n = (n ^ n * 64) % MODULUS;
    let n = (n ^ n / 32) % MODULUS;
    let n = (n ^ n * 2048) % MODULUS;
    n
}
