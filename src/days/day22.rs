use std::{collections::BTreeMap, iter};

const MODULUS: u64 = 16_777_216;

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|n| n.parse().unwrap()).collect()
}

pub fn part1(input: &str) -> u64 {
    let seeds = parse_input(input);

    seeds.iter().map(|&seed| rng(seed).nth(2000).unwrap()).sum()
}

pub fn part2(input: &str) -> u64 {
    let seeds = parse_input(input);

    let mut total_prices = BTreeMap::new();
    for &seed in &seeds {
        let mut rng = rng_digits(seed).take(2001);

        let x = rng.next().unwrap();
        let y = rng.next().unwrap();
        let z = rng.next().unwrap();
        let w = rng.next().unwrap();
        let mut deltas = [0, y - x, z - y, w - z];

        let mut prices = BTreeMap::new();
        let mut last_price = w;
        for price in rng {
            let [_, x, y, z] = deltas;
            deltas = [x, y, z, price - last_price];

            prices.entry(deltas).or_insert(price);

            last_price = price;
        }

        for (deltas, price) in prices {
            *total_prices.entry(deltas).or_insert(0) += price as u64;
        }
    }

    total_prices.into_iter().map(|(_, p)| p).max().unwrap()
}

fn rng_digits(seed: u64) -> impl Iterator<Item = i8> {
    rng(seed).map(|n| (n % 10) as i8)
}

fn rng(seed: u64) -> impl Iterator<Item = u64> {
    let mut state = seed;
    iter::from_fn(move || {
        let n = state;
        state = evolve(state);
        Some(n)
    })
}

fn evolve(n: u64) -> u64 {
    let n = (n ^ n * 64) % MODULUS;
    let n = (n ^ n / 32) % MODULUS;
    let n = (n ^ n * 2048) % MODULUS;
    n
}
